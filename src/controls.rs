

use serde::{Serialize, Deserialize};
use crate::{
	PlayerId,
	Direction
};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Control {
	Move(Direction),
	Suicide,
	Interact(Option<Direction>),
	Select(Selection)
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Selection {
	Next,
	Previous
}
