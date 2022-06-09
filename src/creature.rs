

use crate::{
	sprite::Sprite,
	Pos,
	PlayerId,
	timestamp::Duration,
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
	is_dead: bool
}

impl Creature {
	
	pub fn is_player(&self) -> bool {
		matches!(self.mind, Mind::Player(_))
	}
	
	
	pub fn new_player(playerid: PlayerId, pos: Pos) -> Self {
		Self {
			mind: Mind::Player(playerid.clone()),
			pos,
			cooldown: Duration(0),
			walk_cooldown: Duration(0),
			sprite: Sprite::PlayerDefault,
			is_dead: false
		}
	}
	
	
	pub fn is_dead(&self) -> bool {
		self.is_dead
	}
	
	pub fn kill(&mut self) {
		self.is_dead = true;
	}
}
