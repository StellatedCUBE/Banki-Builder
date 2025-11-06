use crate::controller::{command_handler::CommandSender, level::{Character, LevelTheme}, sound, sprite, undo::UndoAction};

use super::tool::Tool;

#[derive(Default)]
pub struct ToolDelete {}

impl Tool for ToolDelete {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let mut vec = editor.level.lock().unwrap().atf(x, y);
		let mut undo = vec![];
		vec.sort_unstable();
		for obj in vec {
			if let Some(action) = editor.delete(command_sender, obj) {
				undo.push(action);
			}
		}

		if undo.len() > 0 {
			sound::play(command_sender, sound::SE_HOLD);
		}

		undo
	}

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) -> Vec<UndoAction> {
		self.use_start(command_sender, editor, x, y)
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::TOOL_DELETE
	}
}