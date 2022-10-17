
use serde::{Serialize, Deserialize};
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
	
	#[allow(dead_code)]
	pub fn selected(&self) -> Item {
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
}

pub type InventorySave = Vec<(Item, usize)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Item {
	#[serde(rename="<hands>")]
	Hands,
	Reed,
	Flower
}
