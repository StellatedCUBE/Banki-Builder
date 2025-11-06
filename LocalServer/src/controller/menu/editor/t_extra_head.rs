use crate::controller::{command_handler::CommandSender, level::{extra_head::{self, ExtraHead}, Character, LevelTheme}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
pub struct ToolExtraHead {}

impl Tool for ToolExtraHead {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(x, y, extra_head::LAYER) {
			return vec![];
		}

		drop(level);

		sound::play(command_sender, sound::SE_HEADPLUS);
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(ExtraHead::new(x, y))))]
    }

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		self.use_start(command_sender, editor, x, y)
	}

	fn sprite(&self, _theme: LevelTheme, character: Character) -> i32 {
		match character {
			Character::Banki => sprite::HEAD,
			c => sprite::CIRNO_HEAD - 1 + c as i32
		}
	}
}