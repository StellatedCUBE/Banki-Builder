use bincode::{Decode, Encode};

use crate::{Request, Response, SessionToken, User, UserData, UserDataPriv};

pub type AuthToken = [u8; 32];

#[derive(Encode, Decode, Clone)]
pub struct LogInRQ {
	pub user: User,
	pub token: Option<AuthToken>,
	pub version: u16,
}

#[derive(Encode, Decode, Debug)]
pub struct LogInRSSuccess {
	pub auth_token: AuthToken,
	pub session_token: SessionToken,
	pub user_data: UserData,
	pub user_data_priv: UserDataPriv,
}

#[derive(Encode, Decode, Debug)]
pub enum LogInRS {
	Ok(LogInRSSuccess),
	BadToken,
	OutOfDate,
	BadData,
}

impl Request for LogInRQ {
	type Response = LogInRS;
	const PATH: &'static str = "a";
}

impl Response for LogInRS {}