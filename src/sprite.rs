

use serde::{Serialize, Serializer};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Display)]
pub enum Sprite {
	Letter(char),
	Player(&'static str, char),
	Stone,
	Dirt,
	Grass1,
	Grass2,
	Grass3,
	Sanctuary,
	Water,
	Wall,
	Gate,
	Rubble,
	Rock,
	#[strum(serialize=" ")]
	Empty
}

const VALID_COLOURS: &'static[&'static str] = &["r", "g", "b", "c", "m", "y", "lr", "lg", "lb", "lc", "lm", "ly", "a"];

impl Sprite {
	
	
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
}



impl Serialize for Sprite {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		match self {
			Sprite::Letter(letter) => format!("emptyletter-{}", letter).serialize(serializer),
			Sprite::Player(colour, letter) => format!("player_{}-{}", colour, letter).serialize(serializer),
			_ => format!("{}", self).to_lowercase().serialize(serializer)
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::{Value, json};
	
	#[test]
	fn test_player_sprite_creation() {
		assert_eq!(Sprite::player_sprite("player_lg-a"), Some(Sprite::Player("lg", 'a')));
	}
	#[test]
	fn test_player_sprite_serialize() {
		assert_eq!(json!(Sprite::Player("lg", 'a')), json!("player_lg-a"));
	}
	#[test]
	fn test_letter_sprite_creation() {
		assert_eq!(Sprite::letter_sprite('A'), Some(Sprite::Letter('A')));
	}
	#[test]
	fn test_letter_sprite_display() {
		assert_eq!(json!(Sprite::Letter('A')), json!("emptyletter-A"));
	}
}
