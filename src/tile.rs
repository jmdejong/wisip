
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
#[func(fn has_water(&self) -> bool {false})]
#[func(fn clear(&self) -> Option<Ground>)]
pub enum Ground {
	#[assoc(sprite = Sprite::Dirt)]
	Dirt,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Grass1)]
	Grass1,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Grass2)]
	Grass2,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Grass3)]
	Grass3,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Moss)]
	Moss,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::DeadLeaves)]
	DeadLeaves,
	
	#[assoc(sprite = Sprite::Sanctuary)]
	Sanctuary,
	
	#[assoc(sprite = Sprite::Water)]
	#[assoc(has_water = true)]
	#[assoc(accessible = false)]
	Water,
	
	#[assoc(sprite = Sprite::StoneFloor)]
	RockFloor,
	
	#[assoc(sprite = Sprite::StoneFloor)]
	StoneFloor,
	
	#[assoc(sprite = Sprite::WoodFloor)]
	WoodFloor,
	
	#[assoc(accessible = false)]
	Empty
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Option<Sprite>)]
#[func(fn blocking(&self) -> bool {false})]
#[func(fn open(&self) -> bool {false})]
#[func(fn interactions(&self) -> Vec<Interactable> {Vec::new()})]
#[func(fn take(&self) -> Option<Item>)]
pub enum Structure {
	#[assoc(open = true)]
	Air,
	#[assoc(sprite = Sprite::Wall)]
	#[assoc(blocking = true)]
	Wall,
	#[assoc(sprite = Sprite::WoodWall)]
	#[assoc(blocking = true)]
	WoodWall,
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
	#[assoc(sprite = Sprite::Heather)]
	Heather,
	#[assoc(sprite = Sprite::Rush)]
	Rush,
	#[assoc(sprite = Sprite::Shrub)]
	Shrub,
	#[assoc(sprite = Sprite::Bush)]
	Bush,
	#[assoc(sprite = Sprite::Reed)]
	#[assoc(interactions = vec![Interactable::harvest(ActionType::Cut, 1, &[0.5, 1.0], &[Item::Reed])])]
	Reed,
	#[assoc(sprite = Sprite::PitcherPlant)]
	#[assoc(interactions = vec![Interactable::harvest(ActionType::Cut, 1, &[0.5, 1.0], &[Item::Pitcher])])]
	PitcherPlant,
	#[assoc(sprite = Sprite::Flower)]
	#[assoc(take = Item::Flower)]
	Flower,
	#[assoc(sprite = Sprite::Pebble)]
	#[assoc(take = Item::Pebble)]
	Pebble,
	#[assoc(sprite = Sprite::Stone)]
	#[assoc(take = Item::Stone)]
	#[assoc(interactions = vec![
		Interactable::new(
			ActionType::Smash,
			1,
			&[0.4, 1.0],
			Some(Structure::Gravel),
			&[Item::SharpStone],
		)
	])]
	Stone,
	#[assoc(sprite = Sprite::Gravel)]
	Gravel,
	#[assoc(sprite = Sprite::Sage)]
	#[assoc(blocking = true)]
	Sage,
}

impl Structure {
	fn interactables(&self) -> Vec<Interactable> {
		let mut interactions = self.interactions();
		if let Some(item) = self.take() {
			interactions.push(Interactable::take(&[item]));
		}
		interactions
	}
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
	
	pub fn interact(&self, item: Item, time: Timestamp) -> Option<InteractionResult> {
		match item.action()? {
			Action::Interact(interact) => 
				self.structure.interactables()
					.into_iter()
					.filter_map(|interactable| interactable.apply(interact, time))
					.next(),
			Action::Fill(full) =>
				if self.ground.has_water() {
					Some(InteractionResult::exchange(full))
				} else {
					None 
				}
			Action::Clear =>
				if self.structure.open() {
					Some(InteractionResult {
						remains_ground: Some(self.ground.clear()?),
						..Default::default()
					})
				} else {
					None
				}
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

