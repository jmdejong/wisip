


use serde::{Serialize, Deserialize};
use enum_assoc::Assoc;
use crate::{
	sprite::Sprite,
	action::{InteractionType, Interactable},
	tile::Structure,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Sprite)]
#[func(fn describe(&self) -> &str)]
#[func(fn interactions(&self) -> Vec<Interactable> {Vec::new()})]
#[func(fn next(&self) -> Option<(i64, CropType)>)]
#[func(fn grow(&self) -> Option<(i64, Structure)>)]
#[func(fn fertilized_grow(&self) -> Option<(i64, Structure)>)]
enum CropType {
	
	#[assoc(sprite = Sprite::PlantedSeed)]
	#[assoc(describe = "Planted seed")]
	#[assoc(next = (1, CropType::GreenSeedling))]
	GreenSeed,
	
	#[assoc(sprite = Sprite::Seedling)]
	#[assoc(describe = "Seedling")]
	#[assoc(next = (1, CropType::YoungLeafPlant))]
	GreenSeedling,
	
	#[assoc(sprite = Sprite::LeafPlant)]
	#[assoc(describe = "A small plant with big round leaves")]
	#[assoc(next = (1, CropType::LeafPlant))]
	YoungLeafPlant,
	
	#[assoc(sprite = Sprite::LeafPlant)]
	#[assoc(describe = "A plant with big round leaves")]
	#[assoc(grow = (1, Structure::DiscLeaf))]
	LeafPlant,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crop {
	typ: CropType,
	flags: u8
}

const WATERED: u8 = 1<<7;
const FERTILIZED: u8 = 1<<6;

impl Crop {
	pub fn all_interactions(&self) -> Vec<Interactable> {
		let mut interactions = self.typ.interactions();
		if self.flags & WATERED == 0 {
			interactions.push(Interactable::transform(InteractionType::Water, 1, Structure::Crop(self.water())));
		}
		if self.flags & FERTILIZED == 0 && self.typ.fertilized_grow().is_some() {
			interactions.push(Interactable::transform(InteractionType::Fertilize, 1, Structure::Crop(self.fertilize())));
		}
		interactions
	}
	
	fn water(&self) -> Self {
		Self { typ: self.typ, flags: self.flags | WATERED }
	}
	
	fn fertilize(&self) -> Self {
		Self { typ: self.typ, flags: self.flags | FERTILIZED }
	}
	
	pub fn description(&self) -> String {
		if self.flags & WATERED != 0 {
			self.typ.describe().to_string()
		} else {
			format!("{}. Needs water", self.typ.describe())
		}
	}
	
	pub fn grow(&self) -> Option<(i64, Structure)> {
		if self.flags & WATERED == 0 {
			None
		} else if self.flags & FERTILIZED != 0 && self.typ.fertilized_grow().is_some() {
			self.typ.fertilized_grow()
		} else if let Some((steps, typ)) = self.typ.next() {
			let crop = Self { typ, flags: 0 };
			Some((steps, Structure::Crop(crop)))
		} else {
			self.typ.grow()
		}
	}
	
	pub fn sprite(&self) -> Sprite {
		self.typ.sprite()
	}
	
	pub fn greenseed() -> Self {
		Self {typ: CropType::GreenSeed, flags: 0 }
	}
}
