use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::controller::{command_handler::CommandSender, level::{simple_object::{Direction, ObjectType, SimpleObject}, Character, LevelTheme, ObjectID, PL_IMMUTABLE_BLOCK}, sound, sprite, undo::UndoAction};

use super::{context_menu::ContextMenuItem, tool::Tool, Editor};

#[derive(Default)]
pub struct ToolCannon {
	switch: AtomicBool,
	object: AtomicU32,
}

impl Tool for ToolCannon {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let ix = x.floor() as i32;
		let iy = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(ix, iy, PL_IMMUTABLE_BLOCK) {
			return vec![];
		}

		drop(level);

		if let ObjectID::Object(id) = editor.add(command_sender, Box::new(SimpleObject {
			object_type: ObjectType::Cannon(direction(x, y, ix, iy), self.switch.load(Ordering::Relaxed)),
			x: ix, y: iy
		})) {
			self.object.store(id as u32, Ordering::Relaxed);
			vec![UndoAction::Delete(ObjectID::Object(id))]
		}

		else {
			unreachable!()
		}
	}

	fn use_frame(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) {
		let oid = self.object.load(Ordering::Relaxed);

		if oid != 0 {
			let mut level = editor.level.lock().unwrap();
			let object = level.objects[oid as usize].to_simple_object_mut();
			if let ObjectType::Cannon(d, switch) = object.object_type {
				let nd = direction(x, y, object.x, object.y);
				if d != nd {
					sound::play(command_sender, sound::SE_MESSAGE);
					object.object_type = ObjectType::Cannon(nd, switch);
					editor.objects[oid as usize][0].set_sprite(command_sender, object.object_type.editor_sprite(LevelTheme::DreamFields));
				}
			}
		}
	}

	fn use_end(&self, command_sender: &mut CommandSender, _editor: &mut Editor) {
		if self.object.load(Ordering::Relaxed) != 0 {
			sound::play(command_sender, sound::SE_HEAD);
		}
		
		self.object.store(0, Ordering::Relaxed);
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		if self.switch.load(Ordering::Relaxed) {
			sprite::CANNON_RED_X + 2
		} else {
			sprite::CANNON + 3
		}
	}

	fn context_menu_items(&self, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![ContextMenuItem::IconList(vec![sprite::CANNON + 3, sprite::CANNON_RED_X + 2], self.switch.load(Ordering::Relaxed) as usize, 0.5)]
	}

	fn handle_context_menu_action(&self, action: i32, _theme: LevelTheme) {
		self.switch.store(action > sprite::CANNON_RED_X, Ordering::Relaxed);
	}
}

fn direction(mut x: f32, mut y: f32, object_x: i32, object_y: i32) -> Direction {
	x -= object_x as f32 + 0.5;
	y -= object_y as f32 + 0.5;

	if x.abs() > y.abs() {
		if x > 0.0 { Direction::Right }
		else { Direction::Left }
	} else {
		if y > 0.0 { Direction::Down }
		else { Direction::Up }
	}
}