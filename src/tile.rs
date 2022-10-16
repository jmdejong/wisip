
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use enum_assoc::Assoc;
use crate::{
	sprite::Sprite,
	inventory::Item
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Option<Sprite>)]
#[func(fn accessible(&self) -> bool {true})]
pub enum Ground {
	#[assoc(sprite = Sprite::Dirt)]
	Dirt,
	#[assoc(sprite = Sprite::Grass1)]
	Grass1,
	#[assoc(sprite = Sprite::Grass2)]
	Grass2,
	#[assoc(sprite = Sprite::Grass3)]
	Grass3,
	#[assoc(sprite = Sprite::Sanctuary)]
	Sanctuary,
	#[assoc(sprite = Sprite::Water)]
	#[assoc(accessible = false)]
	Water,
	#[assoc(accessible = false)]
	Empty
}

#[derive(Debug, Clone)]
pub struct Interaction {
	remains: Structure,
	items: Vec<Item>
}

impl Interaction {
	pub fn take(items: &[Item]) -> Self {
		Self { remains: Structure::Air, items: items.to_vec()}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Option<Sprite>)]
#[func(fn blocking(&self) -> bool {false})]
#[func(fn interaction(&self) -> Option<Interaction>)]
pub enum Structure {
	Air,
	#[assoc(sprite = Sprite::Wall)]
	#[assoc(blocking = true)]
	Wall,
	#[assoc(sprite = Sprite::Rock)]
	#[assoc(blocking = true)]
	Rock,
	#[assoc(sprite = Sprite::Tree)]
	#[assoc(blocking = true)]
	Tree,
	#[assoc(sprite = Sprite::DenseGrass)]
	DenseGrass,
	#[assoc(sprite = Sprite::Shrub)]
	Shrub,
	#[assoc(sprite = Sprite::Bush)]
	Bush,
	#[assoc(sprite = Sprite::Reed)]
	#[assoc(interaction = Interaction::take(&[Item::Reed]))]
	Reed,
	#[assoc(sprite = Sprite::Crop)]
	#[assoc(interaction = Interaction::take(&[]))]
	Crop,
	#[assoc(sprite = Sprite::Flower)]
	#[assoc(interaction = Interaction::take(&[Item::Flower]))]
	Flower,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
	ground: Ground,
	structure: Structure
}

impl Tile {
	pub fn ground(ground: Ground) -> Tile{
		Self{ground, structure: Structure::Air}
	}
	
	pub fn structure(ground: Ground, structure: Structure) -> Tile {
		Self{ground, structure}
	}
	
	pub fn sprites(&self) -> Vec<Sprite> {
		[self.structure.sprite(), self.ground.sprite()].into_iter()
			.filter_map(|s| s)
			.collect()
	}
	
	pub fn blocking(&self) -> bool {
		!self.ground.accessible() || self.structure.blocking()
	}
	
	pub fn interact(&self) -> (Tile, Vec<Item>) {
		if let Some(interaction) = self.structure.interaction() {
			(Self {ground: self.ground, structure: interaction.remains}, interaction.items)
		} else {
			(*self, Vec::new())
		}
	}
}

impl Default for Tile {
	fn default() -> Tile {
		Tile::ground(Ground::Empty)
	}
}

impl Serialize for Tile {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		(self.ground, self.structure).serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for Tile {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let (ground, structure) = <(Ground, Structure)>::deserialize(deserializer)?;
		Ok(Self{ground, structure})
	}
}

