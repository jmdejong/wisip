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
mod ground;
mod inventory;
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
	persistence::{PersistentStorage, FileStorage},
	config::{Config, WorldAction},
};



fn main(){
	
	let config = Config::parse();
	
	println!("Server admin(s): {}", config.admins);
	
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
	println!("adresses: {:?}", adresses);
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
		println!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	println!("dezl started world {} on {}", config.name, Utc::now());
	
	while running.load(Ordering::SeqCst) {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					if let Err(err) = world.control_player(&player, control){
						println!("error controlling player {:?}: {:?}", player, err);
					}
				}
				Action::Join(player) => {
					let playersave = persistence.load_player(&player).unwrap_or(world.default_player());
					if let Err(err) = world.add_player(&player, playersave) {
						println!("Error: can not add player {:?}: {:?}", player, err);
						if let Err(senderr) = gameserver.send_player_error(&player, "worlderror", "invalid room or savefile") {
							println!("Error: can not send error message to {:?}: {:?}", player, senderr);
						}
					}
				}
				Action::Leave(player) => {
					persistence.save_player(&player, world.save_player(&player).unwrap()).unwrap();
					if let Err(err) = world.remove_player(&player) {
						println!("Error: can not remove player {:?}: {:?}", player, err);
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
// 			println!("m {}", message.to_json());
			if let Err(err) = gameserver.send(&player, message.to_json()) {
				println!("Error: failed to send to {:?}: {:?}", player, err);
			}
		}
		if world.time.0 % 100 == 1 {
			save(&world, &persistence);
		}
		let elapsed_time = now.elapsed();
		if elapsed_time >= Duration::from_millis(1) {
			println!("Running update() took {} milliseconds.", elapsed_time.as_millis());
		}
		thread::sleep(Duration::from_millis(config.step_duration));
	}
	save(&world, &persistence);
	println!("shutting down on {}", Utc::now());
}

fn save(world: &World, persistence: &impl PersistentStorage) {
	persistence.save_world(world.save()).unwrap();
	for player in world.list_players() {
		persistence.save_player(&player, world.save_player(&player).unwrap()).unwrap();
	}
	println!("saved world {} on step {}", world.name, world.time.0);
}



