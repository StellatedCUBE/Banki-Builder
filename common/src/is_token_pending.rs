use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Clone)]
pub struct IsTokenPendingRQ {
	pub token_id: i64,
}

impl Request for IsTokenPendingRQ {
	type Response = bool;
	const PATH: &'static str = "v";
}