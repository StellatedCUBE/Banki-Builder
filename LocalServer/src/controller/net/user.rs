use std::{collections::HashMap, sync::{atomic::{AtomicU64, Ordering}, RwLock}};

use banki_common::{User, UserData};
use lazy_static::lazy_static;

use crate::controller::loc::data::{LOC_SELF, LOC_UNKNOWN_USER};

lazy_static! {
	static ref USER_DATA: RwLock<HashMap<User, UserData>> = RwLock::default();
}

pub static SELF: AtomicU64 = AtomicU64::new(76561198029653963 * cfg!(debug_assertions) as u64);

pub fn me() -> User {
	User::from_u(SELF.load(Ordering::Relaxed))
}

pub fn is_me(id: u64) -> bool {
	id == 0 || id == SELF.load(Ordering::Relaxed)
}

pub fn learn(data: UserData) {
	USER_DATA.write().unwrap().insert(data.id, data);
}

pub fn get(id: u64) -> UserData {
	let user = if id == 0 {
		let self_id = SELF.load(Ordering::Relaxed);
		if self_id == 0 {
			return UserData {
				id: User::from_i(-1),
				name: LOC_SELF.for_current_locale().to_owned(),	
			};
		}
		User::from_u(self_id)
	} else { User::from_u(id) };

	USER_DATA.read().unwrap().get(&user).cloned().unwrap_or_else(|| UserData {
		id: user,
		name: if user.u() == SELF.load(Ordering::Relaxed) {
			LOC_SELF
		} else {
			LOC_UNKNOWN_USER
		}.for_current_locale().to_owned(),
	})
}