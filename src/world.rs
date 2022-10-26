
use std::collections::{HashMap};
use serde::{Serialize, Deserialize};

use crate::{
	PlayerId,
	controls::{Control, Selection},
	Result,
	aerr,
	pos::{Pos, Area},
	util::Holder,
	sprite::Sprite,
	worldmessages::{WorldMessage, FieldMessage, ChangeMessage},
	timestamp::{Timestamp},
	creature::{Creature, Mind, CreatureId, PlayerSave},
	player::Player,
	ground::{Ground, GroundSave}
};

pub struct World {
	pub name: String,
	pub time: Timestamp,
	ground: Ground,
	players: HashMap<PlayerId, Player>,
	creatures: Holder<CreatureId, Creature>,
	drawing: Option<HashMap<Pos, Vec<Sprite>>>,
}

impl World {
	
	pub fn new(name: String) -> Self {
		let time = Timestamp(0);
		Self {
			name,
			ground: Ground::new(time),
			players: HashMap::new(),
			creatures: Holder::new(),
			time,
			drawing: None,
		}
	}
	
	pub fn default_player(&mut self) -> PlayerSave {
		PlayerSave::new(self.ground.player_spawn())
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId, saved: PlayerSave) -> Result<()> {
		if self.players.contains_key(playerid){
			return Err(aerr!("player {} already exists", playerid));
		}
		let body = self.creatures.insert(Creature::load_player(playerid.clone(), saved));
		self.players.insert(
			playerid.clone(),
			Player::new(body)
		);
		Ok(())
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let player = self.players.remove(playerid).ok_or_else(|| aerr!("player {} not found", playerid))?;
		self.creatures.remove(&player.body);
		Ok(())
	}
	
	pub fn save_player(&self, playerid: &PlayerId) -> Result<PlayerSave> {
		let player = self.players.get(playerid).ok_or_else(|| aerr!("player {} not found", playerid))?;
		let body = self.creatures.get(&player.body).ok_or_else(|| aerr!("player body for {} not found", playerid))?;
		Ok(body.save())
	}
	
	pub fn control_player(&mut self, playerid: &PlayerId, control: Control) -> Result<()> {
		let player = self.players.get_mut(playerid).ok_or_else(|| aerr!("player not found"))?;
		player.plan = Some(control);
		Ok(())
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
				Some(Control::Select(Selection::Next)) => {
					creature.inventory.select_next();
				}
				Some(Control::Select(Selection::Previous)) => {
					creature.inventory.select_previous();
				}
				Some(Control::Interact(direction)) => {
					let pos = creature.pos + direction.map(|dir| dir.to_position()).unwrap_or_else(Pos::zero);
					let tile = self.ground.cell(pos);
					let item = creature.inventory.selected();
					if let Some(interaction) = tile.interact(item, self.time) {
						if interaction.use_item {
							creature.inventory.remove_selected();
						}
						for item in interaction.items {
							creature.inventory.add(item);
						}
						if let Some(remains) = interaction.remains {
							self.ground.set_structure(pos, remains);
						}
						if let Some(remains_ground) = interaction.remains_ground {
							self.ground.set_ground(pos, remains_ground);
						}
					}
				}
				None => { }
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
				sprites.entry(*pos).or_insert_with(||self.ground.cell(*pos).sprites());
			}
			for (pos, tile) in self.ground.modified().into_iter() {
				sprites.entry(pos).or_insert_with(||tile.sprites());
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
			if let Some(body) = self.creatures.get(&player.body) {
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
					player.is_new = false;
					player.view_center = Some(body.pos);
					if field.is_none(){
						field = Some(draw_field(player.view_area().unwrap(), &mut self.ground, &dynamic_sprites));
					}
					wm.field = Some(field.clone().unwrap());
				}
				wm.pos = Some(body.pos);
				wm.inventory = Some(body.inventory.view());
			}
			views.insert(playerid.clone(), wm);
		}
		self.drawing = Some(dynamic_sprites);
		self.ground.flush();
		views
	}
	
	pub fn save(&self) -> WorldSave {
		WorldSave {
			name: self.name.clone(),
			time: self.time,
			ground: self.ground.save()
		}
	}
	
	pub fn load(save: WorldSave) -> World {
		World {
			name: save.name,
			ground: Ground::load(save.ground, save.time),
			players: HashMap::new(),
			creatures: Holder::new(),
			time: save.time,
			drawing: None
		}
	}
}


fn draw_field(area: Area, tiles: &mut Ground, sprites: &HashMap<Pos, Vec<Sprite>>) -> FieldMessage {
	println!("redrawing field");
	let mut values :Vec<usize> = Vec::with_capacity((area.size().x * area.size().y) as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	for pos in area.iter() {
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
	FieldMessage {
		width: area.size().x,
		height: area.size().y,
		field: values,
		mapping,
		offset: area.min()
	}
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSave {
	name: String,
	time: Timestamp,
	ground: GroundSave
}

