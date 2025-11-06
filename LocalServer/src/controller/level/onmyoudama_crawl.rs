use std::mem;

use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;
use super::{Level, LevelObject, LevelTheme, AABB};

pub const LAYER: u32 = super::L_BLOCK | super::L_PHYSICS_OBJECT_FULL;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum Face {
	Bottom,
	Left,
	Top,
	Right
}

pub struct OnmyoudamaCrawl {
	pub x: i32, pub y: i32,
	pub face: Face,
	pub direction: bool
}

impl LevelObject for OnmyoudamaCrawl {
	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, _level: &Level) -> Vec<GameObject> {
		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, -9);
		go.x = (self.x * 32 + match self.face {
			Face::Bottom |
			Face::Top => 0,
			Face::Left => -9,
			Face::Right => 9
		} + if self.direction {
			32
		} else {
			0
		}) as f32;
		go.y = (self.y * 32 + match self.face {
			Face::Left |
			Face::Right => 0,
			Face::Bottom => 9,
			Face::Top => -9
		}) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, sprite::ONMYOUDAMA_BLUE);

		if self.direction {
			go.set_scale(command_sender, -1.0, 1.0);
		}

		vec![go]
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, return_object: bool) -> GameObject {
		let mut go = GameObject::new(game_object::OBJ_ONMYOUDAMA_CRAWL, -11);
		go.x = (self.x * 32 + match self.face {
			Face::Bottom |
			Face::Top => 0,
			Face::Left => -9,
			Face::Right => 9
		}) as f32;
		go.y = (self.y * 32 + match self.face {
			Face::Left |
			Face::Right => 0,
			Face::Bottom => 9,
			Face::Top => -9
		}) as f32;
		go.create(command_sender);
		if self.direction {
			go.set_real(command_sender, 0, 1.0);
		}
		go.set_real(command_sender, 1, self.face as u8 as f32);

		if !return_object {
			go.destroy_server_only();
		}

		go
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32 + match self.face {
				Face::Bottom |
				Face::Top => 0.25,
				Face::Left => 0.0,
				Face::Right => 0.5
			},
			y: self.y as f32 + match self.face {
				Face::Left |
				Face::Right => 0.25,
				Face::Bottom => 0.5,
				Face::Top => 0.0
			},
			width: 0.5,
			height: 0.5
		}
	}

	fn types(&self) -> u32 {
		LAYER
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;	
	}

	fn serialized_type(&self) -> u8 {
		10
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		to.write(&[self.face as u8 | (self.direction as u8 * 4)])?;
		Ok(())
	}

	#[cfg(not(feature = "verify"))]
	fn context_menu_items(&self, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![
			ContextMenuItem::IconList(vec![
				sprite::BUTTON_FACE_BOTTOM,
				sprite::BUTTON_FACE_LEFT,
				sprite::BUTTON_FACE_TOP,
				sprite::BUTTON_FACE_RIGHT
			], self.face as usize, 1.0),
			ContextMenuItem::IconList(vec![
				sprite::SWAP,
				sprite::CLOCKWISE
			], !self.direction as usize, 1.0)
		]
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		if action >= sprite::BUTTON_FACE_BOTTOM {
			let old = self.face;
			let new = unsafe { mem::transmute((action - sprite::BUTTON_FACE_BOTTOM) as u8) };
			if old == new {
				vec![]
			} else {
				self.face = new;
				sound::play(command_sender, sound::SE_HOLD);
				vec![UndoAction::ContextMenuAction(object, old as i32 + sprite::BUTTON_FACE_BOTTOM)]
			}
		}

		else {
			if self.direction == (action == sprite::SWAP) {
				vec![]
			} else {
				self.direction = action == sprite::SWAP;
				sound::play(command_sender, sound::SE_HOLD);
				vec![UndoAction::ContextMenuAction(object, if self.direction { sprite::CLOCKWISE } else { sprite::SWAP })]
			}
		}
	}
}

impl OnmyoudamaCrawl {
	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 4 {
			Self {
				x: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32,
				face: unsafe { mem::transmute(data[4] & 3) },
				direction: data[4] > 3,
			}
		} else {
			Self {
				x: 0,
				y: 0,
				face: Face::Bottom,
				direction: false
			}
		}
	}
}