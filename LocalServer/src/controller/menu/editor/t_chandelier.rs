use crate::controller::{command_handler::CommandSender, level::{chandelier::{self, Chandelier}, Character, LevelTheme}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
pub struct ToolChandelier {}

impl Tool for ToolChandelier {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.round() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(x, y, chandelier::LAYER) || level.any_type_match(x - 1, y, chandelier::LAYER) {
			return vec![];
		}

		drop(level);

		sound::play(command_sender, sound::SE_CHANDELIER2);
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(Chandelier { x: x - 1, y })))]
    }

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::CHANDELIER_TOOL
	}
}