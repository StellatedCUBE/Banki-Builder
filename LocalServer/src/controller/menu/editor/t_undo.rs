use crate::controller::{command_handler::CommandSender, sound, undo::{UndoAction, UndoFrame}};

use super::tool::Tool;

pub struct ToolUndo {
	redo: bool
}

impl Tool for ToolUndo {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, _x: f32, _y: f32) -> Vec<UndoAction> {
		
		if let Some(frame) = (if self.redo { &mut editor.redo_frames } else { &mut editor.undo_frames }).pop() {
			let actions = frame.actions.into_iter().rev().map(|action| action.perform(command_sender, editor)).collect();
			(if self.redo { &mut editor.undo_frames } else { &mut editor.redo_frames }).push(UndoFrame {
				actions
			});
			sound::play(command_sender, if self.redo { sound::SE_HEAD } else { sound::SE_HEAD2 });
		}

		else {
			sound::play(command_sender, sound::SE_NOT);
		}

		vec![]
	}
}

impl ToolUndo {
	pub fn undo() -> Self {
		Self {
			redo: false
		}
	}

	pub fn redo() -> Self {
		Self {
			redo: true
		}
	}
}