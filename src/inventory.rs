
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use enum_assoc::Assoc;
use crate::{
	worldmessages::InventoryMessage,
	action::{Action, InteractionType::*, CraftType},
	controls::Selector,
	tile::Structure,
	hashmap,
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
	
	pub fn select(&mut self, selector: Selector) {
		self.selector = match selector {
			Selector::Next =>
				(self.selector + 1).rem_euclid(self.count()),
			Selector::Previous =>
				(self.selector + self.count() - 1).rem_euclid(self.count()),
			Selector::Idx(idx) =>
				idx.min(self.count() - 1),
		}
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
	
	pub fn pay(&mut self, mut cost: HashMap<Item, usize>) -> bool {
		if cost.is_empty() {
			return true;
		}
		if let Some(items) = self.items.iter()
				.map(|(item, n)| {
					let amount = cost.remove(item).unwrap_or(0);
					if amount > *n {
						None
					} else {
						Some((*item, *n - amount))
					}
				})
				.collect::<Option<Vec<(Item, usize)>>>() {
			if !cost.is_empty() {
				false
			} else {
				self.items = items;
				self.items.retain(|(_, n)| *n > 0);
				self.selector = self.selector.min(self.count() - 1);
				true
			}
		} else {
			false
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
	#[assoc(action=Action::Craft(CraftType::Marker, Item::MarkerStone, hashmap![Item::Stone => 1, Item::Flower => 9]))]
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
	#[assoc(action=Action::Craft(CraftType::Water, Item::FilledPitcher, HashMap::new()))]
	Pitcher,
	
	#[assoc(description="A pitcher from the pitcher plant, filled with water")]
	#[assoc(action=Action::new(Water, 1, false))]
	FilledPitcher,
	
	#[assoc(description="A simple hoe that can be used to clear the ground of small vegetation")]
	#[assoc(action=Action::Clear)]
	Hoe,
	
	#[assoc(description="Tinder from the tinder fungus. Can be placed with some pebbles on a clear space to create a fireplace")]
	#[assoc(action=Action::Build(Structure::Fireplace, hashmap![Item::Pebble => 10]))]
	Tinder,
	
	#[assoc(description="A marker stone that can be placed to create a land claim")]
	#[assoc(action=Action::BuildClaim(Structure::MarkStone))]
	MarkerStone,
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

