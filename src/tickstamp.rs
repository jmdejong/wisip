
use std::ops::{Add, Sub};
use serde::{Serialize, Deserialize};
use crate::random;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tickstamp(pub i64);

impl Tickstamp {
	pub fn increment(&mut self) {
		self.0 += 1;
	}
	
	pub fn random_seed(&self) -> u32 {
		random::randomize_u32(self.0 as u32 ^ 12345)
	}
}

impl Add<TickDuration> for Tickstamp {
	type Output = Self;
	fn add(self, other: TickDuration) -> Self {
		Self(self.0 + other.0)
	}
}

impl Sub<TickDuration> for Tickstamp {
	type Output = Self;
	fn sub(self, other: TickDuration) -> Self {
		Self(self.0 - other.0)
	}
}

impl Sub<Self> for Tickstamp {
	type Output = TickDuration;
	fn sub(self, other: Self) -> TickDuration {
		TickDuration(self.0 - other.0)
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TickDuration(pub i64);

impl Sub<Self> for TickDuration {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		TickDuration(self.0 - other.0)
	}
}
impl Add<Self> for TickDuration {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		TickDuration(self.0 + other.0)
	}
}
