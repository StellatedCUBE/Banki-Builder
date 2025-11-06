use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Vote {
	#[default]
	None,
	Like,
}

#[derive(Encode, Decode, Clone)]
pub struct SetVoteRQ {
	pub level: u32,
	pub vote: Vote
}

impl Request for SetVoteRQ {
	type Response = u32;
	const PATH: &'static str = "V";
}