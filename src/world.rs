
use std::collections::{HashMap};
use serde::{Serialize, Deserialize};

use crate::{
	PlayerId,
	controls::{Control},
	pos::{Pos, Area},
	vec2::Vec2,
	util::Holder,
	sprite::Sprite,
	worldmessages::{WorldMessage, SectionMessage, ViewAreaMessage, ChangeMessage, SoundType::{BuildError}},
	tickstamp::{Tickstamp},
	timestamp::Timestamp,
	creature::{Creature, Mind, CreatureId, PlayerSave, CreatureView},
	player::Player,
	map::{Map, MapSave}
};

const EDGE_OFFSET: i32 = 32;
const VIEW_AREA_SIZE: Pos = Pos::new(128, 128);

pub struct World {
	pub name: String,
	pub time: Tickstamp,
	ground: Map,
	players: HashMap<PlayerId, Player>,
	creatures: Holder<CreatureId, Creature>,
	claims: HashMap<PlayerId, Pos>,
	seed: u32
}

impl World {
	
	pub fn new(name: String, seed: u32) -> Self {
		let time = Tickstamp(0);
		Self {
			name,
			ground: Map::new(seed, time),
			players: HashMap::new(),
			creatures: Holder::new(),
			time,
			claims: HashMap::new(),
			seed
		}
	}
	
	pub fn default_player(&mut self) -> PlayerSave {
		PlayerSave::new(Vec2::from_pos(&self.ground.player_spawn()))
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId, saved: PlayerSave) -> Result<(), PlayerError> {
		if self.players.contains_key(playerid){
			return Err(PlayerError::AlreadyExists(playerid.clone()));
		}
		let body = self.creatures.insert(Creature::load_player(playerid.clone(), saved));
		self.players.insert(
			playerid.clone(),
			Player::new(body)
		);
		Ok(())
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<(), PlayerError> {
		let player = self.players.remove(playerid).ok_or_else(|| PlayerError::NotFound(playerid.clone()))?;
		self.creatures.remove(&player.body);
		Ok(())
	}
	
	pub fn save_player(&self, playerid: &PlayerId) -> Result<PlayerSave, PlayerError> {
		let player = self.players.get(playerid).ok_or_else(|| PlayerError::NotFound(playerid.clone()))?;
		let body = self.creatures.get(&player.body).ok_or_else(|| PlayerError::BodyNotFound(playerid.clone()))?;
		Ok(body.save())
	}
	
	pub fn control_player(&mut self, playerid: &PlayerId, control: Control, timestamp: Timestamp) -> Result<(), PlayerError> {
		let player = self.players.get_mut(playerid).ok_or_else(|| PlayerError::NotFound(playerid.clone()))?;
		if let Control::Movement(delta) = control {
			player.movement = delta.try_normalize();
		}
		player.plan = Some(control);
		Ok(())
	}
	
	pub fn has_player(&mut self, playerid: &PlayerId) -> bool {
		self.players.contains_key(playerid)
	}
	
	pub fn list_players(&self) -> Vec<PlayerId> {
		self.players.keys().cloned().collect()
	}
	
	fn creature_plan(&self, creature: &Creature) -> Option<Control> {
		match &creature.mind {
			Mind::Player(playerid) => {
				if let Some(player) = self.players.get(playerid) {
					player.plan.clone()
				} else {Some(Control::Suicide)}
			}
		}
	}
	
	fn update_creatures(&mut self) -> Option<()> {
		let plans: HashMap<CreatureId, Control> = self.creatures.iter()
			.filter(|(_k, c)| c.cooldown.0 <= 0)
			.filter_map(|(k, c)|
				Some((*k, self.creature_plan(c)?))
			).collect();
		for (id, creature) in self.creatures.iter_mut() {
			creature.heard_sounds = Vec::new();
			if creature.cooldown.0 > 0 {
				creature.cooldown.0 -= 1;
				continue;
			}
			let movement: Option<Vec2> = match &creature.mind {
				Mind::Player(playerid) => self.players.get(playerid).and_then(|player| player.movement)
			};
			if let Some(delta) = movement {
				let newpos = creature.pos + delta * creature.speed();
				let blocking = creature.shape
					.moved(newpos)
					.outer_area()
					.iter()
					.any(|p| self.ground.cell(p).blocking());
				if !blocking {
					creature.pos = newpos;
				}
			}

			let Some(plan) = plans.get(id) 
				else {
					continue 
				};
			match plan {
				Control::Move(direction) => {
					creature.cooldown = creature.walk_cooldown;
					let newpos = creature.pos + Vec2::from_pos(&(Pos::new(0, 0) + *direction)) / 2.0;
					creature.pos = newpos;
					// let tile = self.ground.cell(newpos);
					// if !tile.blocking() {
						// creature.pos = newpos;
					// }
				}
				Control::Movement(_delta) => { }
				Control::Suicide => {
					creature.kill();
				}
				Control::Select(selector) => {
					creature.inventory.select(*selector);
				}
				Control::MoveSelected(selector) => {
					creature.inventory.move_selected(*selector);
				}
				Control::Interact(direction) => { }
				// 	let pos = creature.pos + direction.map(|dir| dir.to_position()).unwrap_or_else(Pos::zero);
				// 	let tile = self.ground.cell(pos);
				// 	let item = creature.inventory.selected();
				// 	let Some(interaction) = tile.interact(item, self.time)
				// 		else {
				// 			continue
				// 		};
				// 	if interaction.claim {
				// 		if let Some(player_id) = creature.player() {
				// 			if self.claims.contains_key(&player_id) {
				// 				creature.heard_sounds.push((BuildError, "Only one claim per player allowed".to_string()));
				// 				continue;
				// 			}
				// 			if self.claims.values().any(|p| p.distance_to(pos) < 64) {
				// 				creature.heard_sounds.push((BuildError, "Too close to existing claim".to_string()));
				// 				continue;
				// 			}
				// 			if pos.distance_to(self.ground.player_spawn()) < 96 {
				// 				creature.heard_sounds.push((BuildError, "Too close to spawn".to_string()));
				// 				continue;
				// 			}
				// 			self.claims.insert(player_id, pos);
				// 		} else {
				// 			creature.heard_sounds.push((
				// 				BuildError,
				// 				"Only players can claim land and you're not a player. If you read this something has probably gone wrong.".to_string()
				// 			));
				// 			continue;
				// 		}
				// 	}
				// 	if interaction.build {
				// 		if let Some(claim_pos) = creature.player().as_ref().and_then(|player_id| self.claims.get(player_id)) {
				// 			if pos.distance_to(*claim_pos) > 24 {
				// 				creature.heard_sounds.push((
				// 					BuildError,
				// 					"Too far from land claim to build".to_string()
				// 				));
				// 				continue;
				// 			}
				// 		} else {
				// 			creature.heard_sounds.push((
				// 				BuildError,
				// 				"Need land claim to build".to_string()
				// 			));
				// 			continue;
				// 		}
				// 	}
				// 	if !creature.inventory.pay(interaction.cost) {
				// 		continue;
				// 	}
				// 	for item in interaction.items {
				// 		creature.inventory.add(item);
				// 	}
				// 	if let Some(remains) = interaction.remains {
				// 		self.ground.set_structure(pos, remains);
				// 	}
				// 	if let Some(remains_ground) = interaction.remains_ground {
				// 		self.ground.set_ground(pos, remains_ground);
				// 	}
				// 	if let Some(message) = interaction.message {
				// 		creature.heard_sounds.push(message);
				// 	}
				// }
			}
		}
		for player in self.players.values_mut() {
			player.plan = None;
		}
		Some(())
	}
	
	fn loaded_areas(&self) -> Vec<Area> {
		self.players.values()
			.filter_map(Player::view_area)
			.collect()
	}
	
	pub fn update(&mut self) {
		self.update_creatures();
		
		self.ground.tick(self.time, self.loaded_areas());
		
		self.time.increment();
	}
	
	
	// fn draw_dynamic(&mut self) -> HashMap<Pos, Vec<Sprite>> {
	// 	let mut sprites: HashMap<Pos, Vec<Sprite>> = HashMap::new();
	// 	sprites.into_iter().map(|(pos, mut sprs)| {
	// 		sprs.append(&mut self.ground.cell(pos).sprites());
	// 		(pos, sprs)
	// 	}).collect()
	// }
	
	fn draw_changes(&mut self) -> Option<ChangeMessage> {
		Some(
			self.ground.modified().into_iter()
				.map(|(pos, tile)| (pos, tile.sprites()))
				.collect()
		)
	}
		// if let Some(last_drawing) = &self.drawing {
		// 	for pos in last_drawing.keys() {
		// 		sprites.entry(*pos).or_insert_with(||self.ground.cell(*pos).sprites());
		// 	}
		// 	for (pos, tile) in self.ground.modified().into_iter() {
		// 		sprites.entry(pos).or_insert_with(||tile.sprites());
		// 	}
		// 	let sprs: ChangeMessage = sprites.iter()
		// 		.filter(|(pos, spritelist)| last_drawing.get(pos) != Some(spritelist))
		// 		.map(|(pos, spritelist)| (*pos, spritelist.clone()))
		// 		.collect();
		// 	Some(sprs)
		// } else {None}
	// }
	
	pub fn view(&mut self) -> HashMap<PlayerId, WorldMessage> {
		let changes = self.draw_changes();
		let mut views: HashMap<PlayerId, WorldMessage> = HashMap::new();
		let dynamics: Vec<CreatureView> = self.players.values()
			.filter_map(|player| self.creatures.get(&player.body))
			.map(|creature| creature.view())
			.collect();
		for (playerid, player) in self.players.iter_mut() {
			let mut wm = WorldMessage::default();
			if let Some(body) = self.creatures.get(&player.body) {
				let in_view_range = player.view_area()
					.map_or(
						false,
						|area|
							(body.pos.x.floor() as i32) > area.min().x + EDGE_OFFSET &&
							(body.pos.x.ceil() as i32) < area.max().x - EDGE_OFFSET &&
							(body.pos.y.floor() as i32) > area.min().y + EDGE_OFFSET &&
							(body.pos.y.ceil() as i32) < area.max().y - EDGE_OFFSET
					 );
				if !in_view_range {
					let (total_area, redraw_area) = Self::new_view_area(body.pos.round(), &player.view_area);
					player.view_area = Some(total_area);
					wm.viewarea = Some(ViewAreaMessage{area: total_area});
					wm.section = Some(draw_field(redraw_area, &mut self.ground));
				}
				if changes.is_some() {
					wm.change = changes.clone();
				}
				wm.pos = Some(body.pos);
				wm.dynamics = Some(dynamics.clone());
				wm.inventory = Some(body.inventory.view());
				if !body.heard_sounds.is_empty() {
					wm.sounds = Some(body.heard_sounds.clone());
				}
			}
			views.insert(playerid.clone(), wm);
		}
		self.ground.flush();
		views
	}

	fn new_view_area(body_pos: Pos, view_area: &Option<Area>) -> (Area, Area) {
		let core_area = Area::centered(body_pos, VIEW_AREA_SIZE);
		let Some(old_area) = view_area else {
			return (core_area, core_area);
		};
		if !core_area.overlaps(old_area) {
			return (core_area, core_area);
		}
		if body_pos.x <= old_area.min().x + EDGE_OFFSET {
			let new_min = Pos::new(body_pos.x - VIEW_AREA_SIZE.x / 2, old_area.min().y);
			(Area::new(new_min, VIEW_AREA_SIZE), Area::between(new_min, Pos::new(old_area.min().x, old_area.max().y)))
		} else if body_pos.y <= old_area.min().y + EDGE_OFFSET {
			let new_min = Pos::new(old_area.min().x, body_pos.y - VIEW_AREA_SIZE.y / 2);
			(Area::new(new_min, VIEW_AREA_SIZE), Area::between(new_min, Pos::new(old_area.max().x, old_area.min().y)))
		} else if body_pos.x >= old_area.max().x - EDGE_OFFSET {
			let new_min = Pos::new(body_pos.x - VIEW_AREA_SIZE.x / 2, old_area.min().y);
			let new_area = Area::new(new_min, VIEW_AREA_SIZE);
			(new_area, Area::between(Pos::new(old_area.max().x, old_area.min().y), new_area.max()))
		} else if body_pos.y >= old_area.max().y - EDGE_OFFSET {
			let new_min = Pos::new(old_area.min().x, body_pos.y - VIEW_AREA_SIZE.y / 2);
			let new_area = Area::new(new_min, VIEW_AREA_SIZE);
			(new_area, Area::between(Pos::new(old_area.min().x, old_area.max().y), new_area.max()))
		} else {
			// this function shouldn't get called when this is the case, but let's do something somewhat sensible anyways
			(core_area, core_area)
		}
	}


	
	pub fn save(&self) -> WorldSave {
		WorldSave {
			name: self.name.clone(),
			time: self.time,
			ground: self.ground.save(),
			claims: self.claims.clone(),
			seed: self.seed,
		}
	}
	
	pub fn load(save: WorldSave) -> World {
		World {
			name: save.name,
			ground: Map::load(save.ground, save.time, save.seed),
			players: HashMap::new(),
			creatures: Holder::new(),
			time: save.time,
			claims: save.claims,
			seed: save.seed,
		}
	}
}


fn draw_field(area: Area, tiles: &mut Map) -> SectionMessage {
	// println!("redrawing field");
	let mut values :Vec<usize> = Vec::with_capacity((area.size().x * area.size().y) as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	for (pos, tile) in tiles.load_area(area) {
		let mut tile_sprites = Vec::new();
		tile_sprites.append(&mut tile.sprites());
		values.push(
			match mapping.iter().position(|x| x == &tile_sprites) {
				Some(index) => {
					index
				}
				None => {
					mapping.push(tile_sprites);
					mapping.len() - 1
				}
			}
		)
	}
	SectionMessage {
		area,
		field: values,
		mapping
	}
}

#[derive(Debug)]
pub enum PlayerError {
	NotFound(PlayerId),
	BodyNotFound(PlayerId),
	AlreadyExists(PlayerId)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSave {
	name: String,
	time: Tickstamp,
	ground: MapSave,
	claims: HashMap<PlayerId, Pos>,
	seed: u32
}

