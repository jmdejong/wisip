
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
		let mut view = vec![(Item::Eyes, 1), (Item::Hands, 1)];
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
	
	fn count(&self) -> usize {
		self.items.len() + 2
	}
	
	pub fn select_next(&mut self) {
		self.selector = (self.selector + 1).rem_euclid(self.count())
	}
	pub fn select_previous(&mut self) {
		self.selector = (self.selector + self.count() - 1).rem_euclid(self.count());
	}
	
	pub fn selected(&self) -> Item {
		if self.selector == 0 {
			Item::Eyes
		} else if self.selector == 1 {
			Item::Hands
		} else {
			self.items[self.selector - 2].0
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
#[func(pub fn description(&self) -> Option<&str>)]
pub enum Item {
	#[serde(rename="<inspect>")]
	#[assoc(action=Action::Inspect)]
	#[assoc(description="Inspect things around you")]
	Eyes,
	#[serde(rename="<take>")]
	#[assoc(action=Action::take())]
	#[assoc(description="Take items that are laying loose")]
	Hands,
	#[assoc(description="Some cut reeds")]
	Reed,
	#[assoc(description="A pretty flower")]
	Flower,
	#[assoc(description="A small stone")]
	Pebble,
	#[assoc(description="A mid-size stone. Stones can be broken by smashing two together")]
	#[assoc(action=Action::new(Smash, 1, true))]
	Stone,
	#[assoc(description="A small stone with a sharp edge. It can be used to cut things, though it is very crude and may not always work")]
	#[assoc(action=Action::new(Cut, 1, false))]
	SharpStone,
	#[assoc(description="A pitcher from the pitcher plant. It can function as a bucket")]
	#[assoc(action=Action::Fill(Item::FilledPitcher))]
	Pitcher,
	#[assoc(description="A pitcher from the pitcher plant, filled with water")]
	#[assoc(action=Action::new(Water, 1, false))]
	FilledPitcher,
	#[assoc(description="A simple hoe that can be used to clear the ground of small vegetation")]
	#[assoc(action=Action::Clear)]
	Hoe,
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn selects_eyes() {
		let inv = Inventory::load(vec![]);
		assert_eq!(inv.selected(), Item::Eyes);
	}
	#[test]
	fn selects_take_hands() {
		let mut inv = Inventory::load(vec![]);
		inv.select_next();
		assert_eq!(inv.selected(), Item::Hands);
	}
	#[test]
	fn selects_stone() {
		let mut inv = Inventory::load(vec![(Item::Stone, 1)]);
		inv.select_next();
		inv.select_next();
		assert_eq!(inv.selected(), Item::Stone);
	}
	#[test]
	fn hands_has_take_action() {
		assert_eq!(Item::Hands.action(), Some(Action::take()));
	}
	
}

