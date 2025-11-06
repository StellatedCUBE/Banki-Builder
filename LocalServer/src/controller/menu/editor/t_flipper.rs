use std::sync::atomic::{AtomicU8, Ordering};

use crate::controller::{command_handler::CommandSender, level::{flipper::{self, Flipper, SpikeDirections}, Character, LevelTheme, L_FLIPPER, L_PHYSICS_OBJECT_FULL}, sound, sprite, undo::UndoAction};

use super::{context_menu::ContextMenuItem, tool::Tool, Editor};

pub struct ToolFlipper {
	spikes: AtomicU8
}

impl Default for ToolFlipper {
	fn default() -> Self {
		Self {
			spikes: AtomicU8::new(SpikeDirections::Up as u8)
		}
	}
}

impl Tool for ToolFlipper {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.round() as i32;
		let y = y.round() as i32;

		let level = editor.level.lock().unwrap();

		flipper::SHIFT_HITBOX.store(true, Ordering::Relaxed);

		for i in -2..2 {
			for j in 0..2 {
				for object in level.at(x + i, y - j) {
					let types = level.object_types(object);
					if types & (L_FLIPPER | L_PHYSICS_OBJECT_FULL) == L_PHYSICS_OBJECT_FULL ||
						(i < 1 && j == 0 && types & L_FLIPPER != 0) {
						flipper::SHIFT_HITBOX.store(false, Ordering::Relaxed);
						return vec![];
					}
				}
			}
		}

		flipper::SHIFT_HITBOX.store(false, Ordering::Relaxed);

		drop(level);

		sound::play(command_sender, sound::SE_HOLD);
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(Flipper { x, y, spikes: SpikeDirections::parse(self.spikes.load(Ordering::Relaxed)) })))]
    }

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		self.use_start(command_sender, editor, x, y)
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::TOOL_FLIPPER + self.spikes.load(Ordering::Relaxed) as i32
	}

	fn context_menu_items(&self, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![ContextMenuItem::IconList(vec![
			sprite::TOOL_FLIPPER,
			sprite::TOOL_FLIPPER + 1,
			sprite::TOOL_FLIPPER + 2,
			sprite::TOOL_FLIPPER + 3,
		], self.spikes.load(Ordering::Relaxed) as usize, 0.5)]
	}

	fn handle_context_menu_action(&self, action: i32, _theme: LevelTheme) {
		self.spikes.store((action - sprite::TOOL_FLIPPER) as u8, Ordering::Relaxed);
	}
}