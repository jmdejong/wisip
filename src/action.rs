
use crate::{
	inventory::Item,
	tile::Structure,
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
	
	pub fn new(typ: ActionType, level: u32) -> Self {
		Self { typ, level, use_item: false }
	}
}


#[derive(Debug, Clone)]
pub struct Interactable {
	remains: Structure,
	items: Vec<Item>,
	action_type: ActionType,
	action_level: u32
}

impl Interactable {
	pub fn new(action_type: ActionType, action_level: u32, remains: Structure, items: &[Item]) -> Self {
		Self {action_type, action_level, remains, items: items.to_vec() }
	}
	pub fn take(items: &[Item]) -> Self {
		Self::new(ActionType::Take, 0, Structure::Air, items)
	}
	
	pub fn apply(&self, action: Action) -> Option<InteractionResult> {
		if self.action_type == action.typ && action.level >= self.action_level {
			Some(InteractionResult {
				remains: Some(self.remains),
				items: self.items.clone(),
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
