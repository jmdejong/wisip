

use serde::{Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize)]
#[serde(rename_all="lowercase")]
pub enum Sprite {
	#[serde(rename="player")]
	PlayerDefault,
	Dirt,
	Grass1,
	Grass2,
	Grass3,
	StoneFloor,
	Gravel,
	DenseGrass,
	Heather,
	Shrub,
	Bush,
	Sanctuary,
	Water,
	Wall,
	Rock,
	RockMid,
	Sapling,
	YoungTree,
	Tree,
	OldTree,
	Stone,
	Pebble,
	Crop,
	Flower,
	Reed,
	PitcherPlant,
}
