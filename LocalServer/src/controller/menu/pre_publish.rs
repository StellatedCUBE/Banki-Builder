use std::sync::{Arc, Mutex};

use crate::controller::{self, command_handler::{Command, CommandOutput, CommandSender}, control_settings::{MenuIntent, MENU_CONTROLS}, event::Event, game_object::{self, GameObject}, global_state::ControllerGlobalState, internal_command::InternalCommand, level::Level, loc::{data::*, Locale}, sound, sprite};

use super::{editor::Editor, play::{Play, PlayingFrom}, Menu};

pub struct PrePublish {
	level: Arc<Mutex<Level>>,
	initial_message: bool,
	downed: bool,
}

impl Menu for PrePublish {
	fn name(&self) -> &'static str { "PrePublish" }

	fn on_enter(&mut self, command_sender: &mut CommandSender) {
		let mut msg = String::new();

		if self.initial_message {
			msg += LOC_PUBLISH_CLEAR_REQUIRED.for_current_locale_static();
		}

		if self.level.lock().unwrap().has_puzzle_piece() {
			if self.initial_message {
				msg += "\n\n\n";
			}

			msg += LOC_PUBLISH_CLEAR_REQUIRED_PP.for_current_locale_static();
		}

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = if self.initial_message { 50.0 } else { 100.0 };
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, &msg);
		text.destroy_server_only();

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 200.0;
		text.create(command_sender);
		if Locale::get() != Locale::EN {
			text.set_real(command_sender, 0, 1.0);
		}
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_PUBLISH_ATTEMPT.for_current_locale_static());
		text.destroy_server_only();

		let mut pointer = GameObject::new(game_object::OBJ_BLANK, 0);
		pointer.x = 190.0;
		pointer.y = 197.0;
		pointer.create(command_sender);
		pointer.set_scale(command_sender, 0.5, 0.5);
		pointer.set_sprite(command_sender, sprite::POINTER);
		pointer.destroy_server_only();

		let mut pointer = GameObject::new(game_object::OBJ_BLANK, 0);
		pointer.x = 288.0;
		pointer.y = 197.0;
		pointer.create(command_sender);
		pointer.set_scale(command_sender, -0.5, 0.5);
		pointer.set_sprite(command_sender, sprite::POINTER);
		pointer.destroy_server_only();
	}

	fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {
		match event {
			Event::KeyDown |
			Event::ButtonDown |
			Event::MouseDown => self.downed = true,

			Event::ButtonUp |
			Event::MouseUp |
			Event::KeyUp => if self.downed {
				if let Some(intent) = MENU_CONTROLS.get_intent(global_state.last_mod_input) {
					match intent {
						MenuIntent::GoBack |
						MenuIntent::Secondary => {
							sound::play(command_sender, sound::SE_CANCEL);
							command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
							InternalCommand::SwitchToMenu(Box::new(Editor::new(self.level.clone(), true))).run();
						}
						
						MenuIntent::Primary => {
							sound::play(command_sender, sound::SE_DECIDE);
							command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
							InternalCommand::SwitchToMenu(Box::new(Play::new(self.level.clone(), PlayingFrom::Publish, false))).run();
						}

						_ => ()
					}
				}
			}

			_ => ()
		}
	}

	fn on_leave(&mut self, _command_sender: &mut CommandSender) {}
}

impl PrePublish {
	pub fn new(level: Arc<Mutex<Level>>, initial_message: bool) -> Box<Self> {
		Box::new(Self {
			level,
			initial_message,
			downed: false,
		})
	}
}