
use crate::{
	inventory::Item,
	tile::{Structure, Ground},
	timestamp::Timestamp,
	random
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
	Take,
	Smash,
	Cut,
	Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interact {
	typ: ActionType,
	level: u32,
	use_item: bool
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
	Interact(Interact),
	Fill(Item),
	Clear
}

impl Action{
	pub fn take() -> Self {
		Self::new(ActionType::Take, 0, false)
	}
	
	pub fn new(typ: ActionType, level: u32, use_item: bool) -> Self {
		Self::Interact(Interact { typ, level, use_item } )
	}
}


#[derive(Debug, Clone)]
pub struct Interactable {
	remains: Option<Structure>,
	items: Vec<Item>,
	action_type: ActionType,
	min_level: u32,
	level_odds: Vec<f32>
}

impl Interactable {
	pub fn new(action_type: ActionType, min_level: u32, level_odds: &[f32], remains: Option<Structure>, items: &[Item]) -> Self {
		Self {
			action_type,
			min_level,
			level_odds: level_odds.to_vec(),
			remains: remains,
			items: items.to_vec()
		}
	}
	
	pub fn harvest(action_type: ActionType, min_level: u32, level_odds: &[f32], items: &[Item]) -> Self {
		Self::new(action_type, min_level, level_odds, Some(Structure::Air), items)
	}
	
	pub fn take(items: &[Item]) -> Self {
		Self::new(ActionType::Take, 0, &[], Some(Structure::Air), items)
	}
	
	pub fn apply(&self, action: Interact, time: Timestamp) -> Option<InteractionResult> {
		if self.action_type == action.typ && action.level >= self.min_level {
			let relative_level = (action.level - self.min_level) as usize;
			let odds = if relative_level < self.level_odds.len() {
				self.level_odds[relative_level]
			} else if self.level_odds.len() > 0 {
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
				use_item: action.use_item,
				..Default::default()
			})
		} else {
			None
		}
	}
}


#[derive(Debug, Clone)]
pub struct InteractionResult {
	pub remains: Option<Structure>,
	pub remains_ground: Option<Ground>,
	pub items: Vec<Item>,
	pub use_item: bool,
	pub message: Option<String>
}

impl InteractionResult {
	pub fn exchange(item: Item) -> Self {
		Self {
			items: vec![item],
			..Default::default()
		}
	}
}

impl Default for InteractionResult {
	fn default() -> Self {
		Self {
			remains: None,
			remains_ground: None,
			items: Vec::new(),
			use_item: false,
			message: None
		}
	}
}
