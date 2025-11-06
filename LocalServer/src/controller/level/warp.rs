use std::mem;

use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use crate::controller::loc::data::*;
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;
use super::{Level, LevelObject, LevelTheme, SubObjectDeleteUndoAction, AABB};

pub const LAYER: u32 = super::L_BLOCK;

pub struct Warp {
	pub x1: i32, pub y1: i32,
	x2: i32, y2: i32
}

impl LevelObject for Warp {
	fn sub_object_count(&self) -> usize {2}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, _return_object: bool) -> GameObject {
		let mut go = GameObject::new(game_object::OBJ_WARP, 2000);
		go.x = (self.x1 * 32) as f32;
		go.y = (self.y1 * 32) as f32;
		go.create(command_sender);
		go.set_real(command_sender, 0, (self.x2 * 32) as f32);
		go.set_real(command_sender, 1, (self.y2 * 32) as f32);
		go.set_sprite(command_sender, sprite::WARP + 1);
		go.destroy_server_only();

		go = GameObject::new(game_object::OBJ_WARP, 2000);
		go.x = (self.x2 * 32) as f32;
		go.y = (self.y2 * 32) as f32;
		go.create(command_sender);
		go.set_real(command_sender, 0, (self.x1 * 32) as f32);
		go.set_real(command_sender, 1, (self.y1 * 32) as f32);
		go.destroy_server_only();

		go
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, _level: &Level) -> Vec<GameObject> {
		let mut a = GameObject::new(game_object::OBJ_NO_ANIM, 2000);
		a.x = (self.x1 * 32) as f32;
		a.y = (self.y1 * 32) as f32;
		a.create(command_sender);
		a.set_sprite(command_sender, sprite::WARP + 1);

		let mut b = GameObject::new(game_object::OBJ_NO_ANIM, 2000);
		b.x = (self.x2 * 32) as f32;
		b.y = (self.y2 * 32) as f32;
		b.create(command_sender);
		b.set_sprite(command_sender, sprite::WARP);

		if (self.x1 - self.x2).abs() + (self.y1 - self.y2).abs() > 1 {
			let mut connector = GameObject::new(game_object::OBJ_WARP_EDITOR, -10);
			let dx = self.x2 - self.x1;
			let dy = self.y2 - self.y1;
			let m24 = ((dx * dx + dy * dy) as f32).sqrt() / 24.0;
			let dx = dx as f32 / m24;
			let dy = dy as f32 / m24;
			connector.x = (self.x1 * 32 + 16) as f32 + dx;
			connector.y = (self.y1 * 32 + 16) as f32 + dy;
			connector.create(command_sender);
			connector.set_real(command_sender, 0, (self.x2 * 32 + 16) as f32 - dx);
			connector.set_real(command_sender, 1, (self.y2 * 32 + 16) as f32 - dy);
			vec![a, b, connector]
		}

		else {
			vec![a, b]
		}
	}

	fn bounding_box(&self) -> AABB {
		self.sub_object_bounding_box(0) | self.sub_object_bounding_box(1)
	}

	fn sub_object_bounding_box(&self, sub_object: usize) -> AABB {
		if sub_object == 0 {
			AABB {
				x: self.x1 as f32,
				y: self.y1 as f32,
				width: 1.0,
				height: 1.0
			}
		} else {
			AABB {
				x: self.x2 as f32,
				y: self.y2 as f32,
				width: 1.0,
				height: 1.0
			}
		}
	}

	fn delete_sub_object(&mut self, _command_sender: &mut CommandSender, _objects: &mut Vec<GameObject>, _object: usize, _sub_object: usize)
	-> SubObjectDeleteUndoAction {
		SubObjectDeleteUndoAction::DeleteMain
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x1 += x;
		self.x2 += x;
		self.y1 += y;
		self.y2 += y;
	}

	fn move_sub_object_by(&mut self, sub_object: usize, x: i32, y: i32) {
		if sub_object == 0 {
			self.x1 += x;
			self.y1 += y;
		} else {
			self.x2 += x;
			self.y2 += y;
		}
	}

	fn ghost_sub_object(&self, command_sender: &mut CommandSender, sub_object: usize, objects: &mut Vec<GameObject>) -> ((i32, i32), i32) {
		if objects.len() > 2 {
			objects.remove(2).destroy(command_sender);
		}

		if objects.len() > 1 {
			objects.remove(sub_object).destroy(command_sender);
		}

		else {
			objects[0].destroy(command_sender);
		}

		(
			if sub_object == 0 {(
				self.x1,
				self.y1
			)} else {(
				self.x2,
				self.y2
			)},
			sprite::WARP + 1 - sub_object as i32
		)
	}

	fn types(&self) -> u32 {
		LAYER
	}

	fn sub_object_types(&self, _sub_object: usize) -> u32 {
		LAYER
	}

	fn serialized_type(&self) -> u8 {
		5
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x1 as i16).to_le_bytes())?;
		to.write(&(self.y1 as i16).to_le_bytes())?;
		to.write(&(self.x2 as i16).to_le_bytes())?;
		to.write(&(self.y2 as i16).to_le_bytes())?;
		Ok(())
	}

	#[cfg(not(feature = "verify"))]
	fn sub_object_context_menu_items(&self, _sub_object: usize, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![ContextMenuItem::LabeledIcon(sprite::SWAP, LOC_SWAP_ENDS.for_current_locale_static(), 0xffffff)]
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		sound::play(command_sender, sound::SE_WARP);
		mem::swap(&mut self.x1, &mut self.x2);
		mem::swap(&mut self.y1, &mut self.y2);
		vec![UndoAction::ContextMenuAction(object, action)]
	}

	fn handle_sub_object_context_menu_action(
		&mut self,
		command_sender: &mut CommandSender,
		object: usize,
		_sub_object: usize,
		action: i32,
		theme: LevelTheme
	) -> (bool, Vec<UndoAction>) {
		(true, self.handle_context_menu_action(command_sender, object, action, theme))
	}
}

impl Warp {
	pub const fn new(x: i32, y: i32) -> Self {
		Self {
			x1: x, y1: y,
			x2: x + 1, y2: y,
		}
	}

	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 7 {
			Self {
				x1: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y1: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32,
				x2: i16::from_le_bytes(data[4..6].try_into().unwrap()) as i32,
				y2: i16::from_le_bytes(data[6..8].try_into().unwrap()) as i32,
			}
		} else {
			Self::new(0, 0)
		}
	}
}