use crate::controller::{command_handler::CommandOutput, game_object::{self, GameObject}, sprite};

use super::{Level, LevelObject, AABB};

pub struct Mochi {
	pub x: i32, pub y: i32,
}

impl LevelObject for Mochi {
	fn save_ids(&self, level: &Level) -> Vec<usize> {
		let mut c = level.connect_up(self.x, self.y);
		c.append(&mut level.connect_down(self.x, self.y));
		c
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, _return_object: bool) -> GameObject {
		let mut go = GameObject::new(game_object::OBJ_MOCHI, -1);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);

		go
	}

	fn post_create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, self_object: &GameObject, saved_objects: Vec<&GameObject>) {
		for (i, object) in saved_objects.into_iter().enumerate() {
			self_object.set_object(command_sender, i as u8, object);
		}
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, _level: &Level) -> Vec<GameObject> {
		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, -1);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, sprite::MOCHI);
		
		vec![go]
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32,
			y: self.y as f32,
			width: 1.0,
			height: 1.0
		}
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn types(&self) -> u32 {
		super::L_BLOCK
	}

	fn serialized_type(&self) -> u8 {
		3
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		Ok(())
	}
}

impl Mochi {
	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 3 {
			Self {
				x: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32
			}
		} else {
			Self { x: 0, y: 0 }
		}
	}
}