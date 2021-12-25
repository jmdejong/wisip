

use std::fmt;
use serde::{Serialize, Serializer};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Display)]
// #[serde(rename_all="lowercase", untagged)]

pub enum Sprite {
// 	#[serde(serialize_with="serialize_custom_sprite")]
	Custom(&'static str),
// 	#[serde(serialize_with="serialize_letter_sprite")]
	Letter(char),
// 	#[serde(serialize_with="serialize_player_sprite")]
	Player(&'static str, char),
	Stone,
	Dirt,
	Grass1,
	Grass2,
	Grass3,
	Sanctuary,
	Water,
	#[strum(serialize=" ")]
	Empty
}

const VALID_COLOURS: &'static[&'static str] = &["r", "g", "b", "c", "m", "y", "lr", "lg", "lb", "lc", "lm", "ly", "a"];

impl Sprite {
	
	pub const fn new(name: &'static str) -> Self {
		Self::Custom(name)
	}
	
	pub fn player_sprite(spritename: &str) -> Option<Sprite> {
		let lowername = spritename.to_lowercase();
		let (colour_name, letter_str) = lowername.strip_prefix("player_")?.split_once("-")?;
		let letter: char = letter_str.chars().next()?;
		let colour = VALID_COLOURS.iter().find(|colour| *colour == &colour_name)?;
		if letter_str.len() == 1 && letter.is_ascii_alphabetic() {
			Some(Self::Player(colour, letter))
		} else {
			None
		}
	}
	
	pub fn letter_sprite(letter: char) -> Option<Sprite> {
		if letter.is_ascii_graphic() {
			Some(Self::Letter(letter))
		} else {
			None
		}
	}
	
	pub const DIRT: Sprite = Sprite::Custom("dirt");
	pub const STONE: Sprite = Sprite::Custom("stone");
	pub const GRASS1: Sprite = Sprite::Custom("grass1");
	pub const GRASS2: Sprite = Sprite::Custom("grass2");
	pub const GRASS3: Sprite = Sprite::Custom("grass3");
	pub const SANCTUARY: Sprite = Sprite::Custom("sanctuary");
	pub const WATER: Sprite = Sprite::Custom("water");
}

fn serialize_custom_sprite<S>(name: &'static str, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
	name.serialize(serializer)
}
fn serialize_letter_sprite<S>(letter: &char, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
	format!("emptyletter-{}", letter).serialize(serializer)
}
fn serialize_player_sprite<S>(colour: &'static str, letter: &char, serializer: S) -> Result<S::Ok, S::Error>
		where S: Serializer {
	format!("player_{}-{}", colour, letter).serialize(serializer)
}


impl Serialize for Sprite {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		(match self {
			Sprite::Custom(name) => format!("{}", name),
			Sprite::Letter(letter) => format!("emptyletter-{}", letter),
			Sprite::Player(colour, letter) => format!("player_{}-{}", colour, letter),
			sprite => format!("{}", self).to_lowercase()
		}).serialize(serializer)
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_player_sprite_creation() {
		assert_eq!(Sprite::player_sprite("player_lg-a"), Some(Sprite::Player("lg", 'a')));
	}
// 	#[test]
// 	fn test_player_sprite_display() {
// 		assert_eq!(format!("{}", Sprite::Player("lg", 'a')), "player_lg-a".to_string());
// 	}
	#[test]
	fn test_letter_sprite_creation() {
		assert_eq!(Sprite::letter_sprite('A'), Some(Sprite::Letter('A')));
	}
// 	#[test]
// 	fn test_letter_sprite_display() {
// 		assert_eq!(format!("{}", Sprite::Letter('A')), "emptyletter-A".to_string());
// 	}
}
