use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use banki_common::auth::AuthToken;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::fs;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
	pub auth_token: Option<AuthToken>,
	pub no_warn_offline: bool,
	pub use_custom_mouse_handler: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			auth_token: None,
			no_warn_offline: false,
			use_custom_mouse_handler: true,
		}
	}
}

lazy_static! {
	static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

pub fn load() {
	if let Ok(data) = fs::get_config() {
		if let Ok(config) = ron::from_str(&data) {
			let mut lock = CONFIG.write().unwrap();
			*lock = config;
		}
	}
}

pub fn save() {
	let Ok(data) = ron::to_string(&*get()) else {return};
	let _ = std::fs::write(fs::get_config_path(), data);
}

pub fn get() -> RwLockReadGuard<'static, Config> {
	CONFIG.read().unwrap()
}

pub fn get_mut() -> RwLockWriteGuard<'static, Config> {
	CONFIG.write().unwrap()
}