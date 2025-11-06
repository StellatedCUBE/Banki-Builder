use std::time::SystemTime;

use banki_common::set_vote::Vote;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;

#[must_use]
#[derive(Serialize)]
pub enum LogMessage<'a> {
	StartServer,
	CreateUser(u64, bool),
	UpdateUserProperties(u64, &'a str),
	CreateLevel(u32, &'a str, u64, u32, u8, u32, u32),
	SetTime(u32, u64, u32),
	SetVote(u32, u64, Vote),
	DeleteLevel(u32),
}

impl LogMessage<'_> {
	pub fn print(self) {
		if let Ok(message) = ron::to_string(&self) {
			println!("[{}] {}", timestamp(), message);
		}
	}
}

fn timestamp() -> String {
	let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    now.to_rfc3339_opts(SecondsFormat::Secs, true)
}