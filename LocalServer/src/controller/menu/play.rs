use std::{num::NonZeroU32, sync::{Arc, Mutex}};

use banki_common::{set_time::SetTimeRQ, set_vote::Vote};

use crate::controller::{self, command_handler::{self, Command, CommandOutput, CommandRecorder, CommandSender}, control_settings::{MenuIntent, MENU_CONTROLS}, event::Event, fs::{del_run_tas, get_run_tas, RUN_TAS_FILENAME, VERIFY_TAS_FILENAME}, game_object::{self, GameObject}, global_state::ControllerGlobalState, internal_command::InternalCommand, level::Level, loc::data::LOC_SRT_WARNING, net::{level::{get_vote, pb, set_time, vote, wr}, logged_in, query, user::{is_me, me}}, raw_input::RawInput, sound, sprite};

use super::{editor::Editor, main_menu::MainMenu, pre_publish::PrePublish, publish::Publish, shape::Shape, Menu};

#[derive(Clone, Copy, PartialEq)]
pub enum PlayingFrom {
	Editor,
	MyLevels,
	Publish,
	BrowseLevels,
}

pub struct Play {
	pub level: Arc<Mutex<Level>>,
	done: bool,
	from: PlayingFrom,
	cursor_go: GameObject,
	cursor_timer: u32,
	reset: bool,
	pp: Option<bool>,
	level_id: u32,
	ninth_head_mode: bool,
	liked: bool,
}

impl Menu for Play {
	fn name(&self) -> &'static str { "Play" }

	fn on_enter(&mut self, command_sender: &mut CommandSender) {
		GameObject::clear_all();

		command_sender.send(Command::NinthHead(self.ninth_head_mode));

		command_sender.send(Command::WriteTAS(match self.from {
			PlayingFrom::Publish => VERIFY_TAS_FILENAME,
			PlayingFrom::BrowseLevels => RUN_TAS_FILENAME,
			_ => ""
		}.to_owned()));

		let mut recorder = CommandRecorder::new();

		let level = self.level.lock().unwrap();
		level.load(&mut recorder);
		self.level_id = level.online_id;
		self.liked = get_vote(self.level_id) == Vote::Like;

		let pb = if self.from == PlayingFrom::BrowseLevels && logged_in() {
			pb(self.level_id).map(|t| t.get()).unwrap_or(359999)
		} else { 0 };
		command_sender.send(Command::F32(vec![
			(pb / 3600) as f32,
			(pb % 3600) as f32 / 60.0,
			pb as f32,
			wr(self.level_id).map(|t| t.get()).unwrap_or(0) as f32,
			if self.ninth_head_mode || is_me(level.author) || !logged_in() {
				-1.0
			} else {
				self.liked as u8 as f32
			}
		]));
		command_sender.send(Command::SetTimes);

		if self.from == PlayingFrom::Editor {
			let mut edit_button = GameObject::new(game_object::OBJ_UI, -14000);
			edit_button.create(&mut recorder);
			edit_button.set_real(&mut recorder, 0, 464.0);
			edit_button.set_real(&mut recorder, 1, 4.0);
			edit_button.set_sprite(&mut recorder, sprite::EDIT_BUTTON);
			edit_button.destroy_server_only();
			command_sender.send(Command::F32(vec![1.0]));
			command_sender.send(Command::SetGlobal(command_handler::GLOBAL_QUICKRETRY));
		} else {
			let mut go = GameObject::new(game_object::OBJ_PAUSEMGR, 0);
			go.create(&mut recorder);
			go.destroy_server_only();

			if self.from == PlayingFrom::Publish {
				let mut go = GameObject::new(game_object::OBJ_WATCH_SRT, 0);
				go.create(&mut recorder);
				go.set_string(&mut recorder, 0, LOC_SRT_WARNING.for_current_locale_static());
				go.destroy_server_only();
			}
		}

		self.cursor_go.create(&mut recorder);
		command_sender.send(Command::SetRoomStartCommands(recorder.record));
		command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
	}

	fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {
		match event {
			Event::LevelQuit => {
				self.done = true;
				command_sender.send(Command::SetRoomStartCommands(vec![]));
			}

			Event::LevelCompletePP |
			Event::LevelCompleteNoPP => {
				self.pp = Some(event == Event::LevelCompletePP);
				if self.from == PlayingFrom::BrowseLevels &&
				!self.ninth_head_mode &&
				pb(self.level_id).is_none_or(|t| t.get() > global_state.level_clear_time) {
					if let Some(time) = NonZeroU32::new(global_state.level_clear_time) {
						if let Ok(run) = get_run_tas() {
							let level = self.level_id;
							set_time(level, me(), time);
							tokio::spawn(async move {
								query(&SetTimeRQ {
									level,
									time,
									run
								}).await
							});
						}
					}
				}
				del_run_tas();
			}

			Event::Like => {
				self.liked = !self.liked;
				tokio::spawn(vote(self.level_id, if self.liked {
					Vote::Like
				} else {
					Vote::None
				}));
			}

			Event::RoomLoad => {
				if self.done {
					match self.from {
						PlayingFrom::Editor => {
							command_sender.send(Command::F32(vec![global_state.view_x, global_state.view_y]));
							command_sender.send(Command::MoveCamera);
							InternalCommand::SwitchToMenu(Box::from(Editor::new(self.level.clone(), false))).run();
							sound::set_bgm(command_sender, sound::BGM_EDITOR_MENU);
						}

						PlayingFrom::Publish => {
							sound::set_bgm(command_sender, sound::BGM_EDITOR_MENU);
							InternalCommand::SwitchToMenu(if self.pp.is_none() {
								Box::from(Editor::new(self.level.clone(), true))
							} else if self.pp == Some(false) && self.level.lock().unwrap().has_puzzle_piece() {
								PrePublish::new(self.level.clone(), false)
							} else {
								Publish::new(self.level.clone())
							}).run();
						}

						PlayingFrom::BrowseLevels |
						PlayingFrom::MyLevels => InternalCommand::SwitchToMenu(MainMenu::new()).run(),
					};
					return;
				}

				else {
					self.reset = true;
				}
			}

			Event::KeyUp |
			Event::ButtonUp |
			Event::MouseUp => {
				if !self.done && self.from == PlayingFrom::Editor && ((
					global_state.last_raw_input == RawInput::Mouse(1) &&
					Shape::Rect(464.0, 4.0, 476.0, 14.0).contains(global_state.mouse_x - global_state.view_x, global_state.mouse_y - global_state.view_y)
				 ) || MENU_CONTROLS.get_intent(global_state.last_mod_input) == Some(MenuIntent::GoBack)) {
					self.done = true;
					sound::play(command_sender, sound::SE_CANCEL);
					command_sender.send(Command::SetRoomStartCommands(vec![]));
					let mut transition = GameObject::new(game_object::OBJ_QUICKRETRY, -15990);
					transition.create(command_sender);
					for _ in 0..30 {
						command_sender.send(Command::Yield);
					}
					command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
				}
			}

			Event::MouseMove => if self.from == PlayingFrom::Editor {
				if global_state.was_mouse_actually_moved {
					if self.cursor_timer == 0 || self.reset {
						self.cursor_go.set_sprite(command_sender, sprite::CURSOR);
						self.reset = false;
					}
					self.cursor_timer = 30;
				}
			}

			Event::Tick => {
				if self.cursor_timer > 0 {
					self.cursor_timer -= 1;
					if self.cursor_timer == 0 {
						self.cursor_go.set_sprite(command_sender, -1);
					}
				}
			}

			Event::SpeedrunTechniques => self.level.lock().unwrap().speedrun_techniques = true,

			_ => ()
		}
	}

	fn on_leave(&mut self, _command_sender: &mut CommandSender) {}
}

impl Play {
	pub fn new(level: Arc<Mutex<Level>>, from: PlayingFrom, ninth_head_mode: bool) -> Self {
		Self {
			level,
			ninth_head_mode,
			done: false,
			from,
			cursor_go: GameObject::new(game_object::OBJ_CURSOR, -15000),
			cursor_timer: 0,
			reset: false,
			pp: None,
			level_id: u32::MAX,
			liked: false,
		}
	}
}