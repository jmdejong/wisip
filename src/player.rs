
use std::fmt;
use serde::{Serialize, Deserialize};

use crate::{
	Pos,
	controls::Control,
	item::ItemRef as Item
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub String);

impl fmt::Display for PlayerId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug, Clone)]
pub struct Player {
	pub plan: Option<Control>,
	pub body: usize,
	pub is_new: bool,
	pub view_center: Option<Pos>,
	pub inventory: Vec<Item>
}
