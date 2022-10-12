
use std::collections::HashMap;
use crate::{
	pos::Pos,
	grid::Grid,
	tile::{Tile, Structure},
	basemap::{BaseMap, InfiniteMap},
	timestamp::Timestamp
};

pub struct Ground {
	basemap: InfiniteMap,
	changes: HashMap<Pos, Option<Structure>>
}

impl Ground {
	
	pub fn new() -> Self {
		Self {
			basemap: InfiniteMap::new(9876),
			changes: HashMap::new()
		}
	}
	
	pub fn cell(&mut self, pos: Pos, time: Timestamp) -> Tile {
		self.basemap.cell(pos, time)
	}
	
	pub fn player_spawn(&mut self) -> Pos {
		self.basemap.player_spawn()
	}
}
