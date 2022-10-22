
use crate::{
	inventory::Item,
	tile::Structure,
	timestamp::Timestamp,
	random
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
	Take,
	Smash,
	Cut
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
	typ: ActionType,
	level: u32,
	use_item: bool
}

impl Action{
	pub fn take() -> Self {
		Self { typ: ActionType::Take, level: 0, use_item: false}
	}
	
	pub fn new(typ: ActionType, level: u32, use_item: bool) -> Self {
		Self { typ, level, use_item }
	}
}


#[derive(Debug, Clone)]
pub struct Interactable {
	remains: Structure,
	loot: Vec<(Vec<Item>, u32)>,
	action_type: ActionType,
	min_level: u32,
	level_odds: Vec<f32>
}

impl Interactable {
	pub fn new(action_type: ActionType, min_level: u32, level_odds: &[f32], remains: Structure, items: &[Item]) -> Self {
		Self {
			action_type,
			min_level,
			level_odds: level_odds.to_vec(),
			remains, loot: vec![(items.to_vec(), 1)]
		}
	}
	pub fn loot(action_type: ActionType, min_level: u32, level_odds: &[f32], remains: Structure, loot: &[(&[Item], u32)]) -> Self {
		Self {
			action_type,
			min_level,
			level_odds: level_odds.to_vec(), 
			remains,
			loot: loot.iter().map(|(item, chance)| (item.to_vec(), *chance)).collect()
		}
	}
	pub fn take(items: &[Item]) -> Self {
		Self::new(ActionType::Take, 0, &[], Structure::Air, items)
	}
	
	pub fn apply(&self, action: Action, time: Timestamp) -> Option<InteractionResult> {
		if self.action_type == action.typ && action.level >= self.min_level {
			let relative_level = (action.level - self.min_level) as usize;
			let odds = if relative_level >= self.level_odds.len() {
				1.0
			} else {
				self.level_odds[relative_level]
			};
			Some(InteractionResult {
				remains: Some(self.remains),
				items: if odds >= random::random_float(time.random_seed() ^ 84217) {
					random::pick_weighted(
						time.random_seed(),
						&self.loot
					).to_vec()
				} else {
					Vec::new()
				},
				use_item: action.use_item
			})
		} else {
			None
		}
	}
}


#[derive(Debug, Clone)]
pub struct InteractionResult {
	pub remains: Option<Structure>,
	pub items: Vec<Item>,
	pub use_item: bool
}
