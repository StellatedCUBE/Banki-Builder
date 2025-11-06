use std::{collections::HashMap, num::NonZeroU32, sync::RwLock};

use banki_common::{set_vote::{SetVoteRQ, Vote}, User, UserDataPriv};
use lazy_static::lazy_static;

use super::{query, user::{is_me, me}};

lazy_static! {
	static ref PBS: RwLock<HashMap<u32, NonZeroU32>> = RwLock::default();
	static ref WRS: RwLock<HashMap<u32, (User, NonZeroU32)>> = RwLock::default();
	static ref LIKES: RwLock<HashMap<u32, u32>> = RwLock::default();
	static ref VOTES: RwLock<HashMap<u32, Vote>> = RwLock::default();
}

pub fn pb(level_id: u32) -> Option<NonZeroU32> {
	PBS.read().unwrap().get(&level_id).cloned()
}

pub fn wr(level_id: u32) -> Option<NonZeroU32> {
	WRS.read().unwrap().get(&level_id).map(|p| p.1)
}

pub fn likes(level_id: u32) -> u32 {
	LIKES.read().unwrap().get(&level_id).cloned().unwrap_or_default()
}

pub fn get_vote(level_id: u32) -> Vote {
	VOTES.read().unwrap().get(&level_id).cloned().unwrap_or_default()
}

pub fn set_time(level_id: u32, user: User, time: NonZeroU32) {
	if wr(level_id).is_none_or(|t| t > time) {
		WRS.write().unwrap().insert(level_id, (user, time));
	}

	if is_me(user.u()) && pb(level_id).is_none_or(|t| t > time) {
		PBS.write().unwrap().insert(level_id, time);
	}
}

pub fn set_likes(level_id: u32, likes: u32) {
	LIKES.write().unwrap().insert(level_id, likes);
}

pub fn set_vote(level_id: u32, vote: Vote) {
	VOTES.write().unwrap().insert(level_id, vote);
}

pub fn read_initial(data: &UserDataPriv) {
	for level in &data.self_level_data {
		set_time(level.id, level.wr_holder, level.wr_time);
		set_likes(level.id, level.likes);
	}

	for (level_id, time, vote) in &data.pbs {
		set_time(*level_id, me(), *time);
		set_vote(*level_id, *vote);
	}
}

pub async fn vote(level_id: u32, vote: Vote) {
	if let Ok(likes) = query(&SetVoteRQ {
		level: level_id,
		vote
	}).await {
		set_vote(level_id, vote);
		set_likes(level_id, likes);
	}
}