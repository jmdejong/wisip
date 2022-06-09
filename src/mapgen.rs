
use std::str::FromStr;
use serde::{Serialize, de, Deserialize, Deserializer};
use crate::{
	Pos,
	Direction,
	tile::{Tile, Ground, Structure},
	errors::AnyError,
	aerr,
	grid::Grid,
	pos::Distance,
	random
};



#[derive(Debug, Clone)]
pub struct MapTemplate {
	pub size: Pos,
	pub ground: Grid<Tile>,
	pub spawnpoint: Pos,
	pub monsterspawn: Vec<Pos>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinMap{
	Square
}

impl FromStr for BuiltinMap {
	type Err = AnyError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"square" => Ok(Self::Square),
			_ => Err(aerr!("'{}' is not a valid map", s))
		}
	}
}

#[derive(Debug, Clone)]
pub enum MapType {
	Builtin(BuiltinMap),
	Custom(MapTemplate)
}

pub fn create_map(typ: &MapType) -> MapTemplate {
	match typ {
		MapType::Builtin(BuiltinMap::Square) => create_rectangular_map(999, 1024, 1024),
		MapType::Custom(template) => template.clone()
	}
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Biome {
	Start,
	Forest,
	Field,
	Lake
}


struct BiomeMap {
	size: Pos,
	noise: random::WhiteNoise,
	height: random::Fractal,
	biome_size: i32
}

impl BiomeMap {

	fn new(size: Pos, seed: u32, biome_size: i32) -> Self {
		Self {
			size,
			noise: random::WhiteNoise::new(seed + 333),
			height: random::Fractal::new(seed + 344, vec![(3,0.12), (5,0.20), (7,0.26), (11,0.42)]),
			biome_size: 48
		}
	}

	fn start_biome(&self) -> Pos {
		self.size / 2 / self.biome_size
	}

	fn start_pos(&self) -> Pos{
		self.start_biome() * self.biome_size + Pos::new(self.biome_size / 2, self.biome_size / 2)
	}

	fn biome_at(&self, pos: Pos) -> (Biome, Pos) {
		let rind = self.noise.gen(pos);
		let edge_size = self.biome_size / 3;
		let offset = Pos::new((rind % edge_size as u32) as i32 - edge_size / 2, ((rind / edge_size as u32) % edge_size as u32) as i32 - edge_size / 2);
		let b_pos = (pos + offset) / self.biome_size;
		let biome = if b_pos == self.start_biome() {
			Biome::Start
		} else {
			*random::pick_weighted(self.noise.gen(b_pos), &[(Biome::Forest, 10), (Biome::Field, 10), (Biome::Lake, 12)])
		};
		let dpos = pos - b_pos * self.biome_size - Pos::new(self.biome_size / 2, self.biome_size / 2);
		(biome, dpos)
	}

	fn tile(&self, biome: Biome, dpos: Pos, rind: u32) -> Tile {
		match biome {
			Biome::Start => {
				let dspawn = dpos.abs();
				if dspawn.x <= 4 && dspawn.y <= 4 {
					if dspawn.x + dspawn.y <= 5 {
						Tile::ground(Ground::Sanctuary)
					} else {
						Tile::structure(Ground::Dirt, Structure::Wall)
					}
				} else if dspawn.x <= 1 || dspawn.y <= 1 {
					Tile::ground(Ground::Dirt)
				} else {
					*random::pick(rind, &[
						Tile::ground(Ground::Grass1),
						Tile::ground(Ground::Grass2),
						Tile::ground(Ground::Grass3)
					])
				}
			}
			Biome::Field =>
				*random::pick_weighted(rind, &[
					(Tile::ground(Ground::Grass1), 10),
					(Tile::ground(Ground::Grass2), 10),
					(Tile::ground(Ground::Grass3), 10),
					(Tile::structure(Ground::Grass1, Structure::DenseGrass), 10),
					(Tile::structure(Ground::Grass1, Structure::Shrub), 1)
				]),
			Biome::Forest =>
				*random::pick_weighted(rind, &[
					(Tile::ground(Ground::Grass1), 10),
					(Tile::ground(Ground::Grass2), 10),
					(Tile::ground(Ground::Grass3), 10),
					(Tile::ground(Ground::Dirt), 20),
					(Tile::structure(Ground::Dirt, Structure::Tree), 7)
				]),
			Biome::Lake => {
				let d_center = ((dpos.x * dpos.x + dpos.y * dpos.y) as f32).sqrt() / (self.biome_size as f32 * 0.5);
				if d_center < 1.0 {
					Tile::ground(Ground::Water)
				} else if rind % 64 == 0 {
					Tile::structure(Ground::Grass1, Structure::Shrub)
				} else {
					*random::pick_weighted(rind, &[
						(Tile::ground(Ground::Grass1), 10),
						(Tile::ground(Ground::Grass2), 10),
						(Tile::ground(Ground::Grass3), 10),
						(Tile::structure(Ground::Grass1, Structure::DenseGrass), 10),
						(Tile::structure(Ground::Grass1, Structure::Shrub), 1)
					])
				}
			}
		}
	}
}


fn create_rectangular_map(seed: u32, width: i32, height: i32) -> MapTemplate {
	let size = Pos::new(width, height);
	let biomes = BiomeMap::new(size, seed, 48);
	let mut map = MapTemplate {
		size,
		ground: Grid::new(size, Tile::ground(Ground::Dirt)),
		spawnpoint: biomes.start_pos(),
		monsterspawn: vec![Pos::new(0,0), Pos::new(size.x - 1, 0), Pos::new(0, size.y - 1), Pos::new(size.x - 1, size.y - 1)],
	};

	for x in 0..map.size.x {
		for y in 0..map.size.y {
			let pos = Pos::new(x, y);
			let (biome, dpos) = biomes.biome_at(pos);
			let rind = random::WhiteNoise::new(seed + 7943).gen(pos);
			let floor = biomes.tile(biome, dpos, rind);
			map.ground.set(pos, floor);
		}
	}
	
	map
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MapTemplateSave {
	pub size: Pos,
	pub ground: Vec<String>,
	pub spawnpoint: Pos,
	pub monsterspawn: Vec<Pos>,
}

impl<'de> Deserialize<'de> for MapTemplate {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let MapTemplateSave{size, ground, spawnpoint, monsterspawn} =
			MapTemplateSave::deserialize(deserializer)?;
		let mut groundmap = Grid::new(size, Tile::ground(Ground::Dirt));
		for (y, line) in ground.iter().enumerate(){
			for (x, c) in line.chars().enumerate(){
				let tile = Tile::from_char(c).ok_or_else(||de::Error::custom(format!("Invalid tile character '{}'", c)))?;
				groundmap.set(Pos::new(x as i32, y as i32), tile);
			}
		}
		Ok(MapTemplate {
			size,
			spawnpoint,
			monsterspawn,
			ground: groundmap
		})
	}
}


