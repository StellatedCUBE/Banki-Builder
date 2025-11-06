use bincode::{Decode, Encode};

use crate::{auth::AuthToken, Request, Response, User};

#[derive(Encode, Decode, Clone)]
pub struct GetSteamOpenIDLinkRQ {
	pub user: User,
	pub language: u8,
}

#[derive(Encode, Decode)]
pub struct GetSteamOpenIDLinkRSSuccess {
	pub link: String,
	pub auth_token: AuthToken,
	pub token_id: i64,
}

#[derive(Encode, Decode)]
pub enum GetSteamOpenIDLinkRS {
	Ok(GetSteamOpenIDLinkRSSuccess),
	Err
}

impl Request for GetSteamOpenIDLinkRQ {
	type Response = GetSteamOpenIDLinkRS;
	const PATH: &'static str = "o";
}

impl Response for GetSteamOpenIDLinkRS {}