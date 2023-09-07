

use serde::{Serialize, Deserialize};
use crate::{
	PlayerId,
	Direction,
	Vec2,
	timestamp::Timestamp
};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Control {
	Move(Direction),
	Movement(Vec2),
	Suicide,
	Interact(Option<Direction>),
	Select(Selector),
	MoveSelected(Selector),
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control, Timestamp)
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Selector {
	Next,
	Previous,
	Idx(usize),
}
