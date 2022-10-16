

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
	DenseGrass,
	Shrub,
	Bush,
	Sanctuary,
	Water,
	Wall,
	Rock,
	Tree,
	Crop,
	Flower,
	Reed
}
