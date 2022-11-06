
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use enum_assoc::Assoc;
use crate::{
	sprite::Sprite,
	inventory::Item,
	action::{Action, InteractionType, CraftType, Interactable, InteractionResult},
	timestamp::Timestamp,
	worldmessages::SoundType::Explain,
	hashmap,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Option<Sprite>)]
#[func(fn accessible(&self) -> bool {true})]
#[func(fn clear(&self) -> Option<Ground>)]
#[func(fn describe(&self) -> Option<&str>)]
#[func(fn craft(&self) -> Option<CraftType>)]
#[func(fn buildable(&self) -> bool {false})]
#[func(pub fn restoring(&self) -> bool {false})]
pub enum Ground {
	#[assoc(sprite = Sprite::Dirt)]
	#[assoc(describe = "Dirt")]
	#[assoc(buildable = true)]
	#[assoc(restoring = true)]
	Dirt,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Grass1)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Grass")]
	Grass1,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Grass2)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Grass")]
	Grass2,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Grass3)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Grass")]
	Grass3,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::Moss)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Moss")]
	Moss,
	
	#[assoc(clear = Ground::Dirt)]
	#[assoc(sprite = Sprite::DeadLeaves)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Old leaves")]
	DeadLeaves,
	
	#[assoc(sprite = Sprite::Sanctuary)]
	#[assoc(describe = "Ornate stone floor")]
	Sanctuary,
	
	#[assoc(sprite = Sprite::Water)]
	#[assoc(has_water = true)]
	#[assoc(accessible = false)]
	#[assoc(describe = "Water")]
	#[assoc(craft = CraftType::Water)]
	Water,
	
	#[assoc(sprite = Sprite::StoneFloor)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Rock floor")]
	RockFloor,
	
	#[assoc(sprite = Sprite::StoneFloor)]
	#[assoc(buildable = true)]
	#[assoc(describe = "Stone floor")]
	StoneFloor,
	
	#[assoc(sprite = Sprite::WoodFloor)]
	#[assoc(describe = "Wooden plank floor")]
	WoodFloor,
	
	#[assoc(accessible = false)]
	Empty
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Assoc, Serialize, Deserialize)]
#[func(fn sprite(&self) -> Option<Sprite>)]
#[func(fn blocking(&self) -> bool {false})]
#[func(pub fn is_open(&self) -> bool {false})]
#[func(fn explain(&self) -> Option<&str>)]
#[func(fn interactions(&self) -> Vec<Interactable> {Vec::new()})]
#[func(fn take(&self) -> Option<Item>)]
#[func(fn describe(&self) -> Option<&str>)]
#[func(fn craft(&self) -> Option<CraftType>)]
#[func(fn grow(&self) -> Option<(i64, Structure)>)]
pub enum Structure {
	#[assoc(is_open = true)]
	Air,
	
	#[assoc(sprite = Sprite::Wall)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Stone wall")]
	Wall,
	
	#[assoc(sprite = Sprite::WoodWall)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Wooden wall")]
	WoodWall,
	
	#[assoc(sprite = Sprite::Rock)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Natural rock wall")]
	Rock,
	
	#[assoc(sprite = Sprite::RockMid)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Natural rock wall")]
	RockMid,
	
	#[assoc(sprite = Sprite::Sapling)]
	#[assoc(describe = "Sapling")]
	Sapling,
	
	#[assoc(sprite = Sprite::YoungTree)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Young tree")]
	YoungTree,
	
	#[assoc(sprite = Sprite::Tree)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Tree")]
	Tree,
	
	#[assoc(sprite = Sprite::OldTree)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Dead tree")]
	OldTree,
	
	#[assoc(sprite = Sprite::OldTree)]
	#[assoc(blocking = true)]
	#[assoc(interactions = vec![Interactable::new(InteractionType::Cut, 1, &[0.5, 1.0], Some(Structure::OldTree), &[Item::Tinder])])]
	#[assoc(describe = "Dead tree with tinder fungus on it")]
	OldTreeTinder,
	
	#[assoc(sprite = Sprite::DenseGrass)]
	#[assoc(interactions = vec![Interactable::harvest(InteractionType::Take, 0, &[0.1], &[Item::GreenSeed])])]
	#[assoc(describe = "Dense grass")]
	DenseGrassGrn,
	
	#[assoc(sprite = Sprite::DenseGrass)]
	#[assoc(interactions = vec![Interactable::harvest(InteractionType::Take, 0, &[0.1], &[Item::BrownSeed])])]
	#[assoc(describe = "Dense grass")]
	DenseGrassBrn,
	
	#[assoc(sprite = Sprite::DenseGrass)]
	#[assoc(interactions = vec![Interactable::harvest(InteractionType::Take, 0, &[0.1], &[Item::YellowSeed])])]
	#[assoc(describe = "Dense grass")]
	DenseGrassY,
	
	#[assoc(sprite = Sprite::Heather)]
	#[assoc(describe = "Heather")]
	Heather,
	
	#[assoc(sprite = Sprite::Rush)]
	#[assoc(describe = "Rush")]
	Rush,
	
	#[assoc(sprite = Sprite::Shrub)]
	#[assoc(describe = "Some shrub")]
	Shrub,
	
	#[assoc(sprite = Sprite::Bush)]
	#[assoc(describe = "Just a bush")]
	Bush,
	
	#[assoc(sprite = Sprite::Reed)]
	#[assoc(interactions = vec![Interactable::harvest(InteractionType::Cut, 1, &[0.5, 1.0], &[Item::Reed])])]
	#[assoc(describe = "Reeds. Can be cut")]
	Reed,
	
	#[assoc(sprite = Sprite::PitcherPlant)]
	#[assoc(interactions = vec![Interactable::harvest(InteractionType::Cut, 1, &[0.5, 1.0], &[Item::Pitcher])])]
	#[assoc(describe = "Pitcher plant. Can be cut")]
	PitcherPlant,
	
	#[assoc(sprite = Sprite::Flower)]
	#[assoc(take = Item::Flower)]
	#[assoc(describe = "Flower")]
	Flower,
	
	#[assoc(sprite = Sprite::Pebble)]
	#[assoc(take = Item::Pebble)]
	#[assoc(describe = "Pebble. A small stone")]
	Pebble,
	
	#[assoc(sprite = Sprite::Stone)]
	#[assoc(take = Item::Stone)]
	#[assoc(interactions = vec![
		Interactable::new(
			InteractionType::Smash,
			1,
			&[0.4, 1.0],
			Some(Structure::Gravel),
			&[Item::SharpStone],
		)
	])]
	#[assoc(describe = "Stone. A medium-size cobble. Can be smashed to try to get smaller stones")]
	Stone,
	
	#[assoc(sprite = Sprite::Gravel)]
	#[assoc(describe = "Gravel. Small stone rocks")]
	Gravel,
	
	#[assoc(sprite = Sprite::Sage)]
	#[assoc(blocking = true)]
	#[assoc(explain = "Sage")]
	#[assoc(describe = "Sage. An old wise person with grey hair. This sage can tell you about items in your inventory")]
	Sage,
	
	#[assoc(sprite = Sprite::Fireplace)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Fireplace. Safe place to have a fire")]
	Fireplace,
	
	#[assoc(sprite = Sprite::Altar)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Marker Altar. Bring 10 flowers and a stone to create a marker stone")]
	#[assoc(craft = CraftType::Marker)]
	MarkerAltar,
	
	#[assoc(sprite = Sprite::MarkStone)]
	#[assoc(blocking = true)]
	#[assoc(describe = "Mark stone. Center of a land claim")]
	MarkStone,
	
	#[assoc(sprite = Sprite::PlantedSeed)]
	#[assoc(grow = (1, Structure::BrownSeedling))]
	#[assoc(describe = "Planted seed")]
	BrownSeed,
	
	#[assoc(grow = (1, Structure::BrownSeedling))]
	#[assoc(describe = "Planted seed")]
	GreenSeed,
	
	#[assoc(sprite = Sprite::PlantedSeed)]
	#[assoc(grow = (1, Structure::BrownSeedling))]
	#[assoc(describe = "Planted seed")]
	YellowSeed,
	
	#[assoc(sprite = Sprite::Seedling)]
	#[assoc(describe = "Seedling")]
	#[assoc(grow = (1, Structure::StickPlant))]
	BrownSeedling,
	
	#[assoc(sprite = Sprite::GreenStem)]
	#[assoc(describe = "A plant with a long stem")]
	#[assoc(grow = (3, Structure::Stick))]
	StickPlant,
	
	#[assoc(sprite = Sprite::BrownStem)]
	#[assoc(describe = "Stick. A brown stem is what remains of the plant")]
	#[assoc(interactions = vec![Interactable::harvest(InteractionType::Take, 0, &[], &[Item::Stick])])]
	Stick,
}

impl Structure {
	fn interactables(&self) -> Vec<Interactable> {
		let mut interactions = self.interactions();
		if let Some(item) = self.take() {
			interactions.push(Interactable::take(&[item]));
		}
		interactions
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
	pub ground: Ground,
	pub structure: Structure
}

impl Tile {
	pub fn ground(ground: Ground) -> Tile{
		Self{ground, structure: Structure::Air}
	}
	
	pub fn structure(ground: Ground, structure: Structure) -> Tile {
		Self{ground, structure}
	}
	
	pub fn sprites(&self) -> Vec<Sprite> {
		[self.structure.sprite(), self.ground.sprite()].into_iter()
			.flatten()
			.collect()
	}
	
	pub fn blocking(&self) -> bool {
		!self.ground.accessible() || self.structure.blocking()
	}
	
	fn can_build(&self) -> bool {
		self.structure.is_open() && self.ground.buildable()
	}
	
	pub fn interact(&self, item: Item, time: Timestamp) -> Option<InteractionResult> {
		if let Some(name) = self.structure.explain() {
			if item.action() != Some(Action::Inspect) {
				return Some(InteractionResult {
					message: Some((Explain, format!("{}: {}", name, item.description().unwrap_or("Unknown")))),
					..Default::default()
				});
			}
		}
		match item.action()? {
			Action::Interact(interact) => {
				let mut result = self.structure.interactables()
					.into_iter()
					.filter_map(|interactable| interactable.apply(interact, time))
					.next()?;
				if interact.use_item {
					result.cost.insert(item, 1);
				}
				Some(result)
			}
			Action::Clear =>
				if self.structure.is_open() {
					Some(InteractionResult {
						remains_ground: Some(self.ground.clear()?),
						..Default::default()
					})
				} else {
					None
				}
			Action::Inspect => 
				Some(InteractionResult {
					message: Some((
						Explain,
						format!("{}  --  {}", self.ground.describe().unwrap_or(""), self.structure.describe().unwrap_or(""))
					)),
					..Default::default()
				}),
			Action::BuildClaim(structure) =>
				if self.can_build() {
					Some(InteractionResult {
						remains: Some(structure),
						cost: hashmap!{item => 1},
						claim: true,
						..Default::default()
					})
				} else {
					None
				}
			Action::Build(structure, mut cost) =>
				if self.can_build() {
					cost.entry(item).and_modify(|n| {*n += 1;}).or_insert(1);
					Some(InteractionResult {
						remains: Some(structure),
						cost,
						build: true,
						..Default::default()
					})
				} else {
					None
				}
			Action::Craft(typ, product, mut cost) => {
				cost.entry(item).and_modify(|n| {*n += 1;}).or_insert(1);
				if Some(typ) == self.structure.craft() || Some(typ) == self.ground.craft() {
					Some(InteractionResult {
						items: vec![product],
						cost,
						..Default::default()
					})
				} else {
					None
				}
			}
		}
	}
	
	pub fn grow(&self) -> Option<(i64, Structure)> {
		self.structure.grow()
	}
}

impl Default for Tile {
	fn default() -> Tile {
		Tile::ground(Ground::Empty)
	}
}

impl Serialize for Tile {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		(self.ground, self.structure).serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for Tile {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let (ground, structure) = <(Ground, Structure)>::deserialize(deserializer)?;
		Ok(Self{ground, structure})
	}
}

