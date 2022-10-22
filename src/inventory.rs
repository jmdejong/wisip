
use serde::{Serialize, Deserialize};
use enum_assoc::Assoc;
use crate::{
	worldmessages::InventoryMessage,
	action::{Action, ActionType::*}
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
	
	fn selected(&self) -> Item {
		if self.selector == 0 {
			Item::Hands
		} else {
			self.items[self.selector - 1].0
		}
	}
	
	pub fn remove_selected(&mut self) {
		if self.selector == 0 {
			return;
		}
		let idx = self.selector - 1;
		self.items[idx].1 -= 1;
		if self.items[idx].1 == 0 {
			self.items.remove(idx);
			self.selector -= 1;
		}
	}
}

pub type InventorySave = Vec<(Item, usize)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Assoc)]
#[serde(rename_all="snake_case")]
#[func(pub fn action(&self) -> Option<Action>)]
pub enum Item {
	#[serde(rename="<take>")]
	#[assoc(action=Action::take())]
	Hands,
	Reed,
	Flower,
	Pebble,
	#[assoc(action=Action::new(Smash, 1, true))]
	Stone,
	#[assoc(action=Action::new(Cut, 1, false))]
	SharpStone,
	#[assoc(action=Action::new(Fill, 1, true))]
	Pitcher,
	#[assoc(action=Action::new(Water, 1, false))]
	FilledPitcher,
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn selects_take_action() {
		let inv = Inventory::load(vec![]);
		assert_eq!(inv.selected(), Item::Hands);
		assert_eq!(inv.selected_action(), Some(Action::take()));
	}
	#[test]
	fn hands_has_take_action() {
		assert_eq!(Item::Hands.action(), Some(Action::take()));
	}
}

