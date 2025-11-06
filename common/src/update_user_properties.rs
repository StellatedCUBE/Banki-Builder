use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Clone)]
pub struct UpdateUserPropertiesRQ {
	pub name: String,
}

impl Request for UpdateUserPropertiesRQ {
	type Response = bool;
	const PATH: &'static str = "u";
}