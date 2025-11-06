use crate::controller::{command_handler::{CommandSender, Command, CommandOutput}, event::Event, global_state::ControllerGlobalState, self};

use super::Menu;

// TODO

pub struct ErrorMenu {
	pub message: String
}

impl Menu for ErrorMenu {
	fn name(&self) -> &'static str { "Error" }

	fn on_enter(&mut self, command_sender: &mut CommandSender) {
		command_sender.send(Command::Log(format!("Error: {}", self.message)));
		command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
	}

	fn on_event(&mut self, _command_sender: &mut CommandSender, _event: Event, _global_state: &mut ControllerGlobalState) {
		
	}

	fn on_leave(&mut self, _command_sender: &mut CommandSender) {
		
	}
}