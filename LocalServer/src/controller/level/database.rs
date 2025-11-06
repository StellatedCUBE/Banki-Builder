use std::{collections::{HashMap, HashSet}, sync::atomic::{AtomicBool, Ordering}, time::SystemTime};
use bincode::{Decode, Encode};
use lazy_static::lazy_static;
use tokio::{io::AsyncWriteExt, sync::Mutex};

use crate::controller::fs;
use super::metadata::LevelMetadata;

const DB_VER: u16 = 0;

#[derive(Encode, Decode, Default)]
struct LevelDB {
	levels: HashMap<String, LevelMetadata>
}

impl LevelDB {
	async fn load() -> Self {
		let mut db = Self::default();

		if let Ok(data) = fs::get_level_db() {
			if data.len() > 2 && u16::from_le_bytes(data[0..2].try_into().unwrap()) == DB_VER {
				if let Ok((deserialized_db, _)) = bincode::decode_from_slice(&data[2..], bincode::config::standard()) {
					db = deserialized_db;
				}
			}
		}

		let mut level_filenames: HashSet::<String> = HashSet::new();
		let mut level_files: Vec<(String, SystemTime)> = vec![];

		if let Ok(read_dir) = std::fs::read_dir(fs::get_level_folder()) {
			for dir in read_dir {
				if let Ok(dir) = dir {
					if let Ok(metadata) = dir.metadata() {
						if let Ok(time) = metadata.modified() {
							let name = dir.file_name().to_string_lossy().to_string();
							if name.ends_with(".lvl") {
								level_filenames.insert(name.clone());
								level_files.push((name, time));
							}
						}
					}
				}
			}
		}

		let mut dirty = false;

		for (name, time) in level_files {
			if !db.levels.contains_key(&name) || db.levels[&name].modified_time != time {
				let mut level_path = fs::get_level_folder();
				level_path.push(&name);
				if let Ok(mut file) = tokio::fs::File::open(level_path).await {
					if let Ok(mut new) = LevelMetadata::load(&mut file).await {
						new.modified_time = time;
						new.filename = name.clone();
						db.levels.insert(name, new);
						dirty = true;
					}
				}
			}
		}

		for name in db.levels.keys().cloned().collect::<Vec<String>>() {
			if !level_filenames.contains(&name) {
				db.levels.remove(&name);
				dirty = true;
			}
		}

		if dirty {
			if let Ok(buf) = bincode::encode_to_vec(&db, bincode::config::standard()) {
				if let Ok(mut file) = tokio::fs::File::create(fs::get_level_db_path()).await {
					file.write_u16_le(DB_VER).await.ok();
					file.write_all(&buf).await.ok();
				}
			}
		}

		db
	}
}

lazy_static!{
	static ref LEVEL_DB: Mutex<LevelDB> = Mutex::from(LevelDB::default());
}

static LOADED: AtomicBool = AtomicBool::new(false);

pub fn load() {
	LOADED.store(false, Ordering::Relaxed);
	tokio::spawn(async {
		*LEVEL_DB.lock().await = LevelDB::load().await;
		LOADED.store(true, Ordering::Relaxed);
	});
}

pub fn get_levels() -> Option<Vec<LevelMetadata>> {
	if !LOADED.load(Ordering::Relaxed) {
		return None;
	}

	match LEVEL_DB.try_lock() {
		Ok(level_db) => {
			let mut vec: Vec<LevelMetadata> = level_db.levels.values().cloned().collect();
			vec.sort_unstable_by_key(|m| m.modified_time);
			vec.reverse();
			Some(vec)
		}

		Err(_) => None
	}
}