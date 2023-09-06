
use crate::pos::{Pos, Area};

use std::ops::{Add, Mul, Div};
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32
}

impl Vec2 {

	pub fn new(x: f32, y: f32) -> Self {
		Self {x, y}
	}

	pub fn zero() -> Self {
		Self {x: 0.0, y: 0.0}
	}

	pub fn from_pos(pos: &Pos) -> Self {
		Self::new(pos.x as f32, pos.y as f32)
	}

	pub fn size(&self) -> f32 {
		self.x.hypot(self.y)
	}

	pub fn try_normalize(self) -> Option<Self> {
		let size = self.size();
		if size == 0.0 {
			None
		} else {
			Some(self / size)
		}
	}

	pub fn round(&self) -> Pos {
		Pos::new(self.x.round() as i32, self.y.round() as i32)
	}

	pub fn floor(&self) -> Pos {
		Pos::new(self.x.floor() as i32, self.y.floor() as i32)
	}

	pub fn ceil(&self) -> Pos {
		Pos::new(self.x.ceil() as i32, self.y.ceil() as i32)
	}
}


impl Add<Vec2> for Vec2 {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Div<f32> for Vec2 {
	type Output = Self;
	fn div(self, denominator: f32) -> Self {
		Self {
			x: self.x / denominator,
			y: self.y / denominator
		}
	}
}
impl Mul<f32> for Vec2 {
	type Output = Self;
	fn mul(self, multiplicant: f32) -> Self {
		Self {
			x: self.x * multiplicant,
			y: self.y * multiplicant
		}
	}
}


impl Serialize for Vec2 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		(self.x, self.y).serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for Vec2 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
		Ok(Self{x, y})
	}
}


#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Rect {
	origin: Vec2,
	size: Vec2
}

impl Rect {
	pub fn new(origin: Vec2, size: Vec2) -> Self {
		Self { origin, size }
	}

	pub fn min(&self) -> Vec2 {
		self.origin
	}

	pub fn max(&self) -> Vec2 {
		self.origin + self.size
	}

	pub fn outer_area(&self) -> Area {
		Area::between(self.origin.floor(), self.max().ceil())
	}

	pub fn moved(&self, delta: Vec2) -> Self {
		Self {
			origin: self.origin + delta,
			size: self.size
		}
	}
}




