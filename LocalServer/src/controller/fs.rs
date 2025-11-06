use std::{fs, path::PathBuf, sync::Mutex};
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static!{
	static ref SAVE_DIRECTORY: Mutex<PathBuf> = Mutex::new(PathBuf::from("."));
}

pub fn set_save_directory(path: PathBuf) {
	*SAVE_DIRECTORY.lock().unwrap() = path;
}

pub fn get_level_folder() -> PathBuf {
	let mut buf = SAVE_DIRECTORY.lock().unwrap().clone();
	buf.push("levels");
	buf
}

pub fn new_level_filename() -> PathBuf {
	let mut buf = get_level_folder();
	fs::create_dir(&buf).ok();
	buf.push(Uuid::new_v4().as_simple().to_string() + ".lvl");
	buf
}

pub fn get_level_db_path() -> PathBuf {
	let mut buf = SAVE_DIRECTORY.lock().unwrap().clone();
	buf.push("levels.db");
	buf
}

pub fn get_level_db() -> std::io::Result<Vec<u8>> {
	fs::read(get_level_db_path())
}

pub fn get_config_path() -> PathBuf {
	let mut buf = SAVE_DIRECTORY.lock().unwrap().clone();
	buf.push("banki-builder-DO-NOT-SHARE.ron");
	buf
}

pub fn get_config() -> std::io::Result<String> {
	Ok(String::from_utf8_lossy(&fs::read(get_config_path())?).to_string())
}

pub const VERIFY_TAS_FILENAME: &'static str = "verify.tas";

fn get_verify_tas_path() -> PathBuf {
	let mut buf = SAVE_DIRECTORY.lock().unwrap().clone();
	buf.push(VERIFY_TAS_FILENAME);
	buf
}

pub fn get_verify_tas() -> std::io::Result<Vec<u8>> {
	fs::read(get_verify_tas_path())
}

pub const RUN_TAS_FILENAME: &'static str = "run.tas";

fn get_run_tas_path() -> PathBuf {
	let mut buf = SAVE_DIRECTORY.lock().unwrap().clone();
	buf.push(RUN_TAS_FILENAME);
	buf
}

pub fn get_run_tas() -> std::io::Result<Vec<u8>> {
	fs::read(get_run_tas_path())
}

pub fn del_run_tas() {
	let _ = fs::remove_file(get_run_tas_path());
}

#[cfg(windows)]
fn get_portkey_path() -> PathBuf {
	let mut buf = SAVE_DIRECTORY.lock().unwrap().clone();
	buf.push("portkey");
	buf
}

#[cfg(windows)]
pub fn write_portkey(port: u16, key: [u8; 16]) {
	use std::io::Write;
	
	let mut buf = Vec::with_capacity(18);
	let _ = buf.write(&port.to_be_bytes());
	let _ = buf.write(&key);
	fs::write(get_portkey_path(), &buf).unwrap();
}

#[cfg(not(windows))]
pub fn write_portkey(_port: u16, _key: [u8; 16]) {}