use crate::controller::{command_handler::CommandSender, level::{mochi::Mochi, Character, LevelTheme, L_BLOCK}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
pub struct ToolMochi {}

impl Tool for ToolMochi {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(x, y, L_BLOCK) {
			return vec![];
		}

		drop(level);

		sound::play(command_sender, sound::SE_MOCHI2);
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(Mochi { x, y })))]
    }

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		self.use_start(command_sender, editor, x, y)
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::MOCHI
	}
}