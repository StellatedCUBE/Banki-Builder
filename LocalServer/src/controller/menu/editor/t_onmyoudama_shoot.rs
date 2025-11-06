use crate::controller::{command_handler::CommandSender, level::{onmyoudama_shoot::{self, OnmyoudamaShoot}, Character, LevelTheme}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
pub struct ToolOnmyoudamaShoot {}

impl Tool for ToolOnmyoudamaShoot {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(x, y, onmyoudama_shoot::LAYER) {
			return vec![];
		}

		drop(level);

		sound::play(command_sender, sound::SE_HOLD);
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(OnmyoudamaShoot::new(x, y))))]
    }

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		self.use_start(command_sender, editor, x, y)
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::ONMYOUDAMA_BLUE
	}
}