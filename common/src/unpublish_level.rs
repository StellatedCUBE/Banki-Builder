use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Clone)]
pub struct UnpublishLevelRQ {
	pub level: u32
}

impl Request for UnpublishLevelRQ {
	type Response = ();
	const PATH: &'static str = "x";
}