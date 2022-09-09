
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ItemRef {
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

impl ItemRef {
	#[allow(dead_code)]
	pub fn properties(&self) -> ItemDef {
		match self {
			ItemRef::Axe => ItemDef{is_tool: true},
			ItemRef::OakWood => ItemDef{is_tool: false},
			ItemRef::OakNut => ItemDef{is_tool: false},
			ItemRef::RadishSeed => ItemDef{is_tool: false},
			ItemRef::Radish => ItemDef{is_tool: false}
		}
	}
}
