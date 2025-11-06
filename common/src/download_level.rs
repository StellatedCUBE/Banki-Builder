use bincode::{Decode, Encode};

use crate::Request;

#[derive(Encode, Decode, Clone)]
pub struct DownloadLevelRQ(pub u32);

impl Request for DownloadLevelRQ {
	type Response = Vec<u8>;
	const PATH: &'static str = "l";
}