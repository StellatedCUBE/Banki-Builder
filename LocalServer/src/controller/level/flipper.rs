use std::{mem, sync::atomic::{AtomicBool, Ordering}};

use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;
use super::{Level, LevelObject, LevelTheme, AABB};

pub static SHIFT_HITBOX: AtomicBool = AtomicBool::new(false);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum SpikeDirections {
	None,
	Up,
	Down,
	Both
}

impl SpikeDirections {
	pub fn up(self) -> bool {
		self as u8 & 1 == 1
	}

	pub fn count(self) -> u8 {
		match self {
			Self::None => 0,
			Self::Up |
			Self::Down => 1,
			Self::Both => 2
		}
	}

	pub fn parse(data: u8) -> Self {
		unsafe { mem::transmute(data & 3) }
	}
}

pub struct Flipper {
	pub x: i32, pub y: i32,
	pub spikes: SpikeDirections
}

impl LevelObject for Flipper {
	fn save_ids(&self, level: &Level) -> Vec<usize> {
		let mut c = level.connect_up(self.x, self.y);
		c.append(&mut level.connect_up(self.x - 1, self.y));
		c.append(&mut level.connect_down(self.x, self.y - 1));
		c.append(&mut level.connect_down(self.x - 1, self.y - 1));
		c
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, _return_object: bool) -> GameObject {
		let mut go = GameObject::new(game_object::OBJ_FLIPPER, 0);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_real(command_sender, 0, self.spikes as u8 as f32);

		go
	}

	fn post_create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, self_object: &GameObject, saved_objects: Vec<&GameObject>) {
		for (i, object) in saved_objects.into_iter().enumerate() {
			self_object.set_object(command_sender, i as u8, object);
		}
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, _level: &Level) -> Vec<GameObject> {
		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 0);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, match self.spikes {
			SpikeDirections::None => sprite::FLIPPER_NONE,
			SpikeDirections::Both => sprite::FLIPPER_BOTH,
			_ => sprite::FLIPPER
		});

		if self.spikes == SpikeDirections::Down {
			go.set_rotation(command_sender, 180.0);
		}
		
		vec![go]
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32 - 1.5625,
			y: self.y as f32 - if SHIFT_HITBOX.load(Ordering::Relaxed) { 0.0 } else if self.spikes.up() { 0.375 } else { 0.125 },
			width: if SHIFT_HITBOX.load(Ordering::Relaxed) { 2.0 } else { 3.0625 },
			height: 0.25 * (1 + self.spikes.count()) as f32
		}
	}

	fn types(&self) -> u32 {
		super::L_FLIPPER | super::L_PHYSICS_OBJECT_FULL
	}

	fn serialized_type(&self) -> u8 {
		12
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		to.write(&[self.spikes as u8])?;
		Ok(())
	}

	#[cfg(not(feature = "verify"))]
	fn context_menu_items(&self, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![ContextMenuItem::IconList(vec![
			sprite::TOOL_FLIPPER,
			sprite::TOOL_FLIPPER + 1,
			sprite::TOOL_FLIPPER + 2,
			sprite::TOOL_FLIPPER + 3,
		], self.spikes as usize, 0.5)]
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		let new = SpikeDirections::parse((action - sprite::TOOL_FLIPPER) as u8);

		if self.spikes == new {
			vec![]
		}

		else {
			sound::play(command_sender, sound::SE_HOLD);
			let revert = self.spikes as i32 + sprite::TOOL_FLIPPER;
			self.spikes = new;
			vec![UndoAction::ContextMenuAction(object, revert)]
		}
	}
}

impl Flipper {
	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 4 {
			Self {
				x: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32,
				spikes: SpikeDirections::parse(data[4]),
			}
		} else {
			Self { x: 0, y: 0, spikes: SpikeDirections::None }
		}
	}
}