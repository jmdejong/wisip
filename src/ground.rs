
use std::collections::{HashMap, HashSet};
use crate::{
	pos::{Pos, Area},
	tile,
	tile::{Tile, Structure},
	basemap::{BaseMap, InfiniteMap},
	timestamp::{Timestamp, Duration},
	randomtick
};

const SEED: u32 = 9876;

pub struct Ground {
	basemap: InfiniteMap,
	changes: HashMap<Pos, (Tile, Timestamp)>,
	time: Timestamp,
	modifications: HashMap<Pos, Tile>
}

impl Ground {
	
	pub fn new(time: Timestamp) -> Self {
		Self {
			basemap: InfiniteMap::new(SEED),
			changes: HashMap::new(),
			time,
			modifications: HashMap::new()
		}
	}
	
	fn base_cell(&mut self, pos: Pos) -> Tile {
		self.basemap.cell(pos, self.time)
	}
	
	pub fn cell(&mut self, pos: Pos) -> Tile {
		self.changes.get(&pos).map(|change| change.0).unwrap_or_else(|| self.base_cell(pos))
	}
	
	pub fn set(&mut self, pos: Pos, tile: Tile) {
		if tile == self.base_cell(pos) {
			self.changes.remove(&pos);
		} else {
			self.changes.insert(pos, (tile, self.time));
		}
		self.modifications.insert(pos, tile);
	}
	
	pub fn set_structure(&mut self, pos: Pos, structure: Structure) {
		let new_tile = Tile::structure(self.cell(pos).ground, structure) ;
		self.set(pos, new_tile )
	}
	
	pub fn set_ground(&mut self, pos: Pos, ground: tile::Ground) {
		let new_tile = Tile::structure(ground, self.cell(pos).structure);
		self.set(pos, new_tile )
	}
	
	pub fn player_spawn(&mut self) -> Pos {
		self.basemap.player_spawn()
	}
	
	pub fn tick(&mut self, time: Timestamp, areas: Vec<Area>) {
		let chunk_size = randomtick::CHUNK_SIZE;
		let tick_pos = randomtick::tick_position(time);
		let tick_positions = areas.iter()
			.flat_map(|area| {
				let chunk_min = area.min() / chunk_size;
				let chunk_max = (area.max() - Pos::new(1, 1) / chunk_size) + Pos::new(1, 1);
				let chunk_area = Area::new(chunk_min, chunk_max - chunk_min);
				chunk_area.iter()
					.map(|chunk_pos| chunk_pos * chunk_size + tick_pos)
					.filter(|pos| area.contains(*pos))
			})
			.collect::<HashSet<Pos>>();
		self.time = time;
		let last_tick = time - Duration((chunk_size * chunk_size) as i64);
		for pos in tick_positions {
			if self.basemap.cell(pos, last_tick) != self.basemap.cell(pos, time) {
				self.modifications.insert(pos, self.basemap.cell(pos, time));
			}
		}
	}
	
	pub fn flush(&mut self) {
		self.modifications.clear()
	}
	
	pub fn modified(&self) -> HashMap<Pos, Tile> {
		self.modifications.clone()
	}
	
	pub fn save(&self) -> GroundSave {
		self.changes.clone().into_iter().collect()
	}
	
	pub fn load(changes: GroundSave, time: Timestamp) -> Self {
		Self {
			basemap: InfiniteMap::new(SEED),
			changes: changes.into_iter().collect(),
			time,
			modifications: HashMap::new()
		}
	}
}

pub type GroundSave = Vec<(Pos, (Tile, Timestamp))>;

