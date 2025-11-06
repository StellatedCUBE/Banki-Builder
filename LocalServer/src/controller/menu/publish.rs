use std::{fs::{remove_file, rename, File}, mem, sync::{atomic::{AtomicU32, Ordering}, Arc, Mutex}, u32, usize};

use banki_common::{id::id_to_code, publish_level::PublishLevelRQ};

use crate::controller::{self, command_handler::{Command, CommandOutput, CommandSender}, control_settings::{MenuIntent, MENU_CONTROLS}, event::Event, fs, game_object::{self, GameObject}, global_state::ControllerGlobalState, internal_command::InternalCommand, level::{tag::Tag, Level, AABB}, loc::{data::*, tag_data::tag_name, Locale}, net::{query, user}, sound, sprite};

use super::{editor::Editor, main_menu::MainMenu, setup_menu::ok, Menu};

#[derive(Clone, Copy)]
enum Action {
	None,
	ToggleTag(Tag),
	Publish,
	Editor,
	Menu,
}

struct SelectableItem {
	object: GameObject,
	bounds: AABB,
	up: usize,
	down: usize,
	left: usize,
	right: usize,
	action: Action,
}

pub struct Publish {
	level: Arc<Mutex<Level>>,
	downed: bool,
	menu_objects: Vec<GameObject>,
	cursor: GameObject,
	selection: GameObject,
	selected_item: usize,
	selectable_items: Vec<SelectableItem>,
	channel: Arc<AtomicU32>,
}

impl Menu for Publish {
	fn name(&self) -> &'static str { "Publish" }

	fn on_enter(&mut self, command_sender: &mut CommandSender) {
		self.cursor.create(command_sender);
		self.page_1(command_sender);
	}

	fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {
		match event {
			Event::ButtonDown |
			Event::MouseDown |
			Event::KeyDown => self.downed = true,

			Event::MouseMove => {
				if self.cursor.sprite == -1 {
					self.cursor.set_sprite(command_sender, sprite::CURSOR);
				}

				for (i, si) in self.selectable_items.iter().enumerate() {
					if si.bounds.contains(global_state.mouse_x, global_state.mouse_y) {
						self.select(command_sender, i);
						return;
					}
				}

				self.select(command_sender, usize::MAX);
			}

			Event::ButtonUp |
			Event::MouseUp |
			Event::KeyUp => if self.downed {
				if let Some(intent) = MENU_CONTROLS.get_intent(global_state.last_mod_input) {
					match intent {
						MenuIntent::SelectUp => self.select_hide(
							command_sender,
							self.selectable_items.get(self.selected_item)
							.map(|si| si.up)
							.unwrap_or(usize::MAX - 1)
						),
						MenuIntent::SelectDown => self.select_hide(
							command_sender,
							self.selectable_items.get(self.selected_item)
							.map(|si| si.down)
							.unwrap_or(usize::MAX - 1)
						),
						MenuIntent::SelectLeft => self.select_hide(
							command_sender,
							self.selectable_items.get(self.selected_item)
							.map(|si| si.left)
							.unwrap_or(usize::MAX - 1)
						),
						MenuIntent::SelectRight => self.select_hide(
							command_sender,
							self.selectable_items.get(self.selected_item)
							.map(|si| si.right)
							.unwrap_or(usize::MAX - 1)
						),
						MenuIntent::Primary => if let Some(action) = self.selectable_items
							.get(self.selected_item).map(|si| si.action) {
							self.act(command_sender, global_state, action);
						}
						MenuIntent::GoBack => self.act(command_sender, global_state, Action::Editor),
						_ => ()
					}
				}
			}

			Event::Tick => {
				let channel = self.channel.load(Ordering::Relaxed);
				if channel != u32::MAX {
					self.channel.store(u32::MAX, Ordering::Relaxed);

					{
						let mut level = self.level.lock().unwrap();
						let path = level.get_filepath().to_path_buf();
						let tpath = path.with_extension("tmp");
						if let Ok(mut file) = File::create(&tpath) {
							let _ = if level.serialize(&mut file).is_ok() {
								rename(tpath, path)
							} else {
								remove_file(tpath)
							};
						}
					}

					self.clear(command_sender);
					if channel == u32::MAX - 1 {
						self.page_fail(command_sender);
					} else {
						self.page_success(command_sender, channel);
					}
				}
			}

			_ => ()
		}
	}

	fn on_leave(&mut self, command_sender: &mut CommandSender) {
		command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
	}
}

impl Publish {
	pub fn new(level: Arc<Mutex<Level>>) -> Box<Self> {
		Box::new(Self {
			level,
			downed: false,
			menu_objects: vec![],
			cursor: GameObject::new(game_object::OBJ_CURSOR, -15000),
			selection: GameObject::new(game_object::OBJ_YELLOW_BOX, -10000),
			selected_item: usize::MAX,
			selectable_items: vec![],
			channel: Arc::new(AtomicU32::new(u32::MAX)),
		})
	}

	fn page_loading(&mut self, command_sender: &mut CommandSender) {
		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 120.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_PUBLISH_UPLOADING.for_current_locale_static());
		self.menu_objects.push(text);

		let mut throbber = GameObject::new(game_object::OBJ_THROBBER, 0);
		throbber.x = 240.0;
		throbber.y = 160.0;
		throbber.create(command_sender);
		self.menu_objects.push(throbber);
	}

	fn page_1(&mut self, command_sender: &mut CommandSender) {
		self.downed = false;

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 8.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_scale(command_sender, 1.0, 1.0);
		text.set_string(command_sender, 0, LOC_TAG_HEADER.for_current_locale_static());
		self.menu_objects.push(text);

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 48.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_TAG_MANDATORY.for_current_locale_static());
		self.menu_objects.push(text);

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 100.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_TAG_OTHER.for_current_locale_static());
		self.menu_objects.push(text);

		self.tag(command_sender, Tag::SpeedrunTechniques);
		self.tag(command_sender, Tag::Troll);
		self.tag(command_sender, Tag::Hax);

		self.tag(command_sender, Tag::Puzzle);
		self.tag(command_sender, Tag::Platforming);
		self.tag(command_sender, Tag::Speedrun);
		self.tag(command_sender, Tag::Short);
		self.tag(command_sender, Tag::Long);
		self.tag(command_sender, Tag::Music);

		let sil = self.selectable_items.len();
		for si in &mut self.selectable_items[sil - 3..] {
			si.down = sil;
		}

		let mut submit = GameObject::new(game_object::OBJ_BLANK, 1);
		submit.x = 210.0;
		submit.y = 180.0;
		submit.create(command_sender);
		submit.set_sprite(command_sender, sprite::PUBLISH);
		self.selectable_items.push(SelectableItem {
			object: submit,
			bounds: AABB {
				x: 210.0,
				y: 180.0,
				width: 59.0,
				height: 59.0
			},
			up: sil - 2,
			down: usize::MAX,
			left: usize::MAX,
			right: usize::MAX,
			action: Action::Publish,
		});
		
		let mut submit_text = GameObject::new(game_object::OBJ_TEXT, 0);
		submit_text.x = 240.0;
		submit_text.y = 225.0;
		submit_text.create(command_sender);
		submit_text.set_real(command_sender, 1, 1.0);
		submit_text.set_string(command_sender, 0, LOC_PUBLISH_COMMIT.for_current_locale_static());
		self.menu_objects.push(submit_text);
	}

	fn tag(&mut self, command_sender: &mut CommandSender, tag: Tag) {
		let mut level = self.level.lock().unwrap();
		let locked = level.force_tag(tag);
		if locked {
			level.tags |= tag.bit();
		}

		let mut object = GameObject::new(game_object::OBJ_TAG_BUTTON, 0);
		let x = (40 + self.selectable_items.len() % 3 * 140) as f32;
		let y = if tag.mandatory() { 76.0 } else {
			(92 + self.selectable_items.len() / 3 * 24) as f32
		};
		object.x = x;
		object.y = y;
		object.create(command_sender);
		object.set_string(command_sender, 0, tag_name(tag));
		if level.tags & tag.bit() > 0 {
			object.set_real(command_sender, 0, 1.0);
		}
		self.selectable_items.push(SelectableItem {
			object,
			bounds: AABB {
				x,
				y,
				width: 120.0,
				height: 16.0
			},
			up: self.selectable_items.len().checked_sub(3).unwrap_or(usize::MAX),
			down: self.selectable_items.len() + 3,
			left: if self.selectable_items.len() % 3 == 0 { usize::MAX } else { self.selectable_items.len() - 1 },
			right: if self.selectable_items.len() % 3 == 2 { usize::MAX } else { self.selectable_items.len() + 1 },
			action: if locked { Action::None } else { Action::ToggleTag(tag) },
		});
	}

	fn page_fail(&mut self, command_sender: &mut CommandSender) {
		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 100.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_colour(command_sender, 0x5555ff);
		text.set_string(command_sender, 0, LOC_PUBLISH_FAILED.for_current_locale_static());
		text.destroy_server_only();

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 200.0;
		text.create(command_sender);
		if Locale::get() != Locale::EN {
			text.set_real(command_sender, 0, 1.0);
		}
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_PUBLISH_BACK.for_current_locale_static());
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

		self.selectable_items.push(SelectableItem {
			object: pointer,
			bounds: AABB {
				x: -8.0,
				y: -8.0,
				width: 500.0,
				height: 500.0
			},
			up: usize::MAX,
			down: usize::MAX,
			left: usize::MAX,
			right: usize::MAX,
			action: Action::Editor
		});

		self.selected_item = 0;
	}

	fn page_success(&mut self, command_sender: &mut CommandSender, id: u32) {
		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 8.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_scale(command_sender, 1.0, 1.0);
		text.set_string(command_sender, 0, LOC_PUBLISH_SUCCESS_HEADER.for_current_locale_static());
		text.destroy_server_only();

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 48.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(
			command_sender,
			0,
			&LOC_PUBLISH_SUCCESS.for_current_locale_static().replacen(
				'%',
				&self.level.lock().unwrap().name,
				1
			)
		);
		text.destroy_server_only();

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 240.0;
		text.y = 120.0;
		text.create(command_sender);
		text.set_real(command_sender, 1, 1.0);
		text.set_scale(command_sender, 1.0, 1.0);
		text.set_string(command_sender, 0, &id_to_code(id));
		text.destroy_server_only();

		ok(command_sender);

		self.selectable_items.push(SelectableItem {
			object: text,
			bounds: AABB {
				x: -8.0,
				y: -8.0,
				width: 500.0,
				height: 500.0
			},
			up: usize::MAX,
			down: usize::MAX,
			left: usize::MAX,
			right: usize::MAX,
			action: Action::Menu
		});

		self.selected_item = 0;
	}

	fn clear(&mut self, command_sender: &mut CommandSender) {
		for mut go in mem::take(&mut self.menu_objects) {
			go.destroy(command_sender);
		}

		for mut si in mem::take(&mut self.selectable_items) {
			si.object.destroy(command_sender);
		}

		self.select(command_sender, usize::MAX);
	}

	fn select(&mut self, command_sender: &mut CommandSender, item: usize) {
		self.selected_item = item;

		match self.selectable_items.get(item) {
			Some(item) => {
				if !self.selection.exists() {
					self.selection.create(command_sender);
				}

				self.selection.set_real(command_sender, 0, item.bounds.x - 1.0);
				self.selection.set_real(command_sender, 1, item.bounds.y - 1.0);
				self.selection.set_real(command_sender, 2, item.bounds.x + item.bounds.width);
				self.selection.set_real(command_sender, 3, item.bounds.y + item.bounds.height);
			}
			None => self.selection.destroy(command_sender),
		}
	}

	fn select_hide(&mut self, command_sender: &mut CommandSender, mut item: usize) {
		if self.cursor.sprite != -1 {
			self.cursor.set_sprite(command_sender, -1);
		}

		if item == usize::MAX {
			return;
		} else if item == usize::MAX - 1 {
			item = 0;
		}

		self.select(command_sender, item);
	}

	fn act(&mut self, command_sender: &mut CommandSender, global_state: &ControllerGlobalState, action: Action) {
		match action {
			Action::None => sound::play(command_sender, sound::SE_NOT),

			Action::ToggleTag(tag) => {
				let mut level = self.level.lock().unwrap();
				level.tags ^= tag.bit();
				let set = level.tags & tag.bit() > 0;

				if set && tag == Tag::Short {
					level.tags &= !Tag::Long.bit();
					self.selectable_items[self.selected_item + 1].object.set_real(command_sender, 0, 0.0);
				}
				else if set && tag == Tag::Long {
					level.tags &= !Tag::Short.bit();
					self.selectable_items[self.selected_item - 1].object.set_real(command_sender, 0, 0.0);
				}

				self.selectable_items[self.selected_item].object.set_real(
					command_sender,
					0,
					set as u8 as f32
				);
				sound::play(command_sender, if set { sound::SE_DECIDE } else { sound::SE_CANCEL });
			}

			Action::Publish => {
				sound::play(command_sender, sound::SE_DECIDE);
				self.clear(command_sender);
				self.page_loading(command_sender);

				let level_mutex = self.level.clone();
				let channel = self.channel.clone();
				let verification_time = global_state.level_clear_time;
				tokio::spawn(async move {
					channel.store(async move {
						let mut buf = vec![];
						{
							let level = level_mutex.lock().unwrap();
							level.serialize(&mut buf)?;
						}

						let verification_run = fs::get_verify_tas()?;

						let id = query(&PublishLevelRQ {
							level: buf,
							verification_run,
							verification_time
						}).await?;

						let mut level = level_mutex.lock().unwrap();
						level.author = user::me().u();

						if id < u32::MAX - 1 {
							level.online_id = id;
						}

						anyhow::Result::<u32>::Ok(id)
					}.await.inspect_err(|e| println!("Error when publishing: {}", e))
					.unwrap_or(u32::MAX - 1), Ordering::Relaxed);
				});
			}

			Action::Editor => {
				sound::play(command_sender, sound::SE_CANCEL);
				InternalCommand::SwitchToMenu(Box::new(Editor::new(self.level.clone(), true))).run();
			}

			Action::Menu => {
				sound::play(command_sender, sound::SE_DECIDE);
				GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
				GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);

				for _ in 0..120 {
					command_sender.send(Command::Yield);
				}

				InternalCommand::SwitchToMenu(MainMenu::new()).run();
			}
		}
	}
}