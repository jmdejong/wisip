

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
	Use(Direction)
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

