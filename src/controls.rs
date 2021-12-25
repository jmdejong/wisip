

use serde::{Serialize, Deserialize};
use crate::{
	PlayerId,
	Direction,
	sprite::Sprite,
	item::ItemRef
};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Control {
	Move(Direction),
	Suicide,
	Use(ItemRef, Direction)
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId, Sprite),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

