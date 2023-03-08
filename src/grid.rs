
use crate::pos::{Pos, Area};

#[derive(Debug, Clone)]
pub struct Grid<T> {
	pub area: Area,
	storage: Vec<T>
}

#[allow(dead_code)]
impl<T: Clone> Grid<T> {
	
	pub fn new(area: Area, filler: T) -> Grid<T> {
		let surface = area.surface() as usize;
		let mut storage = Vec::with_capacity(surface);
		storage.resize(surface, filler);
		Self {area, storage}
	}
	
	#[inline]
	fn index(&self, global_pos: Pos) -> Option<usize> {
		if self.area.contains(global_pos) {
			let pos = global_pos - self.area.min();
			Some((pos.x + self.area.size().x * pos.y) as usize)
		} else {
			None
		}
	}
	
	#[inline]
	pub fn get(&self, pos: Pos) -> Option<&T>{
		Some(&self.storage[self.index(pos)?])
	}

	#[inline]
	pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T>{
		let index = self.index(pos)?;
		Some(&mut self.storage[index])
	}
	
	#[inline]
	pub fn set(&mut self, pos: Pos, val: T) -> Option<()>{
		let index = self.index(pos)?;
		self.storage[index] = val;
		Some(())
	}
}

