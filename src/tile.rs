
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

enum_properties! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Structure: StructureProperties {
		Wall {sprite: Sprite::Wall, blocking: true, breakable: false},
		Rubble {sprite: Sprite::Rubble, blocking: true, breakable: false},
		Rock {sprite: Sprite::Rock, blocking: true, breakable: false},
		Gate {sprite: Sprite::Gate, blocking: true, breakable: false},
		Tree {sprite: Sprite::Tree, blocking: true, breakable: false},
		DenseGrass {sprite: Sprite::DenseGrass, blocking: false, breakable: false},
		Shrub {sprite: Sprite::Shrub, blocking: false, breakable: false},
		Bush {sprite: Sprite::Bush, blocking: false, breakable: false},
		Crop {sprite: Sprite::Crop, blocking: false, breakable: false},
		Flower {sprite: Sprite::Flower, blocking: false, breakable: true},
	}
}

use Ground::*;
use Structure::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
	ground: Ground,
	structure: Option<Structure>
}

impl Tile {
	pub fn ground(ground: Ground) -> Tile{
		Self{ground, structure: None}
	}
	
	pub fn structure(ground: Ground, structure: Structure) -> Tile {
		Self{ground, structure: Some(structure)}
	}
	
	pub fn sprites(&self) -> Vec<Sprite> {
		let mut sprites = Vec::new();
		if let Some(structure) = self.structure {
			sprites.push(structure.sprite);
		}
		sprites.push(self.ground.sprite);
		sprites
	}
	
	pub fn blocking(&self) -> bool {
		!self.ground.accessible || self.structure.map_or(false, |structure| structure.blocking)
	}
	
	pub fn interact(&self) -> Tile {
		Self {
			ground: self.ground,
			structure: self.structure.and_then(|structure|
				if structure.breakable {
					None
				} else {
					Some(structure)
				}
			)
		}
	}
	
	pub fn from_char(c: char) -> Option<Self>{
		Some(match c {
			'"' => Self::ground(Stone),
			'.' => Self::ground(Dirt),
			',' => Self::ground(Grass1),
			'\'' => Self::ground(Grass2),
			'`' => Self::ground(Grass3),
			'=' => Self::structure(Stone, Gate),
			'+' => Self::ground(Sanctuary),
			'#' => Self::structure(Stone, Wall),
			'X' => Self::structure(Stone, Rock),
			'R' => Self::structure(Stone, Rubble),
			'~' => Self::ground(Water),
			_ => {return None}
		})
	}
}

impl Default for Tile {
	fn default() -> Tile {
		Tile::ground(Empty)
	}
}

