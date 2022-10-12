// #![recursion_limit="512"]
use std::thread::sleep;
use std::time::{Instant, Duration};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use structopt::StructOpt;
use chrono::Utc;
use std::fs;

mod server;
mod gameserver;
mod util;
mod controls;
mod worldmessages;
mod pos;
mod player;
mod world;
mod sprite;
mod timestamp;
mod config;
mod errors;
mod creature;
mod tile;
mod item;
mod mapgen;
mod grid;
mod random;
mod basemap;

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
	mapgen::{MapType},
};



fn main(){
	
	let config = config::Config::from_args();
	
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
	
	let map = if let Some(map_path) = config.custom_map {
		let maptext = fs::read_to_string(&map_path).unwrap_or_else(|_| panic!("can't read map {:?}", map_path));
		let template = json5::from_str(&maptext).unwrap_or_else(|_| panic!("invalid map text:\n{:?}", maptext));
		MapType::Custom(template)
	} else {
		MapType::Builtin(config.map)
	};
	
	let mut world = World::new(map);
	
	let mut message_cache = MessageCache::default();
	
	// close handler
	// todo: don't let the closing wait on sleep (using a timer thread or recv_timeout)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	ctrlc::set_handler(move || {
		println!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	println!("dezl started on {}", Utc::now());
	
	let mut empty_timer = 1000000;
	
	while running.load(Ordering::SeqCst) {
		empty_timer += 1;
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					if let Err(err) = world.control_player(player.clone(), control){
						println!("error controlling player {:?}: {:?}", player, err);
					}
				}
				Action::Join(player) => {
					if let Err(err) = world.add_player(&player) {
						println!("Error: can not add player {:?}: {:?}", player, err);
						if let Err(senderr) = gameserver.send_player_error(&player, "worlderror", "invalid room or savefile") {
							println!("Error: can not send error message to {:?}: {:?}", player, senderr);
						}
					}
				}
				Action::Leave(player) => {
					if let Err(err) = world.remove_player(&player) {
						println!("Error: can not remove player {:?}: {:?}", player, err);
					}
					message_cache.remove(&player);
					empty_timer = 0;
				}
			}
		}
		if world.nplayers() == 0 && empty_timer > 100 {
			if empty_timer == 600 {
				world.reset();
			}
			sleep(Duration::from_millis(500));
			continue;
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
		let elapsed_time = now.elapsed();
		if elapsed_time > Duration::from_millis(3) {
			println!("Running update() took {} milliseconds.", elapsed_time.as_millis());
		}
		sleep(Duration::from_millis(config.step_duration));
	}
	println!("shutting down on {}", Utc::now());
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::mapgen::BuiltinMap;
	
	#[inline(never)]
	fn actually_redraw(world: &mut World, message_cache: &mut MessageCache){
	
		let now = Instant::now();
		world.update();
		let updated = now.elapsed();
		let messages = world.view();
		let viewed = now.elapsed();
		for (player, mut message) in messages {
			message_cache.trim(&player, &mut message);
			if message.is_empty(){
				continue;
			}
			println!("m {}", message.to_json());
		}
		let shown = now.elapsed();
		println!(
			"update: {}ms, view: {}ms, shown: {}ms",
			updated.as_millis(),
			(viewed - updated).as_millis(),
			(shown - viewed).as_millis()
		);
	}
	
	// #[test]
	fn redraw_field(){
		let map = MapType::Builtin(BuiltinMap::Square);
		
		let mut world = World::new(map);
	
		let mut message_cache = MessageCache::default();
		
		for i in 0..200{
			
			let player = PlayerId(format!("player{}", i));
			world.add_player(&player);
			actually_redraw(&mut world, &mut message_cache);
		}
	}
}


