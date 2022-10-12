
use std::collections::{HashMap};

use crate::{
	PlayerId,
	controls::Control,
	Result,
	aerr,
	Pos,
	util::Holder,
	sprite::Sprite,
	worldmessages::{WorldMessage, FieldMessage, ChangeMessage},
	timestamp::{Timestamp},
	creature::{Creature, Mind, CreatureId},
	tile::Tile,
	player::Player,
	mapgen::{MapTemplate, MapType, create_map},
	ground::Ground,
	grid::Grid
};

pub struct World {
	time: Timestamp,
	ground: Ground,
	players: HashMap<PlayerId, Player>,
	creatures: Holder<CreatureId, Creature>,
	map: MapType,
	drawing: Option<HashMap<Pos, Vec<Sprite>>>,
}

impl World {
	
	pub fn new(map: MapType) -> Self {
		let time = Timestamp(0);
		Self {
			ground: Ground::new(time),
			players: HashMap::new(),
			creatures: Holder::new(),
			time,
			map,
			drawing: None,
		}
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId) -> Result<()> {
		if self.players.contains_key(playerid){
			return Err(aerr!("player {} already exists", playerid));
		}
		self.players.insert(
			playerid.clone(),
			Player{
				plan: None,
				body: None,
				is_new: true,
				view_center: None,
				inventory: Vec::new()
			}
		);
		Ok(())
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let player = self.players.remove(playerid).ok_or_else(|| aerr!("player {} not found", playerid))?;
		if let Some(body) = &player.body {
			self.creatures.remove(body);
		}
		Ok(())
	}
	
	pub fn control_player(&mut self, playerid: PlayerId, control: Control) -> Result<()>{
		let player = self.players.get_mut(&playerid).ok_or_else(|| aerr!("player not found"))?;
		player.plan = Some(control);
		Ok(())
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
		let mut creature_map: HashMap<Pos, CreatureId> = self.creatures.iter()
			.map(|(creatureid, creature)| (creature.pos, *creatureid))
			.collect();
		let plans: HashMap<CreatureId, Control> = self.creatures.iter()
			.filter(|(_k, c)| c.cooldown.0 <= 0)
			.filter_map(|(k, c)|
				Some((*k, self.creature_plan(c)?))
			).collect();
		for (id, creature) in self.creatures.iter_mut() {
			if creature.cooldown.0 > 0 {
				creature.cooldown.0 -= 1;
				continue;
			}
			match plans.get(id) {
				Some(Control::Move(direction)) => {
					creature.cooldown = creature.walk_cooldown;
					let newpos = creature.pos + *direction;
					let tile = self.ground.cell(newpos);
					if !tile.blocking() && !creature_map.contains_key(&newpos) {
						if creature_map.get(&creature.pos) == Some(id){
							creature_map.remove(&creature.pos);
						}
						creature_map.insert(newpos, *id);
						creature.pos = newpos;
					}
				}
				Some(Control::Suicide) => {
					creature.kill();
				}
				Some(Control::Use(_direction)) => {
					
				}
				Some(Control::Interact(direction)) => {
					let pos = creature.pos + direction.map(|dir| dir.to_position()).unwrap_or(Pos::zero());
					let tile = self.ground.cell(pos);
					self.ground.set(pos, tile.interact());
				}
				None => { }
			}
		}
		Some(())
	}
	
	
	fn spawn(&mut self){
		
		// spawn players
		for (playerid, player) in self.players.iter_mut() {
			if player.body.map_or(true, |id| !self.creatures.contains_key(&id)) {
				let body = self.creatures.insert(Creature::new_player(
					playerid.clone(),
					self.ground.player_spawn()
				));
				player.body = Some(body)
			}
			player.plan = None;
		}
	}
	
	pub fn update(&mut self) {
		self.ground.tick(self.time);
		self.update_creatures();
		
		self.spawn();
		
		self.time.increment();
	}
	
	
	fn draw_dynamic(&mut self) -> HashMap<Pos, Vec<Sprite>> {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = HashMap::new();
		for creature in self.creatures.values() {
			sprites.entry(creature.pos).or_insert_with(Vec::new).push(creature.sprite);
		}
		sprites.into_iter().map(|(pos, mut sprs)| {
			sprs.append(&mut self.ground.cell(pos).sprites());
			(pos, sprs)
		}).collect()
	}
	
	fn draw_changes(&mut self, mut sprites: HashMap<Pos, Vec<Sprite>>) -> Option<ChangeMessage> {
		if let Some(last_drawing) = &self.drawing {
			for pos in last_drawing.keys() {
				sprites.entry(*pos).or_insert(self.ground.cell(*pos).sprites());
			}
			for (pos, tile) in self.ground.modified().into_iter() {
				sprites.entry(pos).or_insert(tile.sprites());
			}
			let sprs: ChangeMessage = sprites.iter()
				.filter(|(pos, spritelist)| last_drawing.get(pos) != Some(spritelist))
				.map(|(pos, spritelist)| (*pos, spritelist.clone()))
				.collect();
			Some(sprs)
		} else {None}
	}
	
	pub fn view(&mut self) -> HashMap<PlayerId, WorldMessage> {
		let dynamic_sprites = self.draw_dynamic();
		let changes = self.draw_changes(dynamic_sprites.clone());
		let mut field = None;
		let mut views: HashMap<PlayerId, WorldMessage> = HashMap::new();
		for (playerid, player) in self.players.iter_mut() {
			let mut wm = WorldMessage::default();
			if let Some(body) = player.body.as_ref().and_then(|id| self.creatures.get(id)){
				let in_view_range = player.view_center
					.map_or(
						false,
						|view_center|
							(view_center.x - body.pos.x).abs() < 32 && 
							(view_center.y - body.pos.y).abs() < 32
					);
				if changes.is_some() && !player.is_new && in_view_range {
					wm.change = changes.clone();
				} else {
					let view_center = body.pos;
					if field.is_none(){
						field = Some(draw_field(view_center, Pos::new(128, 128), &mut self.ground, &dynamic_sprites));
					}
					wm.field = Some(field.clone().unwrap());
					player.is_new = false;
					player.view_center = Some(view_center);
				}
				wm.pos = Some(body.pos);
			}
			views.insert(playerid.clone(), wm);
		}
		self.drawing = Some(dynamic_sprites);
		self.ground.flush();
		views
	}
	
	pub fn nplayers(&self) -> usize {
		self.players.len()
	}
}


fn draw_field(center: Pos, size: Pos, tiles: &mut Ground, sprites: &HashMap<Pos, Vec<Sprite>>) -> FieldMessage {
	println!("redrawing field");
	let mut values :Vec<usize> = Vec::with_capacity((size.x * size.y) as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	let min = center - size / 2;
	for y in 0..size.y {
		for x in 0..size.x {
			let pos = Pos::new(x, y) + min;
			let mut tile_sprites = Vec::new();
			if let Some(dynamic_sprites) = sprites.get(&pos) {
				tile_sprites.extend_from_slice(dynamic_sprites);
			}
			let tile = tiles.cell(pos);
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
	}
	FieldMessage {
		width: size.x,
		height: size.y,
		field: values,
		mapping,
		offset: min
	}
}

