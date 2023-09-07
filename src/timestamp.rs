

use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct Timestamp(SystemTime);


impl Timestamp {
	pub fn from_epoch_millis(epoch_millis: u64) -> Self {
		Self(UNIX_EPOCH + Duration::from_millis(epoch_millis))
	}

	pub fn now() -> Self {
		Self(SystemTime::now())
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TimeDelta(Duration);

