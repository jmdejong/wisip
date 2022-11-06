
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

const FIXED_ENTRIES: usize = 2;

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
		let view = [(Item::Eyes, 1), (Item::Hands, 1)].iter()
			.chain(self.items.iter())
			.map(|(item, count)| (item.name().to_string(), *count))
			.collect();
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
		self.items.len() + FIXED_ENTRIES
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
	
	pub fn move_selected(&mut self, selector: Selector) {
		if self.selector < FIXED_ENTRIES {
			return;
		}
		let target = match selector {
			Selector::Next => self.selector + 1,
			Selector::Previous => self.selector - 1,
			Selector::Idx(idx) => idx,
		};
		if target < FIXED_ENTRIES || target >= self.count() {
			return;
		}
		let item = self.items.remove(self.selector - FIXED_ENTRIES);
		self.items.insert(target - FIXED_ENTRIES, item);
		self.select(selector);
	}
	
	pub fn selected(&self) -> Item {
		if self.selector == 0 {
			Item::Eyes
		} else if self.selector == 1 {
			Item::Hands
		} else {
			self.items[self.selector - FIXED_ENTRIES].0
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
#[func(pub fn name(&self) -> &str)]
pub enum Item {
	#[assoc(name="<eyes>")]
	#[assoc(action=Action::Inspect)]
	#[assoc(description="Inspect things around you")]
	Eyes,
	
	#[assoc(name="<hands>")]
	#[assoc(action=Action::take())]
	#[assoc(description="Take items that are laying loose")]
	Hands,
	
	#[assoc(name="reed")]
	#[assoc(description="Some cut reeds")]
	Reed,
	
	#[assoc(name="flower")]
	#[assoc(description="A pretty flower")]
	#[assoc(action=Action::Craft(CraftType::Marker, Item::MarkerStone, hashmap![Item::Stone => 1, Item::Flower => 9]))]
	Flower,
	
	#[assoc(name="pebble")]
	#[assoc(description="A small stone")]
	Pebble,
	
	#[assoc(name="stone")]
	#[assoc(description="A mid-size stone. Stones can be broken by smashing two together")]
	#[assoc(action=Action::new(Smash, 1, true))]
	Stone,
	
	#[assoc(name="sharp stone")]
	#[assoc(description="A small stone with a sharp edge. It can be used to cut things, though it is very crude and may not always work")]
	#[assoc(action=Action::new(Cut, 1, false))]
	SharpStone,
	
	#[assoc(name="pitcher")]
	#[assoc(description="A pitcher from the pitcher plant. It can function as a bucket")]
	#[assoc(action=Action::Craft(CraftType::Water, Item::FilledPitcher, HashMap::new()))]
	Pitcher,
	
	#[assoc(name="water pitcher")]
	#[assoc(description="A pitcher from the pitcher plant, filled with water")]
	#[assoc(action=Action::new(Water, 1, false))]
	FilledPitcher,
	
	#[assoc(name="hoe")]
	#[assoc(description="A simple hoe that can be used to clear the ground of small vegetation")]
	#[assoc(action=Action::Clear)]
	Hoe,
	
	#[assoc(name="green seeds")]
	#[assoc(description="Unknown green seeds")]
	#[assoc(action=Action::Build(Structure::GreenSeed, HashMap::new()))]
	GreenSeed,
	
	#[assoc(name="yellow seeds")]
	#[assoc(action=Action::Build(Structure::YellowSeed, HashMap::new()))]
	#[assoc(description="Unknown yellow seeds")]
	YellowSeed,
	
	#[assoc(name="brown seeds")]
	#[assoc(action=Action::Build(Structure::BrownSeed, HashMap::new()))]
	#[assoc(description="Unknown brown seeds")]
	BrownSeed,
	
	#[assoc(name="stick")]
	#[assoc(description="Stick")]
	Stick,
	
	#[assoc(name="tinder")]
	#[assoc(description="Tinder from the tinder fungus. Can be placed with some pebbles on a clear space to create a fireplace")]
	#[assoc(action=Action::Build(Structure::Fireplace, hashmap![Item::Pebble => 10]))]
	Tinder,
	
	#[assoc(name="marker stone")]
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
		inv.select(Selector::Next);
		assert_eq!(inv.selected(), Item::Hands);
	}
	#[test]
	fn selects_stone() {
		let mut inv = Inventory::load(vec![(Item::Stone, 1)]);
		inv.select(Selector::Idx(2));
		assert_eq!(inv.selected(), Item::Stone);
	}
	#[test]
	fn hands_has_take_action() {
		assert_eq!(Item::Hands.action(), Some(Action::take()));
	}
	
}

