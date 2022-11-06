// #![recursion_limit="512"]
use std::thread;
use std::time::{Instant, Duration};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use chrono::Utc;
use clap::Parser;

mod action;
mod basemap;
mod config;
mod controls;
mod creature;
mod errors;
mod gameserver;
mod grid;
mod inventory;
mod item;
mod map;
mod persistence;
mod player;
mod pos;
mod random;
mod randomtick;
mod server;
mod sprite;
mod tile;
mod timestamp;
mod util;
mod world;
mod worldmessages;

use self::{
	pos::{Pos, Direction},
	player::PlayerId,
	errors::{Result},
	sprite::Sprite,
	
	gameserver::GameServer,
	server::ServerEnum,
	controls::Action,
	world::World,
	worldmessages::MessageCache,
	persistence::{PersistentStorage, FileStorage, LoaderError},
	config::{Config, WorldAction},
};



fn main(){
	
	let config = Config::parse();
	
	eprintln!("Server admin(s): {}", config.admins);
	
	let adresses = config.address
		.unwrap_or_else(||
			(if cfg!(target_os = "linux") {
				vec!["abstract:dezl", "inet:127.0.0.1:9231"]
			} else {
				vec!["inet:127.0.0.1:9231"]
			})
			.iter()
			.map(|a| a.parse().unwrap())
			.collect()
		);
	eprintln!("adresses: {:?}", adresses);
	let servers: Vec<ServerEnum> = 
		adresses
		.iter()
		.map(|a| a.to_server().unwrap())
		.collect();
	
	let mut gameserver = GameServer::new(servers);
	
	let persistence = FileStorage::new(FileStorage::default_save_dir(config.name.clone()).unwrap());
	let mut world = match config.world_action {
		WorldAction::New => World::new(config.name.clone()),
		WorldAction::Load => World::load(persistence.load_world().expect("Can't load world")),
	};
	
	let mut message_cache = MessageCache::default();
	
	// close handler
	// todo: don't let the closing wait on sleep (using a timer thread or recv_timeout)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	ctrlc::set_handler(move || {
		eprintln!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	eprintln!("dezl started world {} on {}", config.name, Utc::now());
	
	while running.load(Ordering::SeqCst) {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					if let Err(err) = world.control_player(&player, control){
						eprintln!("error controlling player {:?}: {:?}", player, err);
					}
				}
				Action::Join(player) => {
					let playersave = match persistence.load_player(&player) {
						Ok(save) => save,
						Err(LoaderError::MissingResource(_)) => world.default_player(),
						Err(err) => {
							eprintln!("Error loading save for player {:?}: {:?}", player, err);
							if let Err(senderr) = gameserver.send_player_error(&player, "loaderror", "could not load saved player data") {
								eprintln!("Error: can not send error message to {:?}: {:?}", player, senderr);
							}
							continue
						}
					};
					if let Err(err) = world.add_player(&player, playersave) {
						eprintln!("Error: can not add player {:?}: {:?}", player, err);
						if let Err(senderr) = gameserver.send_player_error(&player, "worlderror", "invalid room or savefile") {
							eprintln!("Error: can not send error message to {:?}: {:?}", player, senderr);
						}
					}
				}
				Action::Leave(player) => {
					if world.has_player(&player) {
						persistence.save_player(&player, world.save_player(&player).unwrap()).unwrap();
						if let Err(err) = world.remove_player(&player) {
							eprintln!("Error: can not remove player {:?}: {:?}", player, err);
						}
					}
					message_cache.remove(&player);
				}
			}
		}
		let now = Instant::now();
		world.update();
		let messages = world.view();
		for (player, mut message) in messages {
			message_cache.trim(&player, &mut message);
			if message.is_empty(){
				continue;
			}
// 			eprintln!("m {}", message.to_json());
			if let Err(err) = gameserver.send(&player, message.to_json()) {
				eprintln!("Error: failed to send to {:?}: {:?}", player, err);
			}
		}
		if world.time.0 % 100 == 1 {
			save(&world, &persistence);
		}
		let elapsed_time = now.elapsed();
		if elapsed_time >= Duration::from_millis(5) {
			eprintln!("Running update() took {} milliseconds.", elapsed_time.as_millis());
		}
		thread::sleep(Duration::from_millis(config.step_duration));
	}
	save(&world, &persistence);
	eprintln!("shutting down on {}", Utc::now());
}

fn save(world: &World, persistence: &impl PersistentStorage) {
	persistence.save_world(world.save()).unwrap();
	for player in world.list_players() {
		persistence.save_player(&player, world.save_player(&player).unwrap()).unwrap();
	}
	eprintln!("saved world {} on step {}", world.name, world.time.0);
}



