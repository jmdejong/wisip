
use crate::Sprite;
use enum_properties::enum_properties;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GroundProperties {
	sprite: Sprite,
	accessible: bool
}

enum_properties! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	enum Ground: GroundProperties {
		Stone {sprite: Sprite::Stone, accessible: true},
		Dirt {sprite: Sprite::DIRT, accessible: true},
		Grass1 {sprite: Sprite::Grass1, accessible: true},
		Grass2 {sprite: Sprite::Grass2, accessible: true},
		Grass3 {sprite: Sprite::Grass3, accessible: true},
		Sanctuary {sprite: Sprite::Sanctuary, accessible: true},
		Water {sprite: Sprite::Water, accessible: false},
		Empty {sprite: Sprite::Empty, accessible: false},
	}
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Ground{
// 	Stone,
// 	Dirt,
// 	Grass1,
// 	Grass2,
// 	Grass3,
// 	Sanctuary,
// 	Water,
// 	Empty
// }
// 
// impl Deref for Ground {
// 	type Target = GroundProperties;
// 	
// 	fn deref(&self) -> &GroundProperties {
// 		match self {
// 			Ground::Stone => &GroundProperties{sprite: Sprite::Stone, accessible: true},
// 			Ground::Dirt => &GroundProperties{sprite: Sprite::Dirt, accessible: true},
// 			Ground::Grass1 => &GroundProperties{sprite: Sprite::Grass1, accessible: true},
// 			Ground::Grass2 => &GroundProperties{sprite: Sprite::Grass2, accessible: true},
// 			Ground::Grass3 => &GroundProperties{sprite: Sprite::Grass3, accessible: true},
// 			Ground::Sanctuary => &GroundProperties{sprite: Sprite::Sanctuary, accessible: true},
// 			Ground::Water => &GroundProperties{sprite: Sprite::Water, accessible: false},
// 			Ground::Empty => &GroundProperties{sprite: Sprite::Empty, accessible: false}
// 		}
// 	}
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Structure {
	Wall,
	Rubble,
	Rock,
	Gate,
	S1,
	S2,
	S3,
	S4,
	S5,
	S6,
	S7
}


impl Structure {
	fn sprite(&self) -> Sprite {
		Sprite::new(match self {
			Structure::Wall => "wall",
			Structure::Gate => "gate",
			Structure::Rubble => "rubble",
			Structure::Rock => "rock",
			_ => " "
		})
	}
	
	fn blocking(&self) -> bool {
		true
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
	pub fn sprites(&self) -> Vec<Sprite> {
		let mut sprites = Vec::new();
		if let Some(structure) = self.structure {
			sprites.push(structure.sprite());
		}
		sprites.push(self.ground.sprite);
		sprites
	}
	
	pub fn blocking(&self) -> bool {
		!self.ground.accessible || self.structure.map_or(false, |structure| structure.blocking())
	}
	
	pub fn from_char(c: char) -> Option<Self>{
		Some(match c {
			'"' => Self{ground: Stone, structure: None},
			'.' => Self{ground: Dirt, structure: None},
			',' => Self{ground: Grass1, structure: None},
			'\'' => Self{ground: Grass2, structure: None},
			'`' => Self{ground: Grass3, structure: None},
			'=' => Self{ground: Stone, structure: Some(Gate)},
			'+' => Self{ground: Sanctuary, structure: None},
			'#' => Self{ground: Stone, structure: Some(Wall)},
			'X' => Self{ground: Stone, structure: Some(Rock)},
			'R' => Self{ground: Stone, structure: Some(Rubble)},
			'~' => Self{ground: Water, structure: None},
			_ => {return None}
		})
	}
	
	pub const Sanctuary: Tile = Self{ground: Sanctuary, structure: None};
	pub const Water: Tile = Self{ground: Water, structure: None};
	pub const Dirt: Tile = Self{ground: Dirt, structure: None};
	pub const Grass1: Tile = Self{ground: Grass1, structure: None};
	pub const Grass2: Tile = Self{ground: Grass2, structure: None};
	pub const Grass3: Tile = Self{ground: Grass3, structure: None};
	pub const Wall: Tile = Self{ground: Stone, structure: Some(Wall)};
	
}

