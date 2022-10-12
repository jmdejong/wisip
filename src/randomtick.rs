
use crate::{
	pos::Pos,
	timestamp::Timestamp
};

pub const CHUNK_SIZE: i32 = 16;
const STEP : i64 = 541;
const STEP_INVERSE: i64 = 53;
pub const CHUNK_AREA: i64 = (CHUNK_SIZE * CHUNK_SIZE) as i64;

pub fn tick_position(time: Timestamp) -> Pos {
	let ind = (time.0 * STEP % CHUNK_AREA) as i32;
	Pos::new(ind % CHUNK_SIZE, ind / CHUNK_SIZE)
}

fn tick_time(pos: Pos) -> i64 {
	(pos.x + pos.y * CHUNK_SIZE) as i64 * STEP_INVERSE % CHUNK_AREA
}

pub fn tick_num(pos: Pos, time: Timestamp) -> i64 {
	time.0 / CHUNK_AREA
		+ i64::from(tick_time(pos) <= time.0 % CHUNK_AREA)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn step_inverse_is_inverse_of_step() {
		assert_eq!(STEP * STEP_INVERSE % CHUNK_AREA, 1);
	}
	
	#[test]
	fn tick_time_reverses_tick_position() {
		for i in 0..5000.min(CHUNK_AREA as i64) {
			assert_eq!(tick_time(tick_position(Timestamp(i))) as i64, i);
		}
	}
}
