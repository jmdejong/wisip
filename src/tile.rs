
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

impl Default for StructureProperties {
	fn default() -> Self {
		Self {
			sprite: Sprite::Empty,
			blocking: false,
			breakable: false
		}
	}
}

enum_properties! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Structure: StructureProperties {
		Air {sprite: Sprite::Empty, blocking: false, breakable: false},
		Wall {sprite: Sprite::Wall, blocking: true, breakable: false},
		Rubble {sprite: Sprite::Rubble, blocking: true, breakable: false},
		Rock {sprite: Sprite::Rock, blocking: true, breakable: false},
		Gate {sprite: Sprite::Gate, blocking: true, breakable: false},
		Tree {sprite: Sprite::Tree, blocking: true, breakable: false},
		DenseGrass {sprite: Sprite::DenseGrass, blocking: false, breakable: false},
		Shrub {sprite: Sprite::Shrub, blocking: false, breakable: false},
		Bush {sprite: Sprite::Bush, blocking: false, breakable: false},
		Reed {sprite: Sprite::Reed, blocking: false, breakable: true},
		Crop {sprite: Sprite::Crop, blocking: false, breakable: false},
		Flower {sprite: Sprite::Flower, blocking: false, breakable: true},
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

