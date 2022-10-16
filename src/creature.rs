

use crate::{
	sprite::Sprite,
	Pos,
	PlayerId,
	timestamp::Duration,
	util::HolderId,
	inventory::Inventory
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
	
	
	pub fn new_player(playerid: PlayerId, pos: Pos) -> Self {
		Self {
			mind: Mind::Player(playerid),
			pos,
			cooldown: Duration(0),
			walk_cooldown: Duration(0),
			sprite: Sprite::PlayerDefault,
			inventory: Inventory::new(),
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreatureId(pub usize);

impl HolderId for CreatureId {
	fn next(&self) -> Self { Self(self.0 + 1) }
	fn initial() -> Self { Self(1) }
}
