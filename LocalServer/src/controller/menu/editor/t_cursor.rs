use crate::controller::{command_handler::CommandSender, undo::UndoAction};

use super::tool::Tool;

#[derive(Default)]
pub struct ToolCursor {}

impl Tool for ToolCursor {
    fn use_start(&self, _command_sender: &mut CommandSender, _editor: &mut super::Editor, _x: f32, _y: f32) -> Vec<UndoAction> {vec![]}
    fn can_be_used_zoomed_out(&self) -> bool {true}
}