use std::{sync::{atomic::{AtomicBool, AtomicU8, Ordering}, Arc}, time::Duration};

use banki_common::update_user_properties::UpdateUserPropertiesRQ;
use tokio::time::sleep;

use crate::controller::{self, command_handler::{Command, CommandOutput, CommandSender}, config, control_settings::{MenuIntent, MENU_CONTROLS}, event::Event, game_object::{self, GameObject}, global_state::ControllerGlobalState, internal_command::InternalCommand, loc::{data::*, Locale}, net::{self, openid::{get_openid_link, is_token_pending}, query, AuthStatus}, sound, sprite};

use super::{main_menu::MainMenu, not_in_editor::NotInEditor, Menu};

enum EoI {
	E(Event),
	I(MenuIntent)
}

enum Page {
	Start,
	NotConnected,
	Connecting,
	NeedSteamAuth([GameObject; 3], Arc<AtomicBool>),
	OutOfDate,
	NeedName(GameObject, Arc<AtomicU8>),
}

impl Page {
	fn is_start(&self) -> bool {
		match self {
			Self::Start => true,
			_ => false
		}
	}
}

pub struct SetupMenu {
	page: Page,
	downed: bool,
	bad_name: bool,
}

impl Menu for SetupMenu {
	fn name(&self) -> &'static str { "Setup" }

	fn on_enter(&mut self, command_sender: &mut CommandSender) {
		sound::set_bgm(command_sender, 0);
		self.step(command_sender);
	}

	fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {
		let eoi = match event {
			Event::ButtonDown |
			Event::KeyDown |
			Event::MouseDown => {
				self.downed = true;
				EoI::E(event)
			}

			Event::ButtonUp |
			Event::KeyUp |
			Event::MouseUp => if self.downed {
				match MENU_CONTROLS.get_intent(global_state.last_mod_input) {
					Some(intent) => EoI::I(intent),
					None => EoI::E(event)
				}
			} else { EoI::E(event) }

			event => EoI::E(event)
		};

		match (&mut self.page, eoi) {
			(Page::OutOfDate, EoI::I(MenuIntent::GoBack)) |
			//(Page::NeedName(_, _), EoI::I(MenuIntent::GoBack)) |
			//(Page::NeedSteamAuth(_), EoI::I(MenuIntent::GoBack)) |
			(Page::NotConnected, EoI::I(MenuIntent::GoBack)) => self.quit(command_sender),

			(Page::OutOfDate, EoI::I(MenuIntent::Primary)) |
			(Page::NotConnected, EoI::I(MenuIntent::Primary)) => {
				sound::play(command_sender, sound::SE_DECIDE);
				command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
				InternalCommand::SwitchToMenu(MainMenu::new()).run();
			}

			(Page::Connecting, EoI::E(Event::Tick)) => if *net::AUTH_STATUS.lock().unwrap() != AuthStatus::Trying {
				self.step(command_sender);
			}

			(Page::NeedName(_, channel), EoI::E(Event::Tick)) => match channel.load(Ordering::Relaxed) {
				1 => {
					net::NEED_UPDATE_NAME.store(false, Ordering::Relaxed);
					self.step(command_sender);
				}
				2 => {
					self.bad_name = true;
					self.step(command_sender);
				}
				3 => {
					*net::AUTH_STATUS.lock().unwrap() = AuthStatus::Failed;
					self.step(command_sender);
				}
				_ => ()
			}
			(Page::NeedName(input, _), EoI::E(Event::InputUnfocus)) => if input.exists() {
				if global_state.recieved_real == 3.0 {
					sound::play(command_sender, sound::SE_DECIDE);
					input.query_string(command_sender, 0);
					input.destroy_server_only();
				} else if global_state.recieved_real == 4.0 {
					self.quit(command_sender);
				}
			}
			(Page::NeedName(_, channel), EoI::E(Event::GetString)) => {
				let name = global_state.recieved_string.trim().to_owned();
				let channel = channel.clone();
				tokio::spawn(async move {
					channel.store(match query(&UpdateUserPropertiesRQ {
						name
					}).await {
						Ok(true) => 1,
						Ok(false) => 2,
						Err(_) => 3
					}, Ordering::Relaxed);
				});
			}
			(Page::NeedName(input, _), EoI::I(MenuIntent::Primary)) => if event == Event::ButtonUp && input.exists() {
				sound::play(command_sender, sound::SE_DECIDE);
				input.query_string(command_sender, 0);
				input.destroy_server_only();
			}

			(Page::NeedSteamAuth(button, channel), EoI::I(MenuIntent::Primary)) => if button[0].exists() {
				sound::play(command_sender, sound::SE_DECIDE);
				for go in button {
					go.destroy(command_sender);
				}
				let channel = channel.clone();
				tokio::spawn(async move {
					if let Ok(oidl) = get_openid_link().await {
						let _ = webbrowser::open(&oidl.link);
						config::get_mut().auth_token = Some(oidl.auth_token);
						config::save();
						*net::AUTH_STATUS.lock().unwrap() = AuthStatus::NotTried;

						while is_token_pending(oidl.token_id).await {
							sleep(Duration::from_secs(1)).await;
						}

						net::setup().await;
					} else {
						*net::AUTH_STATUS.lock().unwrap() = AuthStatus::Failed;
					}

					channel.store(true, Ordering::Relaxed);
				});
			}
			(Page::NeedSteamAuth(_, _), EoI::I(MenuIntent::GoBack)) => {
				*net::AUTH_STATUS.lock().unwrap() = AuthStatus::Failed;
				sound::play(command_sender, sound::SE_CANCEL);
				self.step(command_sender);
			}
			(Page::NeedSteamAuth(_, channel), EoI::E(Event::Tick)) => if channel.load(Ordering::Relaxed) {
				self.step(command_sender);
			}

			_ => ()
		}
	}

	fn on_leave(&mut self, _command_sender: &mut CommandSender) {}
}

impl SetupMenu {
	pub fn new() -> Box<Self> {
		Box::new(Self {
			page: Page::Start,
			downed: false,
			bad_name: false,
		})
	}

	fn step(&mut self, command_sender: &mut CommandSender) {
		if !self.page.is_start() {
			GameObject::clear_all();
			command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
			self.downed = false;
		}

		match *net::AUTH_STATUS.lock().unwrap() {
			AuthStatus::Failed |
			AuthStatus::NotTried => {
				if net::OUT_OF_DATE.load(Ordering::Relaxed) {
					self.page_out_of_date(command_sender);
				} else {
					self.page_no_connection(command_sender);
				}
			}
			AuthStatus::NeedSteamAuth => self.page_need_steam_auth(command_sender),
			AuthStatus::Trying => self.page_connecting(command_sender),
			AuthStatus::Authed(_) => {
				if net::NEED_UPDATE_NAME.load(Ordering::Relaxed) {
					self.page_need_name(command_sender);
				}

				else {
					InternalCommand::SwitchToMenu(MainMenu::new()).run();
				}
			}
		}
	}

	fn page_no_connection(&mut self, command_sender: &mut CommandSender) {
		if config::get().no_warn_offline {
			InternalCommand::SwitchToMenu(MainMenu::new()).run();
			return;
		}

		self.page = Page::NotConnected;

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 100.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_colour(command_sender, 0x5555ff);
		text.set_string(command_sender, 0, LOC_NOT_CONNECTED.for_current_locale_static());
		text.destroy_server_only();

		ok(command_sender);
	}

	fn page_need_steam_auth(&mut self, command_sender: &mut CommandSender) {
		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 100.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_NEED_STEAM_AUTH.for_current_locale_static());
		text.destroy_server_only();

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 200.0;
		text.create(command_sender);
		if Locale::get() != Locale::EN {
			text.set_real(command_sender, 0, 1.0);
		}
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_OPEN_BROWSER.for_current_locale_static());

		let mut pointer = GameObject::new(game_object::OBJ_BLANK, 0);
		pointer.x = 160.0;
		pointer.y = 197.0;
		pointer.create(command_sender);
		pointer.set_scale(command_sender, 0.5, 0.5);
		pointer.set_sprite(command_sender, sprite::POINTER);

		let mut pointer2 = GameObject::new(game_object::OBJ_BLANK, 0);
		pointer2.x = 318.0;
		pointer2.y = 197.0;
		pointer2.create(command_sender);
		pointer2.set_scale(command_sender, -0.5, 0.5);
		pointer2.set_sprite(command_sender, sprite::POINTER);

		self.page = Page::NeedSteamAuth([text, pointer, pointer2], Arc::default());
	}

	fn page_connecting(&mut self, command_sender: &mut CommandSender) {
		self.page = Page::Connecting;

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 100.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_CONNECTING.for_current_locale_static());
		text.destroy_server_only();

		let mut throbber = GameObject::new(game_object::OBJ_THROBBER, 0);
		throbber.x = 240.0;
		throbber.y = 140.0;
		throbber.create(command_sender);
		throbber.destroy_server_only();
	}

	fn page_out_of_date(&mut self, command_sender: &mut CommandSender) {
		self.page = Page::OutOfDate;

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 100.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_OUT_OF_DATE.for_current_locale_static());
		text.destroy_server_only();

		ok(command_sender);
	}

	fn page_need_name(&mut self, command_sender: &mut CommandSender) {
		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 80.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_ENTER_NAME.for_current_locale_static());
		text.destroy_server_only();

		if self.bad_name {
			sound::play(command_sender, sound::SE_NOT);
			let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
			text.x = 240.0;
			text.y = 110.0;
			text.create(command_sender);
			text.set_real(command_sender, 0, 1.0);
			text.set_real(command_sender, 1, 1.0);
			text.set_colour(command_sender, 0x5555ff);
			text.set_string(command_sender, 0, LOC_BAD_NAME.for_current_locale_static());
			text.destroy_server_only();
		}

		let mut backing = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1);
		backing.x = 80.0;
		backing.y = 140.0;
		backing.create(command_sender);
		backing.set_real(command_sender, 0, 0xffffff as f32);
		backing.set_real(command_sender, 1, 320.0);
		backing.set_real(command_sender, 2, 34.0);
		backing.destroy_server_only();

		let mut input = GameObject::new(game_object::OBJ_TEXTBOX, 0);
		input.x = 82.0;
		input.y = 157.0;
		input.create(command_sender);
		input.set_real(command_sender, 0, 1.0);
		input.set_real(command_sender, 1, 24.0);
		input.set_real(command_sender, 2, 160.0);
		input.set_real(command_sender, 4, 1.0);

		self.page = Page::NeedName(input, Arc::default());
	}

	fn quit(&mut self, command_sender: &mut CommandSender) {
		self.page = Page::Start;

		sound::play(command_sender, sound::SE_CANCEL);
		GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -2).create(command_sender);
		GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -2).create(command_sender);

		for _ in 0..120 {
			command_sender.send(Command::Yield);
		}

		InternalCommand::SwitchToMenu(Box::new(NotInEditor{})).run();
	}
}

pub fn ok(command_sender: &mut CommandSender) {
	let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
	text.x = 240.0;
	text.y = 200.0;
	text.create(command_sender);
	text.set_real(command_sender, 1, 1.0);
	text.set_string(command_sender, 0, "OK");
	text.destroy_server_only();

	let mut pointer = GameObject::new(game_object::OBJ_BLANK, 0);
	pointer.x = 210.0;
	pointer.y = 197.0;
	pointer.create(command_sender);
	pointer.set_scale(command_sender, 0.5, 0.5);
	pointer.set_sprite(command_sender, sprite::POINTER);
	pointer.destroy_server_only();

	let mut pointer = GameObject::new(game_object::OBJ_BLANK, 0);
	pointer.x = 268.0;
	pointer.y = 197.0;
	pointer.create(command_sender);
	pointer.set_scale(command_sender, -0.5, 0.5);
	pointer.set_sprite(command_sender, sprite::POINTER);
	pointer.destroy_server_only();
}