use crate::controller::{command_handler::{Command, CommandOutput}, event::Event, global_state::ControllerGlobalState, internal_command::InternalCommand, level::Character, sound};

use super::{setup_menu::SetupMenu, Menu};

pub struct NotInEditor {}

impl Menu for NotInEditor {
	fn name(&self) -> &'static str { "NotInEditor" }

	fn on_enter(&mut self, command_sender: &mut crate::controller::command_handler::CommandSender) {
		sound::set_bgm(command_sender, 0);
		command_sender.send(Command::SetLevelData(Character::Banki));
		command_sender.send(Command::GotoRoom(128));
	}

	fn on_leave(&mut self, _command_sender: &mut crate::controller::command_handler::CommandSender) {}
	fn on_event(&mut self, _command_sender: &mut crate::controller::command_handler::CommandSender, event: crate::controller::event::Event, _global_state: &mut ControllerGlobalState) {
		if event == Event::RoomLoad {
			InternalCommand::SwitchToMenu(SetupMenu::new()).run();
		}
	}
}