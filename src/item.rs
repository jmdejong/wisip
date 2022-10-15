
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Item {
	Axe,
	OakWood,
	OakNut,
	RadishSeed,
	Radish,
}

#[allow(dead_code)]
pub struct ItemDef {
	is_tool: bool,
}

impl Item {
	#[allow(dead_code)]
	pub fn properties(&self) -> ItemDef {
		match self {
			Item::Axe => ItemDef{is_tool: true},
			Item::OakWood => ItemDef{is_tool: false},
			Item::OakNut => ItemDef{is_tool: false},
			Item::RadishSeed => ItemDef{is_tool: false},
			Item::Radish => ItemDef{is_tool: false}
		}
	}
}
