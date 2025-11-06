use crate::controller::{command_handler::CommandSender, undo::UndoAction};

use super::tool::Tool;

#[derive(Default)]
pub struct ToolPan {}

impl Tool for ToolPan {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, _x: f32, _y: f32) -> Vec<UndoAction> {
		editor.editor_go.set_real(command_sender, 0, 1.0);
		vec![]
	}

	fn use_end(&self, command_sender: &mut CommandSender, editor: &mut super::Editor) {
		editor.editor_go.set_real(command_sender, 0, 0.0);
	}

	fn clear_selection(&self) -> bool {false}
	fn can_be_used_zoomed_out(&self) -> bool {true}
}