
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
	Take,
	Smash
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
	typ: ActionType,
	level: u32
}

impl Action{
	pub fn take(level: u32) -> Self {
		Self { typ: ActionType::Take, level }
	}
	pub fn smash(level: u32) -> Self {
		Self { typ: ActionType::Smash, level }
	}
	
	pub fn fulfilled_by(&self, other: Action) -> bool {
		other.typ == self.typ && other.level >= self.level
	}
}
