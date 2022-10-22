
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use enum_assoc::Assoc;
use crate::{
	sprite::Sprite,
	inventory::Item,
	action::{Action, ActionType, Interactable, InteractionResult},
	timestamp::Timestamp
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
	#[assoc(sprite = Sprite::StoneFloor)]
	Stone,
	#[assoc(accessible = false)]
	Empty
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Option<Sprite>)]
#[func(fn blocking(&self) -> bool {false})]
#[func(fn interactions(&self) -> Vec<Interactable> {Vec::new()})]
pub enum Structure {
	Air,
	#[assoc(sprite = Sprite::Wall)]
	#[assoc(blocking = true)]
	Wall,
	#[assoc(sprite = Sprite::Rock)]
	#[assoc(blocking = true)]
	Rock,
	#[assoc(sprite = Sprite::RockMid)]
	#[assoc(blocking = true)]
	RockMid,
	#[assoc(sprite = Sprite::Sapling)]
	Sapling,
	#[assoc(sprite = Sprite::YoungTree)]
	#[assoc(blocking = true)]
	YoungTree,
	#[assoc(sprite = Sprite::Tree)]
	#[assoc(blocking = true)]
	Tree,
	#[assoc(sprite = Sprite::OldTree)]
	#[assoc(blocking = true)]
	OldTree,
	#[assoc(sprite = Sprite::DenseGrass)]
	DenseGrass,
	#[assoc(sprite = Sprite::Shrub)]
	Shrub,
	#[assoc(sprite = Sprite::Bush)]
	Bush,
	#[assoc(sprite = Sprite::Reed)]
	#[assoc(interactions = vec![Interactable::new(ActionType::Cut, 1, &[0.5, 1.0], Structure::Air, &[Item::Reed])])]
	Reed,
	#[assoc(sprite = Sprite::PitcherPlant)]
	#[assoc(interactions = vec![Interactable::new(ActionType::Cut, 1, &[0.5, 1.0], Structure::Air, &[Item::Reed])])]
	PitcherPlant,
	#[assoc(sprite = Sprite::Crop)]
	#[assoc(interactions = vec![Interactable::take(&[])])]
	Crop,
	#[assoc(sprite = Sprite::Flower)]
	#[assoc(interactions = vec![Interactable::take(&[Item::Flower])])]
	Flower,
	#[assoc(sprite = Sprite::Pebble)]
	#[assoc(interactions = vec![Interactable::take(&[Item::Pebble])])]
	Pebble,
	#[assoc(sprite = Sprite::Stone)]
	#[assoc(interactions = vec![
		Interactable::take(&[Item::Stone]),
		Interactable::new(
			ActionType::Smash,
			1,
			&[0.4, 1.0],
			Structure::Gravel,
			&[Item::SharpStone],
		)
	])]
	Stone,
	#[assoc(sprite = Sprite::Gravel)]
	Gravel,
	
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
	pub ground: Ground,
	pub structure: Structure
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
	
	pub fn interact(&self, action: Action, time: Timestamp) -> Option<InteractionResult> {
		self.structure.interactions()
			.into_iter()
			.filter_map(|interactable| interactable.apply(action, time))
			.next()
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

