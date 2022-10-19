

use crate::{
	pos::{Pos, Area},
	timestamp::Timestamp,
	tile::{Tile, Ground, Structure},
	grid::Grid,
	random,
	randomtick
};

pub trait BaseMap {
	fn cell(&mut self, pos: Pos, time: Timestamp) -> Tile;
	
	#[allow(dead_code)]
	fn region(&mut self, area: Area, time: Timestamp) -> Grid<Tile> {
		let mut grid = Grid::with_offset(area.size(), area.min(), Tile::ground(Ground::Dirt));
		for pos in area.iter() {
			grid.set(pos, self.cell(pos, time));
		}
		grid
	}
	
	fn player_spawn(&mut self) -> Pos;
}

pub struct InfiniteMap {
	seed: u32,
	height: random::Fractal,
	biome_size: i32
}

impl InfiniteMap {
	pub fn new(seed: u32) -> Self {
		Self {
			seed,
			height: random::Fractal::new(seed + 344, vec![(3,0.12), (5,0.20), (7,0.26), (11,0.42)]),
			biome_size: 48
		}
	}
	
	
	fn start_biome(&self) -> BPos {
		BPos(Pos::new(0, 0))
	}

	fn start_pos(&self) -> Pos {
		self.biome_core(self.start_biome())
	}

	fn biome_at(&self, b_pos: BPos) -> Biome {
		if b_pos == self.start_biome() {
			Biome::Start
		} else {
			*random::pick_weighted(
				random::WhiteNoise::new(self.seed+333).gen(b_pos.0),
				&[
					(Biome::Field, 11),
					(Biome::Forest, 14),
					// (Biome::Hamlet, 5),
					(Biome::Rocks, 5),
					(Biome::Lake, 5),
				]
			)
		}
	}
	
	fn biome_core(&self, bpos: BPos) -> Pos {
		let rind = random::WhiteNoise::new(self.seed+821).gen(bpos.0);
		let core_size = self.biome_size / 2;
		let core_offset = Pos::new(
			(rind % core_size as u32) as i32 - core_size / 2,
			((rind / core_size as u32) % core_size as u32) as i32 - core_size / 2
		);
		bpos.0 * self.biome_size + core_offset
	}
	
	fn closest_biome_pos(&self, pos: Pos) -> BPos {
		let bpos = BPos(pos / self.biome_size);
		[(0, 0), (1, 0), (0, 1), (1, 1)].into_iter()
			.map(|p| BPos(bpos.0 + p))
			.min_by_key(|b| pos.distance_to(self.biome_core(*b)))
			.unwrap()
	}
	
	fn edge_distance(&self, pos: Pos) -> i32 {
		let bpos = BPos(pos / self.biome_size);
		let mut distances: Vec<(i32, Biome)> = [(0, 0), (1, 0), (0, 1), (1, 1)].into_iter()
			.map(|p| {
				let b = BPos(bpos.0 + p);
				(pos.distance_to(self.biome_core(b)), self.biome_at(b))
			})
			.collect();
		distances.sort_by_key(|(d, _)| *d);
		let (dist, my_biome) = distances[0];
		distances[1..].iter()
			.filter(|(_, biome)| *biome != my_biome)
			.next()
			.map(|(d, _)| d - dist)
			.unwrap_or(self.biome_size / 2)
	}

	fn biome_pos(&self, pos: Pos) -> (BPos, Pos) {
		let rind = random::WhiteNoise::new(self.seed+343).gen(pos);
		let edge_size = self.biome_size / 4;
		let mut offset = Pos::new((rind % edge_size as u32) as i32 - edge_size / 2, ((rind / edge_size as u32) % edge_size as u32) as i32 - edge_size / 2);
		if offset.size() > edge_size / 2 {
			offset = offset % edge_size - Pos::new(edge_size/2, edge_size/2);
		}
		let fuzzy_pos = pos + offset;
		let b_pos = self.closest_biome_pos(fuzzy_pos);
		let dpos = pos - self.biome_core(b_pos);
		(b_pos, dpos)
	}


	fn tile(&self, pos: Pos, time: Timestamp) -> Tile {
		let (bpos, dpos) = self.biome_pos(pos);
		let biome = self.biome_at(bpos);
		let rind = random::WhiteNoise::new(self.seed + 7943).gen(pos);
		let rtime = randomtick::tick_num(pos, time) as u32 + random::WhiteNoise::new(self.seed + 356).gen(pos);
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
			Biome::Field => {
				*random::pick_weighted(rind, &[
					(Tile::ground(Ground::Grass1), 10),
					(Tile::ground(Ground::Grass2), 10),
					(Tile::ground(Ground::Grass3), 10),
					(Tile::structure(Ground::Grass1, Structure::DenseGrass), 10),
					(Tile::structure(Ground::Grass1, Structure::Shrub), 1),
					(
						if rtime.rem_euclid(2) == 0 {
							Tile::structure(Ground::Grass1, Structure::Flower)
						} else {
							Tile::ground(Ground::Grass1)
						},
						2
					)
				])
			}
			Biome::Forest => {
				*random::pick_weighted(rtime, &[
					(*random::pick(rind, &[
						Tile::ground(Ground::Grass1),
						Tile::ground(Ground::Grass2),
						Tile::ground(Ground::Grass3),
						Tile::ground(Ground::Dirt),
						Tile::ground(Ground::Dirt),
					]), 100),
					(Tile::structure(Ground::Grass1, Structure::Sapling), 3),
					(Tile::structure(Ground::Dirt, Structure::YoungTree), 4),
					(Tile::structure(Ground::Dirt, Structure::Tree), 13),
					(Tile::structure(Ground::Dirt, Structure::OldTree), 1),
					(Tile::ground(Ground::Dirt), 1)
				])
			}
			Biome::Lake => {
				let d_center = ((dpos.x * dpos.x + dpos.y * dpos.y) as f32).sqrt() / (self.biome_size as f32 * 0.5);
				let reed_density = random::Fractal::new(self.seed+276, vec![(7, 0.5), (11, 0.5)]).gen_f(pos) * 0.4 - 0.2;
				let height = d_center - self.height.gen_f(pos);
				if height.abs() < reed_density {
					Tile::structure(
						if height > 0.0 { Ground::Dirt } else { Ground::Water },
						if randomtick::tick_num(pos, time).rem_euclid(4) as u32 != rind.rem_euclid(4) {
							Structure::Reed
						} else {
							Structure::Air
						}
					)
				} else if height < 0.0 {
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
			Biome::Rocks => {
				let min_height = 0.6;
				let height = self.rock_height(pos);
				if height > min_height {
					let ismid = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)]
						.into_iter()
						.all(|d| self.rock_height(pos + d) > min_height);
					Tile::structure(
						Ground::Stone,
						if ismid {
							Structure::RockMid
						} else {
							Structure::Rock
						}
					)
				} else {
					*random::pick_weighted((height * 100.0) as u32, &[
						(*random::pick_weighted(rind, &[
							(Tile::ground(Ground::Grass2), 10),
							(Tile::ground(Ground::Grass3), 10),
							(Tile::ground(Ground::Dirt), 1),
							(Tile::ground(Ground::Stone), (height * 10.0) as u32),
						]), 50),
						(*random::pick_weighted(rind, &[
							(Tile::ground(Ground::Grass2), 1),
							(Tile::ground(Ground::Grass3), 1),
							(*random::pick_weighted(rtime, &[
								(Tile::structure(Ground::Stone, Structure::Gravel), 20),
								(Tile::ground(Ground::Stone), 50),
								(Tile::structure(Ground::Stone, Structure::Stone), 3),
								(Tile::structure(Ground::Stone, Structure::Gravel), 20),
								(Tile::ground(Ground::Stone), 50),
								(Tile::structure(Ground::Stone, Structure::Pebble), 5),
								(Tile::ground(Ground::Stone), 50),
							]), 3),
						]), 50),
					])
				}
			}
			Biome::Hamlet => {
				let brind = random::WhiteNoise::new(self.seed+863).gen(bpos.0);
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
	
	fn rock_height(&self, pos: Pos) -> f32 {
	
		let c = ((self.edge_distance(pos) - self.biome_size / 4) as f32 / 4.0).clamp(0.0, 1.0);
		self.height.gen_f(pos) * c
	}
}

impl BaseMap for InfiniteMap {
	fn cell(&mut self, pos: Pos, time: Timestamp) -> Tile {
		self.tile(pos, time)
	}
	
	
	fn player_spawn(&mut self) -> Pos {
		self.start_pos()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Biome {
	Start,
	Forest,
	Field,
	Lake,
	#[allow(dead_code)]
	Hamlet,
	Rocks
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
struct BPos(Pos);



#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn core_is_in_own_biome() {
		let map = InfiniteMap::new(678);
		let p: Vec<BPos> = [
			(3, 4),
			(0, 0),
			(-4, 2),
			(-5, -3),
			(1, 0),
			(0, -9)
		].into_iter()
			.map(|(x, y)| BPos(Pos::new(x, y)))
			.collect();
		for bpos in p {
			assert_eq!(bpos, map.closest_biome_pos(map.biome_core(bpos)));
			assert_eq!((bpos, Pos::new(0, 0)), map.biome_pos(map.biome_core(bpos)));
		}
	}
	
	#[test]
	fn start_is_start_biome() {
		let map = InfiniteMap::new(9876);
		assert_eq!(map.biome_at(map.biome_pos(map.start_pos()).0), Biome::Start);
	}
	
	#[test]
	fn start_pos_has_sanctuary() {
		let map = InfiniteMap::new(9876);
		assert_eq!(map.tile(map.start_pos(), Timestamp(1)), Tile::ground(Ground::Sanctuary));
	}
}

