

use serde::{Serialize, Deserialize};
use crate::{
	sprite::Sprite,
	Pos,
	PlayerId,
	timestamp::Duration,
	util::HolderId,
	inventory::{Inventory, InventorySave}
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mind {
	Player(PlayerId)
}

#[derive(Debug, Clone)]
pub struct Creature {
	pub mind: Mind,
	pub pos: Pos,
	pub cooldown: Duration,
	pub walk_cooldown: Duration,
	pub sprite: Sprite,
	pub inventory: Inventory,
	is_dead: bool
}

impl Creature {
	
	#[allow(dead_code)]
	pub fn is_player(&self) -> bool {
		matches!(self.mind, Mind::Player(_))
	}
	
	
	pub fn load_player(playerid: PlayerId, saved: PlayerSave) -> Self {
		Self {
			mind: Mind::Player(playerid),
			pos: saved.pos,
			cooldown: Duration(0),
			walk_cooldown: Duration(0),
			sprite: Sprite::PlayerDefault,
			inventory: Inventory::load(saved.inventory),
			is_dead: false
		}
	}
	
	
	#[allow(dead_code)]
	pub fn is_dead(&self) -> bool {
		self.is_dead
	}
	
	pub fn kill(&mut self) {
		self.is_dead = true;
	}
	
	pub fn save(&self) -> PlayerSave {
		PlayerSave {
			pos: self.pos,
			inventory: self.inventory.save()
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreatureId(pub usize);

impl HolderId for CreatureId {
	fn next(&self) -> Self { Self(self.0 + 1) }
	fn initial() -> Self { Self(1) }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSave {
	pub inventory: InventorySave,
	pub pos: Pos
}

impl PlayerSave {
	pub fn new(pos: Pos) -> Self {
		Self {
			pos,
			inventory: Vec::new()
		}
	}
}
