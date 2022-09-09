
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
	creature::{Creature, Mind},
	tile::Tile,
	player::Player,
	mapgen::{MapTemplate, MapType, create_map},
	grid::Grid
};

pub struct World {
	time: Timestamp,
	size: Pos,
	ground: Grid<Tile>,
	players: HashMap<PlayerId, Player>,
	creatures: Holder<usize, Creature>,
	spawnpoint: Pos,
	map: MapType,
	drawing: Option<HashMap<Pos, Vec<Sprite>>>,
}

impl World {
	
	pub fn new(map: MapType) -> Self {
		
		let mut world = World {
			size: Pos::new(0, 0),
			spawnpoint: Pos::new(0, 0),
			ground: Grid::empty(),
			players: HashMap::new(),
			creatures: Holder::new(),
			time: Timestamp(0),
			map,
			drawing: None,
		};
		world.reset();
		world
	}
	
	pub fn reset(&mut self) {
		self.creatures.clear();
		let template: MapTemplate = create_map(&self.map);
		self.size = template.size;
		self.ground = template.ground;
		self.spawnpoint = template.spawnpoint;
		self.drawing = None;
		for player in self.players.values_mut() {
			player.is_new = true;
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
				body: 0,
				is_new: true,
				view_center: None,
				inventory: Vec::new()
			}
		);
		Ok(())
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let player = self.players.remove(playerid).ok_or(aerr!("player {} not found", playerid))?;
		self.creatures.remove(&player.body);
		Ok(())
	}
	
	pub fn control_player(&mut self, playerid: PlayerId, control: Control) -> Result<()>{
		let player = self.players.get_mut(&playerid).ok_or(aerr!("player not found"))?;
		player.plan = Some(control);
		Ok(())
	}
	
	fn creature_plan(&self, creature: &Creature) -> Option<Control> {
		match &creature.mind {
			Mind::Player(playerid) => {
				if let Some(player) = self.players.get(&playerid) {
					player.plan.clone()
				} else {Some(Control::Suicide)}
			}
		}
	}
	
	fn update_creatures(&mut self) -> Option<()> {
		let mut creature_map: HashMap<Pos, usize> = self.creatures.iter()
			.map(|(creatureid, creature)| (creature.pos, *creatureid))
			.collect();
		let plans: HashMap<usize, Control> = self.creatures.iter()
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
					if let Some(tile) = self.ground.get(newpos) {
						if !tile.blocking() && !creature_map.contains_key(&newpos) {
							if creature_map.get(&creature.pos) == Some(id){
								creature_map.remove(&creature.pos);
							}
							creature_map.insert(newpos, *id);
							creature.pos = newpos;
						}
					}
				}
				Some(Control::Suicide) => {
					creature.kill();
				}
				Some(Control::Use(_direction)) => {
				
				}
				None => { }
			}
		}
		Some(())
	}
	
	
	fn spawn(&mut self){
		
		// spawn players
		for (playerid, player) in self.players.iter_mut() {
			if !self.creatures.contains_key(&player.body) {
				let body = self.creatures.insert(Creature::new_player(
					playerid.clone(),
					self.spawnpoint
				));
				player.body = body
			}
			player.plan = None;
		}
	}
	
	pub fn update(&mut self) {
		self.update_creatures();
		
		self.spawn();
		
		self.time.increment();
	}
	
	
	fn draw_dynamic(&self) -> HashMap<Pos, Vec<Sprite>> {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = HashMap::new();
		for creature in self.creatures.values() {
			sprites.entry(creature.pos).or_insert_with(Vec::new).push(creature.sprite);
		}
		sprites.into_iter().filter_map(|(pos, mut sprs)| {
			sprs.append(&mut self.ground.get(pos)?.sprites());
			Some((pos, sprs))
		}).collect()
	}
	
	fn draw_changes(&self, mut sprites: HashMap<Pos, Vec<Sprite>>) -> Option<ChangeMessage> { 
		if let Some(last_drawing) = &self.drawing {
			for pos in last_drawing.keys() {
				sprites.entry(*pos).or_insert(self.ground.get(*pos)?.sprites());
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
			if let Some(body) = self.creatures.get(&player.body){
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
						field = Some(draw_field(view_center, Pos::new(128, 128), &self.ground, &dynamic_sprites));
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
		views
	}
	
	pub fn nplayers(&self) -> usize {
		self.players.len()
	}
}


fn draw_field(center: Pos, size: Pos, tiles: &Grid<Tile>, sprites: &HashMap<Pos, Vec<Sprite>>) -> FieldMessage {
	println!("redrawing field");
	let mut values :Vec<usize> = Vec::with_capacity((size.x * size.y) as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	let min = center - size / 2;
	for y in 0..size.y {
		for x in 0..size.x {
			let pos = Pos::new(x, y) + min;
			let mut tile_sprites = Vec::new();
// 			vec![tiles.get_unchecked(Pos::new(x, y)).sprite()];
			
			if let Some(dynamic_sprites) = sprites.get(&pos) {
				tile_sprites.extend_from_slice(dynamic_sprites);
			}
			if let Some(tile) = tiles.get(pos) {
				tile_sprites.append(&mut tile.sprites());
			}
// 			let sprs: &Vec<Sprite> = sprites.get(&Pos{x, y}).unwrap_or(&tilesprite);
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

