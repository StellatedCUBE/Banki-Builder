use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Clone)]
pub struct PublishLevelRQ {
	pub level: Vec<u8>,
	pub verification_run: Vec<u8>,
	pub verification_time: u32,
}

impl Request for PublishLevelRQ {
	type Response = u32;
	const PATH: &'static str = "p";
}

#[derive(Encode, Decode)]
pub struct VerificationResponse {
	pub name: String,
	pub tags: u32,
	pub character_bit: u8,
	pub theme_bit: u32,
	pub metadata_buf: Vec<u8>,
}