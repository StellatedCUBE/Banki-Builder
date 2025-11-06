use std::hash::Hash;

use crate::controller::{command_handler::CommandOutput, game_object::{self, GameObject}, level::LevelTheme, sprite};

use super::{Level, LevelObject, AABB};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TileSplit {
	pub x: i32,
	pub y: i32,
	pub vertical: bool
}

impl LevelObject for TileSplit {
	fn create(&self, _command_sender: &mut dyn CommandOutput, _level: &Level, _return_object: bool) -> GameObject {
		GameObject::null()
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject> {
		level.tile_manager().splits.lock().unwrap().insert(*self);

		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, -5000);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;

		if self.vertical {
			go.x -= 3.0;
			go.y += 2.0;
		} else {
			go.x += 2.0;
			go.y -= 3.0;
		}

		go.create(command_sender);
		go.set_sprite(command_sender, if self.vertical { sprite::TILE_SPLIT_VERTICAL } else { sprite::TILE_SPLIT_HORIZONTAL });
		go.set_colour(command_sender, match level.theme {
			LevelTheme::FarawayLabyrinth |
			LevelTheme::JerryAttack |
			LevelTheme::Koumakan |
			LevelTheme::Entrance |
			LevelTheme::MindBreak => 0xff00,
			_ => 0xff
		});

		vec![go]
	}

	#[cfg(not(feature = "verify"))]
	fn destroy_editor_view(&self, command_sender: &mut dyn CommandOutput, objects: &mut Vec<GameObject>, level: &Level) {
		let tm = level.tile_manager();
		if tm.splits.lock().unwrap().remove(self) {
			let bounds = AABB {
				x: -100000.0,
				y: -100000.0,
				width: 200000.0,
				height: 200000.0
			};
			let x = self.x;
			let y = self.y;
			tm.update_tile(command_sender, x, y, &bounds, None);
			tm.update_tile(command_sender, x + 1, y, &bounds, None);
			tm.update_tile(command_sender, x - 1, y, &bounds, None);
			tm.update_tile(command_sender, x, y + 1, &bounds, None);
			tm.update_tile(command_sender, x, y - 1, &bounds, None);
			tm.update_tile(command_sender, x - 1, y - 1, &bounds, None);
			tm.update_tile(command_sender, x - 1, y + 1, &bounds, None);
			tm.update_tile(command_sender, x + 1, y - 1, &bounds, None);
		}

		objects[0].destroy(command_sender);
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32 - if self.vertical { 0.09375 } else { 0.0 },
			y: self.y as f32 - if self.vertical { 0.0 } else { 0.09375 },
			width: if self.vertical { 0.1875 } else { 1.0 },
			height: if self.vertical { 1.0 } else { 0.1875 }
		}
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn types(&self) -> u32 { 0 }

	fn serialized_type(&self) -> u8 { 2 }

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		to.write(&[self.vertical as u8])?;
		Ok(())
	}

	fn to_tile_split(&self) -> Option<&TileSplit> {
		Some(self)
	}
}

impl Hash for TileSplit {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		((self.x + 32768) as u64 | ((self.y + 32768) as u64 * 65536) | if self.vertical { 1 << 32 } else { 0 }).hash(state);
	}
}

impl TileSplit {
	pub fn deserialize(from: &[u8]) -> anyhow::Result<Self> {
		let x = i16::from_le_bytes(from[0..2].try_into()?) as i32;
		let y = i16::from_le_bytes(from[2..4].try_into()?) as i32;
		let vertical = from[4] != 0;

		Ok(Self {x, y, vertical})
	}
}