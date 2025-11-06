use std::num::NonZeroU32;

use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Clone)]
pub struct SetTimeRQ {
	pub level: u32,
	pub time: NonZeroU32,
	pub run: Vec<u8>,
}

impl Request for SetTimeRQ {
	type Response = ();
	const PATH: &'static str = "t";
}