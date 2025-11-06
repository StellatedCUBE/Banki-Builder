use crate::controller::{command_handler::CommandSender, level::{tile_split::TileSplit, Character, LevelTheme}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
pub struct ToolTileSplit {}

impl Tool for ToolTileSplit {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let xnear = (x - x.round()).abs() < 0.1;
		let ynear = (y - y.round()).abs() < 0.1;

		if xnear == ynear {
			return vec![];
		}

		let split = if ynear {
			let x = x.floor() as i32;
			let y = y.round() as i32;
			TileSplit { x, y, vertical: false }
		}

		else {
			let x = x.round() as i32;
			let y = y.floor() as i32;
			TileSplit { x, y, vertical: true }
		};

		{
			let mut level = editor.level.lock().unwrap();
			let tmm = level.tile_manager_mut();

			if !tmm.splits.get_mut().unwrap().insert(split) {
				return vec![];
			}

			tmm.set_and_update(command_sender, split.x, split.y, tmm.get(split.x, split.y));

			sound::play(command_sender, sound::SE_HOLD);
		}

		vec![UndoAction::Delete(editor.add(command_sender, Box::from(split)))]
	}

	fn use_frame(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) {
		let actions = self.use_start(command_sender, editor, x, y);
		editor.add_undo_frame(actions);
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::TILE_SPLIT_TOOL
	}
}