
use std::collections::HashMap;
use crate::{
	item::Item,
	tile::{Structure, Ground},
	timestamp::Timestamp,
	worldmessages::SoundType,
	random
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionType {
	Take,
	Smash,
	Cut,
	Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CraftType {
	Marker,
	Water
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interact {
	typ: InteractionType,
	level: u32,
	pub use_item: bool
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
	Interact(Interact),
	Clear,
	Inspect,
	BuildClaim(Structure),
	Craft(CraftType, Item, HashMap<Item, usize>),
	Build(Structure, HashMap<Item, usize>),
}

impl Action{
	pub fn take() -> Self {
		Self::new(InteractionType::Take, 0, false)
	}
	
	pub fn new(typ: InteractionType, level: u32, use_item: bool) -> Self {
		Self::Interact(Interact { typ, level, use_item } )
	}
}


#[derive(Debug, Clone)]
pub struct Interactable {
	remains: Option<Structure>,
	items: Vec<Item>,
	action_type: InteractionType,
	min_level: u32,
	level_odds: Vec<f32>
}

impl Interactable {
	pub fn new(action_type: InteractionType, min_level: u32, level_odds: &[f32], remains: Option<Structure>, items: &[Item]) -> Self {
		Self {
			action_type,
			min_level,
			level_odds: level_odds.to_vec(),
			remains,
			items: items.to_vec()
		}
	}
	
	pub fn harvest(action_type: InteractionType, min_level: u32, level_odds: &[f32], items: &[Item]) -> Self {
		Self::new(action_type, min_level, level_odds, Some(Structure::Air), items)
	}
	
	pub fn take(items: &[Item]) -> Self {
		Self::new(InteractionType::Take, 0, &[], Some(Structure::Air), items)
	}
	
	pub fn apply(&self, action: Interact, time: Timestamp) -> Option<InteractionResult> {
		if self.action_type == action.typ && action.level >= self.min_level {
			let relative_level = (action.level - self.min_level) as usize;
			let odds = if relative_level < self.level_odds.len() {
				self.level_odds[relative_level]
			} else if !self.level_odds.is_empty() {
				self.level_odds[self.level_odds.len() - 1]
			} else {
				1.0
			};
			Some(InteractionResult {
				remains: self.remains,
				items: if odds >= random::random_float(time.random_seed() ^ 84217) {
					self.items.clone()
				} else {
					Vec::new()
				},
				..Default::default()
			})
		} else {
			None
		}
	}
}


#[derive(Debug, Clone, Default)]
pub struct InteractionResult {
	pub remains: Option<Structure>,
	pub remains_ground: Option<Ground>,
	pub items: Vec<Item>,
	pub cost: HashMap<Item, usize>,
	pub message: Option<(SoundType, String)>,
	pub claim: bool,
	pub build: bool,
}
