use std::time::SystemTime;

const EPOCH: u64 = 1744544300;

pub fn now() -> u32 {
	(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - EPOCH) as u32
}