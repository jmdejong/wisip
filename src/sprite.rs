

use serde::{Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize)]
#[serde(rename_all="lowercase")]
#[allow(dead_code)]
pub enum Sprite {
	#[serde(rename="player")]
	PlayerDefault,
	Sage,
	Dirt,
	Grass1,
	Grass2,
	Grass3,
	StoneFloor,
	WoodFloor,
	Gravel,
	DenseGrass,
	Heather,
	Shrub,
	Bush,
	Sanctuary,
	Water,
	Wall,
	WoodWall,
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
	Rush,
	Lilypad,
	Moss,
	DeadLeaves,
	PitcherPlant,
	Fireplace,
	MarkStone,
	Altar,
}
