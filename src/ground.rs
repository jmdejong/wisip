
use std::collections::{HashMap, HashSet};
use crate::{
	pos::Pos,
	grid::Grid,
	tile::{Tile},
	basemap::{BaseMap, InfiniteMap},
	timestamp::Timestamp
};

pub struct Ground {
	basemap: InfiniteMap,
	changes: HashMap<Pos, Tile>,
	time: Timestamp,
	modifications: HashMap<Pos, Tile>
}

impl Ground {
	
	pub fn new(time: Timestamp) -> Self {
		Self {
			basemap: InfiniteMap::new(9876),
			changes: HashMap::new(),
			time,
			modifications: HashMap::new()
		}
	}
	
	fn base_cell(&mut self, pos: Pos) -> Tile {
		self.basemap.cell(pos, self.time)
	}
	
	pub fn cell(&mut self, pos: Pos) -> Tile {
		self.changes.get(&pos).map(|tile| tile.clone()).unwrap_or_else(|| self.base_cell(pos))
	}
	
	pub fn set(&mut self, pos: Pos, tile: Tile) {
		if tile == self.base_cell(pos) {
			self.changes.remove(&pos);
		} else {
			self.changes.insert(pos, tile);
		}
		self.modifications.insert(pos, tile);
	}
	
	pub fn player_spawn(&mut self) -> Pos {
		self.basemap.player_spawn()
	}
	
	pub fn tick(&mut self, time: Timestamp) {
		self.time = time;
	}
	
	pub fn flush(&mut self) {
		self.modifications.clear()
	}
	
	pub fn modified(&self) -> HashMap<Pos, Tile> {
		self.modifications.clone()
	}
}
