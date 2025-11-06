use bincode::{Decode, Encode};

use crate::{OnlineLevelMetadata, Request, Response};

#[derive(Encode, Decode, Clone)]
pub enum LevelOrdering {
	New, Top,
}

#[derive(Encode, Decode, Clone)]
pub struct SearchLevelsRQ {
	pub id: u32,
	pub order: LevelOrdering,
	pub tags: u32,
	pub neg_tags: u32,
	pub characters: u8,
	pub themes: u32,
}

#[derive(Encode, Decode)]
pub struct SearchLevelsRS {
	pub results: Vec<OnlineLevelMetadata>,
	pub more: bool,
}

impl Response for SearchLevelsRS {}

impl Request for SearchLevelsRQ {
	type Response = SearchLevelsRS;
	const PATH: &'static str = "q";
}