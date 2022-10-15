
use crate::Sprite;
use enum_assoc::Assoc;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc)]
#[func(fn sprite(&self) -> Sprite)]
#[func(fn accessible(&self) -> bool {true})]
pub enum Ground {
	#[assoc(sprite = Sprite::Stone)]
	Stone,
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
	#[assoc(sprite = Sprite::Empty)]
	#[assoc(accessible = false)]
	Empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc)]
#[func(fn sprite(&self) -> Sprite)]
#[func(fn blocking(&self) -> bool {false})]
#[func(fn breakable(&self) -> bool {false})]
pub enum Structure {
	#[assoc(sprite = Sprite::Empty)]
	Air,
	#[assoc(sprite = Sprite::Wall)]
	#[assoc(blocking = true)]
	Wall,
	#[assoc(sprite = Sprite::Rubble)]
	#[assoc(blocking = true)]
	Rubble,
	#[assoc(sprite = Sprite::Rock)]
	#[assoc(blocking = true)]
	Rock,
	#[assoc(sprite = Sprite::Gate)]
	#[assoc(blocking = true)]
	Gate,
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
	#[assoc(breakable = true)]
	Reed,
	#[assoc(sprite = Sprite::Crop)]
	#[assoc(breakable = true)]
	Crop,
	#[assoc(sprite = Sprite::Flower)]
	#[assoc(breakable = true)]
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
			.filter(Sprite::visible)
			.collect()
	}
	
	pub fn blocking(&self) -> bool {
		!self.ground.accessible() || self.structure.blocking()
	}
	
	pub fn interact(&self) -> Tile {
		Self {
			ground: self.ground,
			structure: 
				if self.structure.breakable() {
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

