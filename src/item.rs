

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use enum_assoc::Assoc;
use crate::{
	action::{Action, InteractionType::*, CraftType},
	tile::Structure,
	hashmap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Assoc)]
#[serde(rename_all="snake_case")]
#[func(pub fn action(&self) -> Option<Action>)]
#[func(pub fn description(&self) -> Option<&str>)]
#[func(pub fn name(&self) -> &str)]
pub enum Item {
	#[assoc(name="<eyes>")]
	#[assoc(action=Action::Inspect)]
	#[assoc(description="Inspect things around you")]
	Eyes,
	
	#[assoc(name="<hands>")]
	#[assoc(action=Action::take())]
	#[assoc(description="Take items that are laying loose")]
	Hands,
	
	#[assoc(name="reed")]
	#[assoc(description="Some cut reeds")]
	Reed,
	
	#[assoc(name="flower")]
	#[assoc(description="A pretty flower")]
	#[assoc(action=Action::Craft(CraftType::Marker, Item::MarkerStone, hashmap![Item::Stone => 1, Item::Flower => 9]))]
	Flower,
	
	#[assoc(name="pebble")]
	#[assoc(description="A small stone")]
	Pebble,
	
	#[assoc(name="stone")]
	#[assoc(description="A mid-size stone. Stones can be broken by smashing two together")]
	#[assoc(action=Action::new(Smash, 1, true))]
	Stone,
	
	#[assoc(name="sharp stone")]
	#[assoc(description="A small stone with a sharp edge. It can be used to cut things, though it is very crude and may not always work")]
	#[assoc(action=Action::new(Cut, 1, false))]
	SharpStone,
	
	#[assoc(name="pitcher")]
	#[assoc(description="A pitcher from the pitcher plant. It can function as a bucket")]
	#[assoc(action=Action::Craft(CraftType::Water, Item::FilledPitcher, HashMap::new()))]
	Pitcher,
	
	#[assoc(name="water pitcher")]
	#[assoc(description="A pitcher from the pitcher plant, filled with water")]
	#[assoc(action=Action::new(Water, 1, false))]
	FilledPitcher,
	
	#[assoc(name="hoe")]
	#[assoc(description="A simple hoe that can be used to clear the ground of small vegetation")]
	#[assoc(action=Action::Clear)]
	Hoe,
	
	#[assoc(name="green seeds")]
	#[assoc(description="Unknown green seeds")]
	#[assoc(action=Action::Build(Structure::GreenSeed, HashMap::new()))]
	GreenSeed,
	
	#[assoc(name="yellow seeds")]
	#[assoc(action=Action::Build(Structure::YellowSeed, HashMap::new()))]
	#[assoc(description="Unknown yellow seeds")]
	YellowSeed,
	
	#[assoc(name="brown seeds")]
	#[assoc(action=Action::Build(Structure::BrownSeed, HashMap::new()))]
	#[assoc(description="Unknown brown seeds")]
	BrownSeed,
	
	#[assoc(name="stick")]
	#[assoc(description="Stick")]
	Stick,
	
	#[assoc(name="tinder")]
	#[assoc(description="Tinder from the tinder fungus. Can be placed with some pebbles on a clear space to create a fireplace")]
	#[assoc(action=Action::Build(Structure::Fireplace, hashmap![Item::Pebble => 10]))]
	Tinder,
	
	#[assoc(name="marker stone")]
	#[assoc(description="A marker stone that can be placed to create a land claim")]
	#[assoc(action=Action::BuildClaim(Structure::MarkStone))]
	MarkerStone,
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn hands_has_take_action() {
		assert_eq!(Item::Hands.action(), Some(Action::take()));
	}
	
}
