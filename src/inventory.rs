
use serde::{Serialize, Deserialize};
use enum_assoc::Assoc;
use crate::{
	worldmessages::InventoryMessage
};


#[derive(Debug, Clone)]
pub struct Inventory {
	items: Vec<(Item, usize)>,
	selector: usize
}

impl Inventory {
	
	pub fn add(&mut self, item: Item) {
		for entry in self.items.iter_mut() {
			if entry.0 == item {
				entry.1 += 1;
				return;
			}
		}
		self.items.push((item, 1));
	}
	
	fn selected(&self) -> Item {
		if self.selector == 0 {
			Item::Hands
		} else {
			self.items[self.selector - 1].0
		}
	}
	
	pub fn view(&self) -> InventoryMessage {
		let mut view = vec![(Item::Hands, 1)];
		view.extend(&self.items);
		(view, self.selector)
	}
	
	pub fn save(&self) -> InventorySave {
		self.items.clone()
	}
	
	pub fn load(saved: InventorySave) -> Self {
		Self {
			items: saved,
			selector: 0
		}
	}
	
	pub fn select_next(&mut self) {
		self.selector = (self.selector + 1 ).rem_euclid(self.items.len() + 1);
	}
	pub fn select_previous(&mut self) {
		self.selector = (self.selector + self.items.len()).rem_euclid(self.items.len() + 1);
	}
	
	pub fn selected_action(&self) -> Option<Action> {
		self.selected().action()
	}
}

pub type InventorySave = Vec<(Item, usize)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Assoc)]
#[serde(rename_all="lowercase")]
#[func(pub fn action(&self) -> Option<Action>)]
pub enum Item {
	#[serde(rename="<take>")]
	#[assoc(action=Action::take(1))]
	Hands,
	Reed,
	Flower,
	Pebble,
	#[assoc(action=Action::smash(1))]
	Stone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
	Take,
	Smash
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
	typ: ActionType,
	level: u32
}

impl Action{
	pub fn take(level: u32) -> Self {
		Self { typ: ActionType::Take, level }
	}
	pub fn smash(level: u32) -> Self {
		Self { typ: ActionType::Smash, level }
	}
	
	pub fn fulfilled_by(&self, other: Action) -> bool {
		other.typ == self.typ && other.level >= self.level
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn selects_take_action() {
		let inv = Inventory::load(vec![]);
		assert_eq!(inv.selected(), Item::Hands);
		assert_eq!(inv.selected_action(), Some(Action::take(1)));
	}
	#[test]
	fn hands_has_take_action() {
		assert_eq!(Item::Hands.action(), Some(Action::take(1)));
	}
}

