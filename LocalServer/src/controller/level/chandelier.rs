use std::collections::HashMap;

use crate::controller::{command_handler::CommandOutput, game_object::{self, GameObject}, sprite};

use super::{simple_object::ObjectType, Level, LevelObject, AABB};

pub const LAYER: u32 = super::L_BLOCK | super::L_PHYSICS_OBJECT;

pub struct Chandelier {
	pub x: i32,
	pub y: i32
}

impl LevelObject for Chandelier {
	fn save_ids(&self, level: &Level) -> Vec<usize> {
		let mut c = level.connect_up(self.x, self.y);
		c.append(&mut level.connect_up(self.x + 1, self.y));
		c
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, level: &Level, _return_object: bool) -> GameObject {
		let tm = level.tile_manager();
		let mut min_tile_y = tm.bounding_box().y as i32;

		let mut conveyors = HashMap::new();
		for object in &level.objects {
			if let Some(ObjectType::Conveyor(dir)) = object.simple_object_type() {
				conveyors.insert((object.bounding_box().x as i32, object.bounding_box().y as i32), dir);
				min_tile_y = min_tile_y.min(object.bounding_box().y as i32);
			}
		}

		let mut obj_x = self.x * 32 + 8;
		let mut line_length = 0;
		let mut conveyor = false;
		let off_top = loop {
			let y = self.y - line_length - 1;

			if y < min_tile_y {
				break true;
			}

			let mut left = conveyors.contains_key(&(self.x, y));
			let mut right = conveyors.contains_key(&(self.x + 1, y));

			if left || right {
				for object in &level.objects {
					if let Some(split) = object.to_tile_split() {
						if !split.vertical && split.y == y + 1 {
							if split.x == self.x {
								left = false;
							} else if split.x == self.x + 1 {
								right = false;
							}
						}
					}
				}
			}

			if (left && !right) || (left && right && conveyors.get(&(self.x, y)) != conveyors.get(&(self.x + 1, y))) {
				conveyor = true;
				obj_x -= 2;
				break false;
			}

			if !left && right {
				conveyor = true;
				obj_x += 2;
				break false;
			}

			if left && right {
				conveyor = true;
				break false;
			}

			left = tm.get(self.x, y).solid();
			right = tm.get(self.x + 1, y).solid();

			if left || right {
				for object in &level.objects {
					if let Some(split) = object.to_tile_split() {
						if !split.vertical && split.y == y + 1 {
							if split.x == self.x {
								left = false;
							} else if split.x == self.x + 1 {
								right = false;
							}
						}
					}
				}

				if left && !right {
					obj_x -= 8;
				} else if !left && right {
					obj_x += 8;
				}

				break false;
			}

			line_length += 1;
		};

		if conveyor {

			let mut go = GameObject::new(game_object::OBJ_CONVEYOR_CHANDELIER, -1);
			go.x = obj_x as f32;
			go.y = (self.y * 32) as f32;
			go.create(command_sender);
			go.set_real(command_sender, 0, (line_length * 32) as f32);

			go
		}

		else {
			let mut go = GameObject::new(game_object::OBJ_CHANDELIER, -1);
			go.x = obj_x as f32;
			go.y = (self.y * 32) as f32;
			go.create(command_sender);
			go.set_real(command_sender, 0, (line_length * 32) as f32);
			go.set_real(command_sender, 1, if off_top { 2.0 } else { 1.0 });

			go
		}
	}

	fn post_create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, self_object: &GameObject, saved_objects: Vec<&GameObject>) {
		let mut used0 = false;
		let mut used2 = false;

		for object in saved_objects {
			if object.x >= (self.x * 32 + 32) as f32 {
				self_object.set_object(command_sender, 2 + used2 as u8, object);
				used2 = true;
			} else {
				self_object.set_object(command_sender, used0 as u8, object);
				used0 = true;
			}
		}
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, _level: &Level) -> Vec<GameObject> {
		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, -1);
		go.x = (self.x * 32 + 8) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, sprite::CHANDELIER);

		vec![go]
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32 + 0.25,
			y: self.y as f32,
			width: 1.46875,
			height: 1.0
		}
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn types(&self) -> u32 {
		LAYER
	}

	fn serialized_type(&self) -> u8 {
		7
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		Ok(())
	}
}

impl Chandelier {
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