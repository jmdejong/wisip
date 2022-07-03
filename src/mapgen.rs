
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
	Lake,
	Hamlet
}


struct BiomeMap {
	size: Pos,
	seed: u32,
	height: random::Fractal,
	biome_size: i32
}

impl BiomeMap {

	fn new(size: Pos, seed: u32, biome_size: i32) -> Self {
		Self {
			size,
			seed,
			height: random::Fractal::new(seed + 344, vec![(3,0.12), (5,0.20), (7,0.26), (11,0.42)]),
				biome_size: 50
		}
	}

	fn start_biome(&self) -> Pos {
		self.size / 2 / self.biome_size
	}

	fn start_pos(&self) -> Pos {
		self.start_biome() * self.biome_size + Pos::new(self.biome_size / 2, self.biome_size / 2)
	}

	fn biome_at(&self, b_pos: Pos) -> Biome {
		if b_pos == self.start_biome() {
			Biome::Start
		} else {
			*random::pick_weighted(
				random::WhiteNoise::new(self.seed+333).gen(b_pos),
				&[
					(Biome::Forest, 10),
					(Biome::Field, 10),
					(Biome::Hamlet, 10),
					(Biome::Lake, 2)
				]
			)
		}
	}

	fn biome_pos(&self, pos: Pos) -> (Pos, Pos) {
		let rind = random::WhiteNoise::new(self.seed+343).gen(pos);
		let edge_size = self.biome_size / 3;
		let offset = Pos::new((rind % edge_size as u32) as i32 - edge_size / 2, ((rind / edge_size as u32) % edge_size as u32) as i32 - edge_size / 2);
		let fuzzy_pos = pos + offset;
		let b_pos = (fuzzy_pos) / self.biome_size;
		let dpos = pos - b_pos * self.biome_size - Pos::new(self.biome_size / 2, self.biome_size / 2);
		(b_pos, dpos)
	}


	fn tile(&self, pos: Pos) -> Tile {
		let (bpos, dpos) = self.biome_pos(pos);
		let biome = self.biome_at(bpos);
		let rind = random::WhiteNoise::new(self.seed + 7943).gen(pos);
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
				let lake_size_factor = random::WhiteNoise::new(self.seed+276).gen_f(bpos);
				if d_center + 0.0 * lake_size_factor * lake_size_factor * lake_size_factor < self.height.gen_f(pos) {
					Tile::ground(Ground::Water)
				} else {
					*random::pick_weighted(rind, &[
						(Tile::ground(Ground::Grass1), 10),
						(Tile::ground(Ground::Grass2), 10),
						(Tile::ground(Ground::Grass3), 10),
						(Tile::structure(Ground::Grass1, Structure::DenseGrass), 10),
						(Tile::structure(Ground::Grass1, Structure::Shrub), 2)
					])
				}
			}
			Biome::Hamlet => {
				let brind = random::WhiteNoise::new(self.seed+863).gen(bpos);
				let village_width = self.biome_size * 2 / 3;
				let twidth = village_width / 3;
				let vpos = (dpos + Pos::new(village_width, village_width) / 2) * 3 / village_width;
				if  dpos.x.abs() < village_width / 2 && dpos.y.abs() < village_width / 2 {
					let ind: i32 = vpos.x + 3 * vpos.y;
					let trind = random::randomize_u32(brind + ind as u32);
					let tpos = dpos - (vpos - Pos::new(1,1)) * twidth;
					let tmax = tpos.abs().max();
					let di = (tpos.x > tpos.y) as u32 + 2 * (tpos.x.abs() < tpos.y.abs()) as u32;
					let wd = twidth / 2 - 1 - ((trind as i32) >> (4 + di) & 1 );
					if tmax == wd && trind & 3 == 1 {
						let side = vpos.directions_to(tpos)[0];
// 						if side == Direction::DIRECTIONS[trind as usize >> 3 & 3] /*&& tpos.abs().min() == 0*/ {
						if di == trind >> 2 & 3 && tpos.abs().min() == 0 {
							Tile::ground(Ground::Dirt)
						} else {
							Tile::structure(Ground::Dirt, Structure::Wall)
						}
					} else if tmax < wd && trind & 3 == 1 {
						Tile::ground(Ground::Dirt)
					} else if tmax < wd && trind & 3 == 2 {
						Tile::structure(Ground::Dirt, Structure::Crop)
					} else {
						*random::pick_weighted(rind, &[
							(Tile::ground(Ground::Grass1), 10),
							(Tile::ground(Ground::Grass2), 10),
							(Tile::ground(Ground::Dirt), 20)
						])
					}
// 					[Tile::ground(Ground::Stone), Tile::ground(Ground::Empty), Tile::ground(Ground::Water), Tile::ground(Ground::Dirt)][(ind & 3) as usize]
// 				let dcenter = dpos.abs();
// 				if dcenter.x >= 3 && dcenter.x < 10 && dcenter.y >= 3 && dcenter.y < 10 {
// 					Tile::structure(Ground::Dirt, Structure::Wall)
				} else {
					*random::pick_weighted(rind, &[
						(Tile::ground(Ground::Grass1), 10),
						(Tile::ground(Ground::Grass2), 10),
						(Tile::ground(Ground::Grass3), 10)
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
			let floor = biomes.tile(pos);
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


