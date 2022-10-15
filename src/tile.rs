
use crate::Sprite;
use enum_properties::enum_properties;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroundProperties {
	sprite: Sprite,
	accessible: bool
}

enum_properties! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Ground: GroundProperties {
		Stone {sprite: Sprite::Stone, accessible: true},
		Dirt {sprite: Sprite::Dirt, accessible: true},
		Grass1 {sprite: Sprite::Grass1, accessible: true},
		Grass2 {sprite: Sprite::Grass2, accessible: true},
		Grass3 {sprite: Sprite::Grass3, accessible: true},
		Sanctuary {sprite: Sprite::Sanctuary, accessible: true},
		Water {sprite: Sprite::Water, accessible: false},
		Empty {sprite: Sprite::Empty, accessible: false},
	}
}

pub struct StructureProperties {
	sprite: Sprite,
	blocking: bool,
	breakable: bool
}

const DEFAULT_STRUCTURE_PROPERTIES: StructureProperties =
	StructureProperties {
		sprite: Sprite::Empty,
		blocking: false,
		breakable: false
	};

enum_properties! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Structure: StructureProperties {
		Air {sprite: Sprite::Empty},
		Wall {sprite: Sprite::Wall, blocking: true},
		Rubble {sprite: Sprite::Rubble, blocking: true},
		Rock {sprite: Sprite::Rock, blocking: true},
		Gate {sprite: Sprite::Gate, blocking: true},
		Tree {sprite: Sprite::Tree, blocking: true},
		DenseGrass {sprite: Sprite::DenseGrass},
		Shrub {sprite: Sprite::Shrub},
		Bush {sprite: Sprite::Bush},
		Reed {sprite: Sprite::Reed, breakable: true},
		Crop {sprite: Sprite::Crop, breakable: true},
		Flower {sprite: Sprite::Flower, breakable: true},
		..DEFAULT_STRUCTURE_PROPERTIES
	}
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
		[self.structure.sprite, self.ground.sprite].into_iter()
			.filter(Sprite::visible)
			.collect()
	}
	
	pub fn blocking(&self) -> bool {
		!self.ground.accessible || self.structure.blocking
	}
	
	pub fn interact(&self) -> Tile {
		Self {
			ground: self.ground,
			structure: 
				if self.structure.breakable {
					Structure::Air
				} else {
					self.structure
				}
		}
	}
}

impl Default for Tile {
	fn default() -> Tile {
		Tile::ground(Ground::Empty)
	}
}

