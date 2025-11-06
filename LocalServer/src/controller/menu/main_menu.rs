use std::{fs::rename, mem, path::PathBuf, sync::{mpsc::{sync_channel, Receiver}, Mutex}};

use banki_common::{id::{code_to_id, id_to_code}, search_levels::{LevelOrdering, SearchLevelsRQ}, set_vote::Vote, unpublish_level::UnpublishLevelRQ};
use rfd::AsyncFileDialog;
use tokio::task::JoinHandle;

use crate::controller::{self, command_handler::{Command, CommandOutput, CommandSender}, control_settings::{MenuIntent, MENU_CONTROLS}, event::Event, fs, game_object::{self, GameObject}, global_state::ControllerGlobalState, internal_command::InternalCommand, level::{database, metadata::LevelMetadata, tag::Tag, tile_manager::Tile, Character, Level, LevelTheme}, loc::{data::*, tag_data::tag_name, Locale}, net::{self, level::{get_vote, likes, vote}, logged_in, query, user}, sound, sprite};

use super::{Menu, shape::Shape, not_in_editor::NotInEditor, editor::Editor};

pub const BROWSE_TABS: usize = 3;
pub const BROWSE_OTHER: usize = 4;

pub const CONTINUOUS_INTENTS: usize = 2;

#[derive(Clone, Copy, PartialEq)]
enum SubMenu {
	Main,
	NewLevel,
	Create,
	Quit,
	LevelExtra,
	Confirm,
	Browse,
}

#[derive(Clone, Copy, PartialEq)]
#[allow(unused)]
enum BrowseTab {
	New,
	Top,
	ById,
}

struct Selectable {
	pub shape: Shape,
	pub left: usize,
	pub right: usize,
	pub up: usize,
	pub down: usize,
}

impl Selectable {
	pub fn new(shape: Shape) -> Self {
		Self {
			shape,
			left: usize::MAX,
			right: usize::MAX,
			up: usize::MAX,
			down: usize::MAX,
		}
	}

	pub fn attach_left(mut self, to: usize) -> Self {
		self.left = to;
		self
	}

	pub fn attach_right(mut self, to: usize) -> Self {
		self.right = to;
		self
	}

	pub fn attach_up(mut self, to: usize) -> Self {
		self.up = to;
		self
	}

	pub fn attach_down(mut self, to: usize) -> Self {
		self.down = to;
		self
	}
}

struct LevelGameObject {
	level_data: LevelMetadata,
	backing: GameObject,
	objects: Vec<(f32, f32, GameObject)>,
	selection: Option<GameObject>,
}

impl LevelGameObject {
	fn new(data: LevelMetadata, command_sender: &mut CommandSender) -> Self {
		let mut backing = GameObject::new(game_object::OBJ_BLANK, 6100);
		let mut theme = GameObject::new(game_object::OBJ_BLANK, 6050);
		let mut name = GameObject::new(game_object::OBJ_TEXT, 6000);
		let mut author = GameObject::new(game_object::OBJ_TEXT, 6000);

		backing.create(command_sender);
		theme.create(command_sender);
		name.create(command_sender);
		author.create(command_sender);

		backing.set_sprite(command_sender, sprite::LEVEL_BANKI + data.character as i32);

		theme.set_sprite(command_sender, match data.theme {
			LevelTheme::Rasobi => sprite::WALL + 1,
			theme => Tile::GroundBase.sprite(theme)
		});

		name.set_real(command_sender, 0, 1.0);
		name.set_real(command_sender, 2, 1.0);
		name.set_real(command_sender, 3, 175.0);
		name.set_string(command_sender, 0, &data.name);

		author.set_real(command_sender, 0, 1.0);
		author.set_real(command_sender, 2, 1.0);
		author.set_real(command_sender, 3, 175.0);
		author.set_string(command_sender, 0, &user::get(data.author).name);

		let mut objects = vec![(8.0, 6.0, theme), (46.0, 9.0, name), (46.0, 22.0, author)];

		if data.tagged(Tag::PuzzlePiece) {
			let mut pp = GameObject::new(game_object::OBJ_TEXT, 6000);
			pp.create(command_sender);
			pp.set_string(command_sender, 0, "<");
			objects.push((45.0, 30.0, pp));
		}

		if data.tagged(Tag::SpeedrunTechniques) {
			let mut st = GameObject::new(game_object::OBJ_BLANK, 6000);
			st.create(command_sender);
			st.set_sprite(command_sender, sprite::ICON_SPEEDRUN);
			objects.push((55.0, 31.0, st));
		}

		if data.tagged(Tag::Troll) {
			let mut t = GameObject::new(game_object::OBJ_BLANK, 6000);
			t.create(command_sender);
			t.set_sprite(command_sender, sprite::ICON_TROLL);
			objects.push((65.0, 31.0, t));
		}

		if data.tagged(Tag::Hax) {
			let mut eh = GameObject::new(game_object::OBJ_BLANK, 6000);
			eh.create(command_sender);
			eh.set_sprite(command_sender, sprite::ICON_HAX);
			objects.push((75.0, 31.0, eh));
		}

		if data.character == Character::Seija {
			for i in 0..4 {
				if data.seija_flags & (1 << i) == 0 {
					let mut no = GameObject::new(game_object::OBJ_BLANK, 6000);
					no.create(command_sender);
					no.set_sprite(command_sender, sprite::SEIJA_NO);
					objects.push((match i {
						0 => 87.0,
						1 => 107.0,
						2 => 97.0,
						3 => 117.0,
						_ => unreachable!()
					}, 31.0, no));
				}
			}
		}

		if let Some(wr) = net::level::wr(data.online_id) {
			let mut time = GameObject::new(game_object::OBJ_TEXT, 6000);
			time.create(command_sender);
			time.set_real(command_sender, 1, 1.0);
			time.set_string(command_sender, 0, &time_text(wr.into()));
			objects.push((183.0, 17.0, time));
		}

		if let Some(pb) = net::level::pb(data.online_id) {
			let mut time = GameObject::new(game_object::OBJ_TEXT, 6000);
			time.create(command_sender);
			time.set_real(command_sender, 1, 1.0);
			time.set_string(command_sender, 0, &time_text(pb.into()));
			objects.push((183.0, 30.0, time));
		}

		if data.online_id != u32::MAX && logged_in() {
			let mut text = GameObject::new(game_object::OBJ_TEXT, 6000);
			text.create(command_sender);
			text.set_real(command_sender, 1, 1.0);
			text.set_string(command_sender, 0, &format!("l{}", likes(data.online_id)));
			objects.push((24.0, 33.0, text));
		}

		Self {
			level_data: data,
			backing,
			objects,
			selection: None,
		}
	}

	fn goto(&mut self, command_sender: &mut CommandSender, x: f32, y: f32) {
		self.backing.x = x;
		self.backing.y = y;

		self.backing.update_position(command_sender);
		
		for (ofs_x, ofs_y, object) in &mut self.objects {
			object.x = x + *ofs_x;
			object.y = y + *ofs_y;
			object.update_position(command_sender);
		}

		if let Some(selection) = self.selection.as_mut() {
			selection.x = x;
			selection.y = y;
			selection.update_position(command_sender);
		}
	}

	fn set_selected(&mut self, command_sender: &mut CommandSender, selected: bool) {
		match (selected, self.selection.as_mut()) {
			(true, None) => {
				let mut go = GameObject::new(game_object::OBJ_BLANK, 6000);
				go.create(command_sender);
				go.set_sprite(command_sender, sprite::LEVEL_SELECT);
				self.selection = Some(go);
			}
			(false, Some(selection)) => {
				selection.destroy(command_sender);
				self.selection = None;
			}
			_ => ()
		}
	}

	fn destroy(mut self, command_sender: &mut CommandSender) {
		self.backing.destroy(command_sender);
		if let Some(mut sel) = self.selection {
			sel.destroy(command_sender);
		}
		for (_, _, mut go) in self.objects {
			go.destroy(command_sender);
		}
	}
}

struct CameraAnim {
	pub start_x: f32,
	pub end_x: f32,
	pub start_y: f32,
	pub end_y: f32,
	pub duration: u32,
}

impl CameraAnim {
	fn t(&self, tick: u32) -> f32 {
		let t = tick as f32 / self.duration as f32;
		let t = t.clamp(0.0, 1.0);

		t * t * (t * (1.125 * t - 3.0) + 2.25) * (1.0 / 0.375)
	}

	pub fn run(&self, command_sender: &mut CommandSender) {
		for i in 1..self.duration + 1 {
			let t = self.t(i);
			let x = self.start_x + (self.end_x - self.start_x) * t;
			let y = self.start_y + (self.end_y - self.start_y) * t;

			command_sender.send(Command::F32(vec![x, y]));
			command_sender.send(Command::MoveCamera);
			command_sender.send(Command::Yield);
		}

		command_sender.send(Command::F32(vec![self.end_x, self.end_y]));
		command_sender.send(Command::MoveCamera);
	}
}

#[derive(Default)]
enum ConfirmAction {
	#[default]
	None,
	DeleteLevel(PathBuf),
	UnpublishLevel,
}

#[derive(Default)]
enum SaveState {
	#[default]
	Main,
	MyLevels,
	Browse(BrowseTab, Vec<LevelMetadata>, usize, f32),
}

static SAVE_STATE: Mutex<SaveState> = Mutex::new(SaveState::Main);

pub struct MainMenu {
	continuous_intents: [ u32; CONTINUOUS_INTENTS],
	selection_go: GameObject,
	cursor_go: GameObject,
	sub_menu: SubMenu,
	selectable_objects: Vec<Selectable>,
	selection: usize,
	camera_anim_ticks: u32,
	name_entry: GameObject,
	next_menu: Option<Box<dyn Menu + Send>>,

	my_levels: Option<Vec<LevelGameObject>>,
	my_levels_scroll: f32,
	selected_level: usize,
	move_selected_level_into_view: bool,
	extra_actions_menu: Vec<GameObject>,
	my_levels_grey_out: [GameObject; 3],

	confirm_action: ConfirmAction,
	confirm_menu: Vec<GameObject>,

	browse_tab_backings: [GameObject; BROWSE_TABS],
	current_browse_tab: BrowseTab,
	search: Option<SearchLevelsRQ>,
	search_channel: Option<Receiver<Vec<LevelMetadata>>>,
	search_results: Vec<LevelGameObject>,
	search_message: GameObject,
	search_scroll: f32,
	tag_objects: Vec<GameObject>,
	received_results: Vec<LevelMetadata>,
	vote_handle: Option<JoinHandle<()>>,
	browse_grey_out: [GameObject; 3],

	drag: Option<(f32, f32, f32)>,
}

impl Menu for MainMenu {
	fn name(&self) -> &'static str { "Main" }

	fn on_enter(&mut self, command_sender: &mut CommandSender) {
		database::load();

		sound::set_bgm(command_sender, sound::BGM_EDITOR_MENU);

		match mem::take(&mut *SAVE_STATE.lock().unwrap()) {
			SaveState::MyLevels => self.sub_menu = SubMenu::Create,
			SaveState::Browse(tab, results, selection, scroll) => {
				self.current_browse_tab = tab;
				self.sub_menu = SubMenu::Browse;
				self.selected_level = selection;
				self.search_scroll = scroll;
				self.move_selected_level_into_view = true;
				let (sender, receiver) = sync_channel(1);
				self.search_channel = Some(receiver);
				sender.send(results).unwrap();
			}
			SaveState::Main => ()
		}

		let mut background = GameObject::new(game_object::OBJ_BLANK, 5000);
		background.create(command_sender);
		background.set_sprite(command_sender, sprite::MAIN_MENU + Locale::get() as i32);
		background.destroy_server_only();

		let mut background = GameObject::new(game_object::OBJ_BLANK, 15000);
		background.x = 60.0;
		background.y = 60.0;
		background.create(command_sender);
		background.set_sprite(command_sender, sprite::CREATE_BG);
		background.destroy_server_only();

		let mut background = GameObject::new(game_object::OBJ_BLANK, 15000);
		background.x = 600.0;
		background.y = 368.0;
		background.create(command_sender);
		background.set_sprite(command_sender, sprite::BROWSE_BG);
		background.destroy_server_only();

		let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
		text.x = 765.0;
		text.y = 70.0;
		text.create(command_sender);
		text.set_real(command_sender, 0, 1.0);
		text.set_real(command_sender, 1, 1.0);
		text.set_string(command_sender, 0, LOC_NAME_LEVEL.for_current_locale_static());
		text.destroy_server_only();

		for (i, backing) in self.browse_tab_backings.iter_mut().enumerate() {
			backing.x = (600 + 80 * i) as f32;
			backing.y = 337.0;
			backing.create(command_sender);
			backing.set_real(command_sender, 1, 69.0);
			backing.set_real(command_sender, 2, 15.0);

			if i == self.current_browse_tab as usize {
				backing.set_real(command_sender, 0, 0xffff as f32);
			} else {
				backing.set_real(command_sender, 0, 0xffffff as f32);
			}

			let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
			text.x = backing.x + 35.0;
			text.y = 338.0;
			text.create(command_sender);
			text.set_real(command_sender, 0, 1.0);
			text.set_real(command_sender, 1, 1.0);
			text.set_colour(command_sender, 0);
			text.set_string(command_sender, 0, LOC_BROWSE_TAB[i].for_current_locale_static());
			text.destroy_server_only();
		}

		let (vx, vy) = Self::menu_pos(self.sub_menu);
		command_sender.send(Command::F32(vec![vx, vy]));
		command_sender.send(Command::MoveCamera);

		self.cursor_go.create(command_sender);

		self.name_entry.x = 588.0;
		self.name_entry.y = 120.0;
		self.name_entry.create(command_sender);
		self.name_entry.set_real(command_sender, 2, 344.0);

		if !logged_in() {
			let mut go = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, -5);
			go.x = 255.0;
			go.y = 390.0;
			go.create(command_sender);
			go.set_real(command_sender, 0, 0x606060 as f32);
			go.set_real(command_sender, 1, 134.0);
			go.set_real(command_sender, 2, 74.0);
			go.set_alpha(command_sender, 0.75);
			go.destroy_server_only();
		}

		self.my_levels_grey_out[0].x = 30.0;
		self.my_levels_grey_out[0].y = 210.0;
		self.my_levels_grey_out[0].create(command_sender);
		self.my_levels_grey_out[0].set_sprite(command_sender, sprite::GREY_CIRCLE);

		self.my_levels_grey_out[1].x = 315.0;
		self.my_levels_grey_out[1].y = 165.0;
		self.my_levels_grey_out[1].create(command_sender);
		self.my_levels_grey_out[1].set_real(command_sender, 0, 0x606060 as f32);
		self.my_levels_grey_out[1].set_real(command_sender, 1, 59.0);
		self.my_levels_grey_out[1].set_real(command_sender, 2, 59.0);
		self.my_levels_grey_out[1].set_alpha(command_sender, 0.75);

		self.my_levels_grey_out[2].x = 390.0;
		self.my_levels_grey_out[2].y = 165.0;
		self.my_levels_grey_out[2].create(command_sender);
		self.my_levels_grey_out[2].set_real(command_sender, 0, 0x606060 as f32);
		self.my_levels_grey_out[2].set_real(command_sender, 1, 59.0);
		self.my_levels_grey_out[2].set_real(command_sender, 2, 59.0);
		self.my_levels_grey_out[2].set_alpha(command_sender, 0.75);

		self.browse_grey_out[0].x = 555.0;
		self.browse_grey_out[0].y = 444.0;
		self.browse_grey_out[0].create(command_sender);
		self.browse_grey_out[0].set_sprite(command_sender, sprite::GREY_CIRCLE);

		self.browse_grey_out[1].x = 848.0;
		self.browse_grey_out[1].y = 495.0;
		self.browse_grey_out[1].create(command_sender);
		self.browse_grey_out[1].set_real(command_sender, 0, 0x606060 as f32);
		self.browse_grey_out[1].set_real(command_sender, 1, 59.0);
		self.browse_grey_out[1].set_real(command_sender, 2, 59.0);
		self.browse_grey_out[1].set_alpha(command_sender, 0.75);

		self.browse_grey_out[2].x = 923.0;
		self.browse_grey_out[2].y = 495.0;
		self.browse_grey_out[2].create(command_sender);
		self.browse_grey_out[2].set_real(command_sender, 0, 0x606060 as f32);
		self.browse_grey_out[2].set_real(command_sender, 1, 59.0);
		self.browse_grey_out[2].set_real(command_sender, 2, 59.0);
		self.browse_grey_out[2].set_alpha(command_sender, 0.75);

		self.load_sub_menu(command_sender);
	}

	fn on_leave(&mut self, command_sender: &mut CommandSender) {
		if self.sub_menu != SubMenu::Quit {
			command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
			*SAVE_STATE.lock().unwrap() = match self.sub_menu {
				SubMenu::Create |
				SubMenu::NewLevel |
				SubMenu::LevelExtra => SaveState::MyLevels,
				SubMenu::Browse => SaveState::Browse(self.current_browse_tab, mem::take(&mut self.received_results), self.selected_level, self.search_scroll),
				_ => SaveState::Main
			}
		}
	}

	fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {
		if event == Event::Tick && self.sub_menu == SubMenu::Create {
			if self.my_levels.is_none() {
				if let Some(levels) = database::get_levels() {
					self.my_levels = Some(levels.into_iter().map(|data| LevelGameObject::new(data, command_sender)).collect());
					self.my_levels_scroll = 0.0;

					for i in 0..self.my_levels.as_ref().unwrap().len() {
						if self.selectable_objects.len() == i + 5 {
							self.selectable_objects.push(Selectable::new(Shape::null()).attach_up(i + 4).attach_down(i + 6).attach_right(0));
						}
					}

					self.set_my_levels_pos(command_sender);
				}
			}

			else if self.move_selected_level_into_view && self.selected_level < self.my_levels.as_ref().unwrap().len() {
				let target_scroll = self.my_levels_scroll
					.max((self.selected_level as isize * 55 - 135) as f32)
					.min((self.selected_level * 55) as f32);

				let old_scroll = self.my_levels_scroll;
				self.my_levels_scroll = if (old_scroll - target_scroll).abs() <= 1.0 {
					self.move_selected_level_into_view = false;
					target_scroll
				}  else {
					old_scroll * 0.85 + target_scroll * 0.15
				};

				if old_scroll.floor() != self.my_levels_scroll.floor() {
					self.set_my_levels_pos(command_sender);
				}
			}
		}

		if self.camera_anim_ticks > 0 && event == Event::Tick {
			self.camera_anim_ticks -= 1;
			if self.camera_anim_ticks == 0 {
				if self.next_menu.is_some() {
					 InternalCommand::SwitchToMenu(mem::take(&mut self.next_menu).unwrap()).run();
					 return;
				}

				self.load_sub_menu(command_sender);
			}
		}

		if self.camera_anim_ticks > 0 {
			return;
		}

		if event == Event::Tick && self.sub_menu == SubMenu::Browse {
			if let Some(results) = self.search_channel.as_ref().and_then(|c| c.try_recv().ok()) {
				self.search_channel = None;
				self.search_message.destroy(command_sender);
				self.received_results = results;
				if self.received_results.len() == 0 {
					self.search_message = GameObject::new(game_object::OBJ_TEXT, 0);
					self.search_message.x = 712.0;
					self.search_message.y = 372.0;
					self.search_message.create(command_sender);
					self.search_message.set_real(command_sender, 0, 1.0);
					self.search_message.set_real(command_sender, 1, 1.0);
					self.search_message.set_string(command_sender, 0, LOC_NO_RESULTS.for_current_locale_static());
				} else {
					self.search_results = self.received_results.iter().map(|data| LevelGameObject::new(data.clone(), command_sender)).collect();
					for i in 0..self.search_results.len() {
						let j = self.selectable_objects.len();
						let mut s = Selectable::new(Shape::null()).attach_left(3).attach_right(1).attach_up(
						if i > 0 {
								j - 1
							} else {
								BROWSE_OTHER + self.current_browse_tab as usize
							}
						);
						if i < self.search_results.len() - 1 {
							s = s.attach_down(j + 1);
						}
						self.selectable_objects.push(s);
					}
					if self.selected_level == usize::MAX {
						self.search_scroll = 0.0;
					} else {
						self.show_browse_level_metadata(command_sender);
						if self.tag_objects.len() > 0 {
							self.selection =  self.selected_level + BROWSE_TABS + BROWSE_OTHER;
						}
					}
					self.set_browse_levels_pos(command_sender);
				}
			}

			else if self.move_selected_level_into_view && self.selected_level < self.search_results.len() {
				let target_scroll = self.search_scroll
					.max((self.selected_level as isize * 55 - 139) as f32)
					.min((self.selected_level * 55) as f32);

				let old_scroll = self.search_scroll;
				self.search_scroll = if (old_scroll - target_scroll).abs() <= 1.0 {
					self.move_selected_level_into_view = false;
					target_scroll
				}  else {
					old_scroll * 0.85 + target_scroll * 0.15
				};

				if old_scroll.floor() != self.search_scroll.floor() {
					self.set_browse_levels_pos(command_sender);
				}
			}

			if self.vote_handle.as_ref().is_some_and(|h| h.is_finished()) {
				if let Some(lgo) = self.search_results.get_mut(self.selected_level) {
					let mut nlgo = LevelGameObject::new(lgo.level_data.clone(), command_sender);
					mem::swap(lgo, &mut nlgo);
					nlgo.destroy(command_sender);
					self.set_browse_levels_pos(command_sender);
				}
				for mut go in mem::take(&mut self.tag_objects) {
					go.destroy(command_sender);
				}
				self.show_browse_level_metadata(command_sender);
			}
		}

		match event {
			Event::MouseMove => {
				//if global_state.mouse_dx != 0.0 && global_state.mouse_dy != 0.0 && (global_state.mouse_x != global_state.mouse_dx || global_state.mouse_y != global_state.mouse_dy) {
				if global_state.was_mouse_actually_moved {
					if self.cursor_go.sprite == -1 {
						self.cursor_go.set_sprite(command_sender, sprite::CURSOR);
					}

					self.set_selection_by_mouse(command_sender, global_state);
				}

				if let Some((mouse_start, scroll_start, scroll_max)) = self.drag {
					self.move_selected_level_into_view = false;
					self.set_scroll(command_sender, scroll_max.min(scroll_start + mouse_start - global_state.mouse_y).max(0.0));
				}
			}
			Event::GetString => match self.sub_menu {
				SubMenu::NewLevel => {
					let name = global_state.recieved_string.trim();
					if Level::is_name_valid(name) {
						sound::play(command_sender, sound::SE_STAGEDECIDE);

						GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
						GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);
						self.camera_anim_ticks = 120;

						let level = Level::new(name.to_owned());
						self.next_menu = Some(Box::from(Editor::new(level, true)));
					} else {
						sound::play(command_sender, sound::SE_NOT);
					}
				}
				SubMenu::Browse => {
					let id = code_to_id(&global_state.recieved_string);
					if id == u32::MAX {
						sound::play(command_sender, sound::SE_NOT);
					} else {
						sound::play(command_sender, sound::SE_DECIDE);
						self.search = Some(SearchLevelsRQ {
							id,
							order: LevelOrdering::New,
							tags: 0,
							neg_tags: 0,
							characters: 0,
							themes: 0,
						});
						self.selection = BROWSE_OTHER + BrowseTab::ById as usize;
						self.update_selection_go(command_sender);
						self.load_browse_results(command_sender);
					}
				}
				_ => ()
			}
			Event::MouseDown |
			Event::ButtonDown |
			Event::KeyDown => if let Some(intent) = MENU_CONTROLS.get_intent(global_state.last_mod_input) {
				if (intent as usize) < CONTINUOUS_INTENTS {
					self.continuous_intents[intent as usize] = 1;
				}

				if self.cursor_go.sprite != -1 && intent as usize >= CONTINUOUS_INTENTS {
					self.cursor_go.set_sprite(command_sender, -1);
				}

				let sel_len = self.selectable_objects.len();

				match intent {
					MenuIntent::SelectUp => self.selection = self.selectable_objects.get(self.selection).and_then(|o| Some(if o.up < sel_len {o.up} else {self.selection})).unwrap_or_default(),
					MenuIntent::SelectDown => self.selection = self.selectable_objects.get(self.selection).and_then(|o| Some(if o.down < sel_len {o.down} else {self.selection})).unwrap_or_default(),
					MenuIntent::SelectLeft => self.selection = self.selectable_objects.get(self.selection).and_then(|o| Some(if o.left < sel_len {o.left} else {self.selection})).unwrap_or_default(),
					MenuIntent::SelectRight => self.selection = self.selectable_objects.get(self.selection).and_then(|o| Some(if o.right < sel_len {o.right} else {self.selection})).unwrap_or_default(),
					MenuIntent::Primary => {
						self.clicked(command_sender);

						if event == Event::MouseDown {
							if let Some((current, max, area)) = self.scroll() {
								if area.contains(global_state.mouse_x, global_state.mouse_y) {
									self.drag = Some((global_state.mouse_y, current, max));
								}
							}
						}
					}
					MenuIntent::Secondary => self.right_clicked(command_sender),
					MenuIntent::GoBack => {
						self.selection = match self.sub_menu {
							SubMenu::Main => 2,
							SubMenu::Create => 3,
							SubMenu::NewLevel => 1,
							SubMenu::Browse => 0,
							_ => usize::MAX
						};
						self.clicked(command_sender);
						return;
					}
					_ => return
				}

				self.update_selection_go(command_sender);

				if (
					(self.sub_menu == SubMenu::Create && self.selection > 4) ||
					(self.sub_menu == SubMenu::Browse && self.selection >= BROWSE_OTHER)
					) && intent.is_direction() {
					self.clicked(command_sender);
				}
			}
			Event::MouseUp |
			Event::ButtonUp |
			Event::KeyUp => if let Some(intent) = MENU_CONTROLS.get_intent(global_state.last_mod_input) {
				self.drag = None;

				if (intent as usize) < CONTINUOUS_INTENTS {
					if self.continuous_intents[intent as usize] < 3 {
						if let Some((current, max, area)) = self.scroll() {
							if self.cursor_go.sprite == -1 || area.contains(global_state.mouse_x, global_state.mouse_y) {
								match intent {
									MenuIntent::ScrollUp => self.set_scroll(command_sender, (current - 20.0).max(0.0)),
									MenuIntent::ScrollDown => self.set_scroll(command_sender, (current + 20.0).min(max)),
									_ => ()
								}
							}
						}
					}

					self.continuous_intents[intent as usize] = 0;
				}
			}
			Event::InputUnfocus => if global_state.recieved_real == 3.0 {
				match self.sub_menu {
					SubMenu::NewLevel => {
						self.selection = 2;
						self.clicked(command_sender);
					}
					SubMenu::Browse => {
						self.selection = BROWSE_OTHER + BROWSE_TABS + 1;
						self.clicked(command_sender);
					}
					_ => ()
				}
			}
			Event::Tick => for i in 0..CONTINUOUS_INTENTS {
				if self.continuous_intents[i] > 0 {
					if let Some((current, max, area)) = self.scroll() {
						if self.continuous_intents[i] > 1 && (self.cursor_go.sprite == -1 || area.contains(global_state.mouse_x, global_state.mouse_y)) {
							const SCROLL_UP: usize = MenuIntent::ScrollUp as usize;
							const SCROLL_DOWN: usize = MenuIntent::ScrollDown as usize;

							match i {
								SCROLL_UP => self.set_scroll(command_sender, (current - 1.0).max(0.0)),
								SCROLL_DOWN => self.set_scroll(command_sender, (current + 1.0).min(max)),
								_ => ()
							}
						}
					}
					self.continuous_intents[i] += 1;
				}
			}
			_ => ()
		}
	}
}

impl MainMenu {
	pub fn new() -> Box<Self> {
		Box::from(Self {
			continuous_intents: [0; CONTINUOUS_INTENTS],
			selection_go: GameObject::new(game_object::OBJ_BLANK, -100),
			cursor_go: GameObject::new(game_object::OBJ_CURSOR, -15000),
			sub_menu: SubMenu::Main,
			selectable_objects: vec![],
			selection: usize::MAX,
			camera_anim_ticks: 0,
			name_entry: GameObject::new(game_object::OBJ_TEXTBOX, 0),
			next_menu: None,
			my_levels: None,
			my_levels_scroll: 0.0,
			selected_level: usize::MAX,
			move_selected_level_into_view: false,
			extra_actions_menu: vec![],
			my_levels_grey_out: [
				GameObject::new(game_object::OBJ_BLANK, -5),
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, -5),
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, -5),
			],
			confirm_action: ConfirmAction::None,
			confirm_menu: vec![],
			browse_tab_backings: [
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1),
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1),
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1),
			],
			current_browse_tab: BrowseTab::New,
			search: None,
			search_channel: None,
			search_results: vec![],
			search_message: GameObject::null(),
			search_scroll: 0.0,
			tag_objects: vec![],
			received_results: vec![],
			vote_handle: None,
			browse_grey_out: [
				GameObject::new(game_object::OBJ_BLANK, -5),
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, -5),
				GameObject::new(game_object::OBJ_FILLED_RECTANGLE, -5),
			],
			drag: None,
		})
	}

	fn load_sub_menu(&mut self, command_sender: &mut CommandSender) {
		self.selectable_objects = match self.sub_menu {
			SubMenu::Main => vec![
				Selectable::new(Shape::rect_from_pos_size(90.0, 390.0, 135.0, 75.0))
					.attach_right(1).attach_down(2),
				Selectable::new(Shape::rect_from_pos_size(255.0, 390.0, 135.0, 75.0))
					.attach_left(0).attach_down(3).attach_right(4),
				Selectable::new(Shape::rect_from_pos_size(90.0, 480.0, 135.0, 75.0))
					.attach_right(3).attach_up(0),
				Selectable::new(Shape::rect_from_pos_size(255.0, 480.0, 135.0, 75.0))
					.attach_left(2).attach_up(1).attach_right(5),
				
				Selectable::new(Shape::Circle(435.0, 465.0, 15.0))
					.attach_left(1).attach_down(5),
				Selectable::new(Shape::Circle(435.0, 510.0, 15.0))
					.attach_left(3).attach_up(4).attach_down(6),
				Selectable::new(Shape::Circle(435.0, 555.0, 15.0))
					.attach_left(3).attach_up(5),
			],

			SubMenu::Create => vec![
				Selectable::new(Shape::rect_from_pos_size(315.0, 75.0, 135.0, 75.0))
					.attach_down(1).attach_left(3),
				Selectable::new(Shape::rect_from_pos_size(315.0, 165.0, 60.0, 60.0))
					.attach_up(0).attach_right(2).attach_left(4),
				Selectable::new(Shape::rect_from_pos_size(390.0, 165.0, 60.0, 60.0))
					.attach_up(0).attach_left(1),
				Selectable::new(Shape::Circle(45.0, 30.0, 15.0))
					.attach_right(0).attach_down(4),
				Selectable::new(Shape::Circle(45.0, 225.0, 15.0))
					.attach_right(1).attach_up(3),
			],

			SubMenu::NewLevel => vec![
				Selectable::new(Shape::Rect(582.0, 103.0, 948.0, 136.0))
					.attach_up(1).attach_down(2),
				Selectable::new(Shape::Circle(570.0, 30.0, 15.0))
					.attach_down(0),
				Selectable::new(Shape::rect_from_pos_size(735.0, 165.0, 60.0, 60.0))
					.attach_up(0),
			],

			SubMenu::LevelExtra => vec![
				Selectable::new(Shape::rect_from_pos_size(142.0, 87.0, 196.0, 16.0))
					.attach_up(3).attach_down(1),
				Selectable::new(Shape::rect_from_pos_size(142.0, 103.0, 196.0, 16.0))
					.attach_up(0).attach_down(2),
				Selectable::new(Shape::rect_from_pos_size(142.0, 119.0, 196.0, 16.0))
					.attach_up(1).attach_down(3),
				Selectable::new(Shape::rect_from_pos_size(142.0, 135.0, 196.0, 16.0))
					.attach_up(2).attach_down(0),
				Selectable::new(Shape::rect_from_pos_size(128.0, 71.0, 224.0, 128.0))
			],

			SubMenu::Confirm => vec![
				Selectable::new(Shape::rect_from_pos_size(176.0, 150.0, 48.0, 16.0))
					.attach_right(1),
				Selectable::new(Shape::rect_from_pos_size(256.0, 150.0, 48.0, 16.0))
					.attach_left(0),
				Selectable::new(Shape::rect_from_pos_size(157.0, 97.0, 166.0, 75.0))
			],

			SubMenu::Browse => vec![
				Selectable::new(Shape::Circle(570.0, 345.0, 15.0))
					.attach_right(BROWSE_OTHER).attach_down(BROWSE_TABS + BROWSE_OTHER),
				Selectable::new(Shape::rect_from_pos_size(848.0, 495.0, 60.0, 60.0))
					.attach_right(2),
				Selectable::new(Shape::rect_from_pos_size(923.0, 495.0, 60.0, 60.0))
					.attach_left(1),
				Selectable::new(Shape::Circle(570.0, 459.0, 15.0))
					.attach_up(0),

				Selectable::new(Shape::rect_from_pos_size(600.0, 337.0, 70.0, 16.0))
					.attach_down(BROWSE_TABS + BROWSE_OTHER).attach_left(0).attach_right(BROWSE_OTHER + 1),
				Selectable::new(Shape::rect_from_pos_size(680.0, 337.0, 70.0, 16.0))
					.attach_down(BROWSE_TABS + BROWSE_OTHER).attach_left(BROWSE_OTHER).attach_right(BROWSE_OTHER + 2),
				Selectable::new(Shape::rect_from_pos_size(760.0, 337.0, 70.0, 16.0))
					.attach_down(BROWSE_TABS + BROWSE_OTHER).attach_left(BROWSE_OTHER + 1)//.attach_right(BROWSE_OTHER + 3),
			],

			SubMenu::Quit => vec![]
		};

		for mut go in mem::take(&mut self.confirm_menu) {
			go.destroy(command_sender);
		}

		self.selection = usize::MAX;

		self.update_selection_go(command_sender);

		match self.sub_menu {
			SubMenu::Create => {
				self.name_entry.set_string(command_sender, 0, "");
				self.selected_level = usize::MAX;

				for mut go in mem::take(&mut self.extra_actions_menu) {
					go.destroy(command_sender);
				}

				if let Some(levels) = self.my_levels.as_ref() {
					for i in 0..levels.len() {
						self.selectable_objects.push(Selectable::new(Shape::null()).attach_up(i + 4).attach_down(i + 6).attach_left(3));
					}

					self.set_my_levels_pos(command_sender);
				}
			}

			SubMenu::NewLevel => {
				self.selection = 0;
				self.name_entry.set_real(command_sender, 0, 1.0);
			}

			SubMenu::LevelExtra => self.extra_actions_menu = vec![
				{
					let mut bg = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 12);
					bg.create(command_sender);
					bg.set_real(command_sender, 1, 500.0);
					bg.set_real(command_sender, 2, 500.0);
					bg.set_alpha(command_sender, 0.5);
					bg
				},
				{
					let mut menu = GameObject::new(game_object::OBJ_BLANK, 11);
					menu.x = 128.0;
					menu.y = 71.0;
					menu.create(command_sender);
					menu.set_sprite(command_sender, sprite::LEVEL_EXTRA);
					menu
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 10);
					text.x = 160.0;
					text.y = 100.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 2, 2.0);
					text.set_colour(command_sender, 0x483f51);
					text.set_string(command_sender, 0, LOC_PLAY_9_HEAD.for_current_locale_static());
					text
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 10);
					text.x = 160.0;
					text.y = 116.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 2, 2.0);
					text.set_colour(command_sender, 0x483f51);
					text.set_string(command_sender, 0, LOC_EXPORT.for_current_locale_static());
					text
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 10);
					text.x = 160.0;
					text.y = 132.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 2, 2.0);
					text.set_colour(command_sender, 0x483f51);
					text.set_string(command_sender, 0, LOC_UNPUBLISH.for_current_locale_static());
					text
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 10);
					text.x = 160.0;
					text.y = 148.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 2, 2.0);
					text.set_colour(command_sender, 0x483f51);
					text.set_string(command_sender, 0, LOC_DELETE.for_current_locale_static());
					text
				},
			],

			SubMenu::Confirm => self.confirm_menu = vec![
				{
					let mut menu = GameObject::new(game_object::OBJ_BLANK, 1);
					menu.x = 157.0;
					menu.y = 97.0;
					menu.create(command_sender);
					menu.set_sprite(command_sender, sprite::CONFIRM_MENU);
					menu
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
					text.x = 240.0;
					text.y = 120.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 1, 1.0);
					text.set_colour(command_sender, 0x483f51);
					text.set_string(command_sender, 0, LOC_SURE.for_current_locale_static());
					text
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
					text.x = 200.0;
					text.y = 150.0;
					text.create(command_sender);
					text.set_real(command_sender, 1, 1.0);
					text.set_string(command_sender, 0, LOC_SURE_N.for_current_locale_static());
					text
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
					text.x = 280.0;
					text.y = 150.0;
					text.create(command_sender);
					text.set_real(command_sender, 1, 1.0);
					text.set_string(command_sender, 0, LOC_SURE_Y.for_current_locale_static());
					text
				},
			],

			_ => ()
		}
	}

	fn update_selection_go(&mut self, command_sender: &mut CommandSender) {
		let selectable = self.selectable_objects.get(self.selection);

		match selectable {
			Some(selectable) => match selectable.shape {
				Shape::Rect(x, y, x_max, _) => {
					let width = (x_max - x) as u32;
					match width {
						48 |
						60 |
						135 => {
							self.selection_go.x = x - 3.0;
							self.selection_go.y = y;

							if self.selection_go.exists() {
								self.selection_go.update_position(command_sender);
								self.selection_go.set_alpha(command_sender, 1.0);
								self.selection_go.set_scale(command_sender, 1.0, 1.0);
							} else {
								self.selection_go.create(command_sender);
							}

							self.selection_go.set_sprite(command_sender, match width {
								48 => sprite::R_ARROW,
								60 => sprite::BUTTON_SELECT_SMALL,
								135 => sprite::BUTTON_SELECT_BIG,
								_ => unreachable!()
							});
						}

						196 => {
							self.selection_go.x = x;
							self.selection_go.y = y;
							
							if self.selection_go.exists() {
								self.selection_go.update_position(command_sender);
							} else {
								self.selection_go.create(command_sender);
							}

							self.selection_go.set_sprite(command_sender, sprite::TOOL_SMALL);
							self.selection_go.set_alpha(command_sender, 0.5);
							self.selection_go.set_scale(command_sender, 12.25, 1.0);
						}

						70 => {
							self.selection_go.x = x + 3.0;
							self.selection_go.y = y + 3.0;

							if self.selection_go.exists() {
								self.selection_go.update_position(command_sender);
								self.selection_go.set_alpha(command_sender, 1.0);
								self.selection_go.set_scale(command_sender, 1.0, 1.0);
							} else {
								self.selection_go.create(command_sender);
							}

							self.selection_go.set_sprite(command_sender, sprite::R_ARROW);
						}

						_ => self.selection_go.destroy(command_sender)
					}
				}

				Shape::Circle(x, y, _) => {
					self.selection_go.x = x - 17.0;
					self.selection_go.y = y - 17.0;

					if self.selection_go.exists() {
						self.selection_go.update_position(command_sender);
					} else {
						self.selection_go.create(command_sender);
					}

					self.selection_go.set_sprite(command_sender, sprite::BUTTON_SELECT_CIRCLE);
				}
			}

			None => self.selection_go.destroy(command_sender)
		}
	}

	fn set_selection_by_mouse(&mut self, command_sender: &mut CommandSender, global_state: &ControllerGlobalState) {
		let previous_selection = self.selection;
		self.selection = 0;

		for selectable in &self.selectable_objects {
			if selectable.shape.contains(global_state.mouse_x, global_state.mouse_y) {
				break;
			}

			self.selection += 1;
		}

		if self.selection == self.selectable_objects.len() {
			self.selection = usize::MAX;
		}

		if previous_selection != self.selection {
			self.update_selection_go(command_sender);
		}
	}

	fn clicked(&mut self, command_sender: &mut CommandSender) {

		match (self.sub_menu, self.selection) {
			(SubMenu::Main, 0) => {
				sound::play(command_sender, sound::SE_DECIDE);
				self.transition_sub_menu(SubMenu::Create, command_sender);
			}
			(SubMenu::Main, 1) => if logged_in() {
				sound::play(command_sender, sound::SE_DECIDE);
				self.get_query();
				self.load_browse_results(command_sender);
				self.transition_sub_menu(SubMenu::Browse, command_sender);
			} else {
				sound::play(command_sender, sound::SE_NOT);
			}
			(SubMenu::Main, 2) => {
				sound::play(command_sender, sound::SE_CANCEL);
				GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
				GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);
				self.camera_anim_ticks = 120;
				self.next_menu = Some(Box::from(NotInEditor{}));
				self.sub_menu = SubMenu::Quit;
			}
			(SubMenu::Main, 3) => sound::play(command_sender, sound::SE_NOT),
			(SubMenu::Main, 4) => sound::play(command_sender, sound::SE_NOT),
			(SubMenu::Main, 5) => {
				sound::play(command_sender, sound::SE_DECIDE);
				let _ = webbrowser::open("https://banki-builder.shinten.moe/");
			}
			(SubMenu::Main, 6) => {
				sound::play(command_sender, sound::SE_DECIDE);
				let _ = webbrowser::open("https://discord.com/invite/dJpzdXkeHM");
			}

			(SubMenu::Create, 0) => {
				sound::play(command_sender, sound::SE_DECIDE);
				self.transition_sub_menu(SubMenu::NewLevel, command_sender);
			}
			(SubMenu::Create, 1) |
			(SubMenu::Create, 2) => {
				if let Some(level) = self.my_levels.as_ref().and_then(|v| v.get(self.selected_level)) {
					let mut level_path = fs::get_level_folder();
					level_path.push(&level.level_data.filename);
					sound::play(command_sender, sound::SE_STAGEDECIDE);
					GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
					GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);
					tokio::spawn(Level::load_into(level_path, self.selection == 1, false));
					self.camera_anim_ticks = u32::MAX;
					for _ in 0..120 {
						command_sender.send(Command::Yield);
					}
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
			}
			(SubMenu::Create, 3) => {
				sound::play(command_sender, sound::SE_CANCEL);
				self.transition_sub_menu(SubMenu::Main, command_sender);
			}
			(SubMenu::Create, 4) => {
				if self.selected_level < self.my_levels.as_ref().map(|v| v.len()).unwrap_or(0) {
					sound::play(command_sender, sound::SE_DECIDE);
					self.sub_menu = SubMenu::LevelExtra;
					self.camera_anim_ticks = 1;
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
			}
			(SubMenu::Create, sel) => {
				if sel != usize::MAX {
					sound::play(command_sender, sound::SE_MESSAGE);
				}
				self.selected_level = sel - 5;
				self.set_my_levels_pos(command_sender);
				self.move_selected_level_into_view = true;
			}

			(SubMenu::NewLevel, 0) => {
				sound::play(command_sender, sound::SE_MESSAGE);
				self.name_entry.set_real(command_sender, 0, 1.0);
				if self.cursor_go.sprite != -1 {
					self.name_entry.set_real(command_sender, 3, 2.0);
				}
			}
			(SubMenu::NewLevel, sel) => {
				self.name_entry.set_real(command_sender, 0, 0.0);
				if sel < 3 {
					match sel {
						1 => {
							sound::play(command_sender, sound::SE_CANCEL);
							self.transition_sub_menu(SubMenu::Create, command_sender);
						}
						2 => self.name_entry.query_string(command_sender, 0),
						_ => unreachable!()
					}
				}
			}

			(SubMenu::LevelExtra, 0) => {
				if let Some(level) = self.my_levels.as_ref().and_then(|v| v.get(self.selected_level)) {
					let mut level_path = fs::get_level_folder();
					level_path.push(&level.level_data.filename);
					sound::play(command_sender, sound::SE_STAGEDECIDE);
					GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
					GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);
					tokio::spawn(Level::load_into(level_path, false, true));
					self.camera_anim_ticks = u32::MAX;
					for _ in 0..120 {
						command_sender.send(Command::Yield);
					}
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
			}
			(SubMenu::LevelExtra, 1) => {
				sound::play(command_sender, sound::SE_DECIDE);
				self.sub_menu = SubMenu::Create;
				self.camera_anim_ticks = 1;
				if let Some(level) = self.my_levels.as_ref().and_then(|v| v.get(self.selected_level)) {
					let mut level_path = fs::get_level_folder();
					level_path.push(&level.level_data.filename);
					let name = level.level_data.name.replace(&['<','>',':','"','/','\\','|','?','*'], " ").trim().to_owned();
					tokio::spawn(async move {
						if let Some(fh) = AsyncFileDialog::new().add_filter("BankiBuilder Level", &["lvl"]).set_file_name(name).save_file().await {
							let _ = std::fs::copy(level_path, fh.path());
						}
					});
				}
			}
			(SubMenu::LevelExtra, 2) => if !logged_in() || self.my_levels.as_ref().and_then(|v| v.get(self.selected_level)).map(|l| l.level_data.online_id).unwrap_or(u32::MAX) == u32::MAX {
				sound::play(command_sender, sound::SE_NOT);
			} else {
				sound::play(command_sender, sound::SE_DECIDE);
				self.confirm(ConfirmAction::UnpublishLevel);
			}
			(SubMenu::LevelExtra, 3) => {
				if let Some(level) = self.my_levels.as_ref().and_then(|v| v.get(self.selected_level)) {
					sound::play(command_sender, sound::SE_DECIDE);
					let mut level_path = fs::get_level_folder();
					level_path.push(&level.level_data.filename);
					self.confirm(ConfirmAction::DeleteLevel(level_path));
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
			}
			(SubMenu::LevelExtra, usize::MAX) => {
				self.sub_menu = SubMenu::Create;
				self.camera_anim_ticks = 1;
				sound::play(command_sender, sound::SE_CANCEL);
			}

			(SubMenu::Confirm, 1) => {
				self.sub_menu = SubMenu::Create;
				self.camera_anim_ticks = 1;
				sound::play(command_sender, sound::SE_DECIDE);

				match mem::take(&mut self.confirm_action) {
					ConfirmAction::DeleteLevel(path) => {
						let _ = std::fs::remove_file(path);
						database::load();
						for lo in mem::take(&mut self.my_levels).into_iter().flatten() {
							lo.destroy(command_sender);
						}
					}
					ConfirmAction::UnpublishLevel => {
						let mut level_path = fs::get_level_folder();
						level_path.push(&self.my_levels.as_ref().unwrap()[self.selected_level].level_data.filename);
						let level_id = self.my_levels.as_ref().unwrap()[self.selected_level].level_data.online_id;
						tokio::spawn(async move {
							if query(&UnpublishLevelRQ { level: level_id }).await.is_ok() {
								let mut level = Level::load_from_file(level_path.clone()).await?;
								level.online_id = u32::MAX;
								let out_path = level_path.with_extension("tmp");
								level.serialize(&mut std::fs::File::create(&out_path)?)?;
								let _ = rename(out_path, level_path);
							}

							anyhow::Result::<(), anyhow::Error>::Ok(())
						});
					}
					ConfirmAction::None => ()
				}
			}
			(SubMenu::Confirm, 2) => (),
			(SubMenu::Confirm, sel) => {
				self.sub_menu = SubMenu::Create;
				self.camera_anim_ticks = 1;
				sound::play(command_sender, if sel == usize::MAX { sound::SE_CANCEL } else { sound::SE_DECIDE });
			}

			(SubMenu::Browse, 0) => {
				sound::play(command_sender, sound::SE_CANCEL);
				self.transition_sub_menu(SubMenu::Main, command_sender);
			}
			(SubMenu::Browse, 3) => {
				if self.vote_handle.is_none() && self.search_results.get(self.selected_level).is_some_and(|l| l.level_data.can_vote()) {
					sound::play(command_sender, sound::SE_DECIDE);
					let mut throbber = GameObject::new(game_object::OBJ_THROBBER, 0);
					throbber.x = 570.0;
					throbber.y = 459.0;
					throbber.create(command_sender);
					throbber.set_scale(command_sender, 0.5, 0.5);
					self.tag_objects.push(throbber);
					let level_id = self.search_results[self.selected_level].level_data.online_id;
					self.vote_handle = Some(tokio::spawn(vote(level_id, match get_vote(level_id) {
						Vote::Like => Vote::None,
						Vote::None => Vote::Like
					})));
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
			}
			(SubMenu::Browse, sel) => if sel < 3 {
				match self.search_results.get(self.selected_level) {
					Some(level) => {
						sound::play(command_sender, sound::SE_STAGEDECIDE);
						GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
						GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);
						tokio::spawn(Level::download_into(level.level_data.online_id, sel == 2));
						self.camera_anim_ticks = u32::MAX;
						for _ in 0..120 {
							command_sender.send(Command::Yield);
						}
					}
					None => sound::play(command_sender, sound::SE_NOT)
				}
			} else if sel < BROWSE_TABS + BROWSE_OTHER {
				self.set_browse_tab(command_sender, unsafe { mem::transmute(sel as u8 - BROWSE_OTHER as u8) })
			} else if sel - (BROWSE_TABS + BROWSE_OTHER) < self.search_results.len() {
				if sel != usize::MAX {
					sound::play(command_sender, sound::SE_MESSAGE);
				}
				for mut go in mem::take(&mut self.tag_objects) {
					go.destroy(command_sender);
				}
				self.selected_level = sel - (BROWSE_TABS + BROWSE_OTHER);
				self.set_browse_levels_pos(command_sender);
				self.move_selected_level_into_view = true;

				self.show_browse_level_metadata(command_sender);
			} else {
				match (self.current_browse_tab, sel - (BROWSE_TABS + BROWSE_OTHER)) {
					(BrowseTab::ById, 0) => {
						sound::play(command_sender, sound::SE_MESSAGE);
						self.tag_objects[0].set_real(command_sender, 0, 1.0);
					}
					(BrowseTab::ById, 1) => self.tag_objects[0].query_string(command_sender, 0),
					_ => ()
				}
			}

			_ => ()
		}
	}

	fn right_clicked(&mut self, command_sender: &mut CommandSender) {
		if self.sub_menu == SubMenu::Create && self.selection > 4 && self.selection < self.selectable_objects.len() {
			sound::disable();
			self.clicked(command_sender);
			sound::enable();
			self.selection = 4;
			self.clicked(command_sender);
		}

		else if self.sub_menu == SubMenu::LevelExtra || self.sub_menu == SubMenu::Confirm {
			self.selection = usize::MAX;
			self.clicked(command_sender);
		}
	}

	fn confirm(&mut self, action: ConfirmAction) {
		self.confirm_action = action;
		self.sub_menu = SubMenu::Confirm;
		self.camera_anim_ticks = 1;
	}

	fn menu_pos(menu: SubMenu) -> (f32, f32) {
		match menu {
			SubMenu::Main => (0.0, 315.0),
			SubMenu::NewLevel => (525.0, 0.0),
			SubMenu::Browse => (525.0, 315.0),
			_ => (0.0, 0.0)
		}
	}

	fn show_browse_level_metadata(&mut self, command_sender: &mut CommandSender) {
		self.vote_handle = None;
		if let Some(level) = self.search_results.get(self.selected_level) {
			let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
			text.x = 848.0;
			text.y = 368.0;
			text.create(command_sender);
			text.set_string(command_sender, 0, &format!("ID:     {}\nTAGS:", id_to_code(level.level_data.online_id)));
			self.tag_objects.push(text);

			if get_vote(level.level_data.online_id) == Vote::Like {
				let mut cross = GameObject::new(game_object::OBJ_BLANK, 1);
				cross.x = 561.0;
				cross.y = 450.0;
				cross.create(command_sender);
				cross.set_scale(command_sender, 2.0, 2.0);
				cross.set_sprite(command_sender, sprite::SEIJA_NO);
				self.tag_objects.push(cross);
			}

			let mut y = 390.0;
			for tag in Tag::DISPLAY {
				if level.level_data.tagged(tag) && y < 475.0 {
					let mut backing = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1);
					backing.x = 848.0;
					backing.y = y;
					backing.create(command_sender);
					backing.set_real(command_sender, 0, if tag.mandatory() { 255.0 } else { 0xffffff as f32 });
					backing.set_real(command_sender, 1, 135.0);
					backing.set_real(command_sender, 2, 16.0);
					self.tag_objects.push(backing);

					let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
					text.x = 915.0;
					text.y = y + 2.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 1, 1.0);
					if !tag.mandatory() {
						text.set_colour(command_sender, 0);
					}
					text.set_string(command_sender, 0, tag_name(tag));
					self.tag_objects.push(text);

					y += 20.0;
				}
			}
		}
	}

	fn transition_sub_menu(&mut self, sub_menu: SubMenu, command_sender: &mut CommandSender) {
		let (start_x, start_y) = Self::menu_pos(self.sub_menu);
		let (end_x, end_y) = Self::menu_pos(sub_menu);

		self.sub_menu = sub_menu;
		CameraAnim {
			start_x, start_y,
			end_x, end_y,
			duration: 60
		}.run(command_sender);
		self.camera_anim_ticks = 60;
	}

	fn set_my_levels_pos(&mut self, command_sender: &mut CommandSender) {
		if self.camera_anim_ticks > 0 {
			return;
		}

		let mut y = 60.0 - self.my_levels_scroll.floor();

		for (i, level) in self.my_levels.as_mut().unwrap().into_iter().enumerate() {
			level.set_selected(command_sender, i == self.selected_level);
			level.goto(command_sender, 67.0, y);
			self.selectable_objects[i + 5].shape = Shape::Rect(67.0, y.max(60.0), 292.0, y.min(195.0) + 45.0);

			self.selectable_objects[i + 5].left = 4;
			self.selectable_objects[i + 5].right = 1;
			if y < 202.5 {
				self.selectable_objects[1].left = i + 5;
				self.selectable_objects[4].left = i + 5;
				if y < 135.0 {
					self.selectable_objects[i + 5].right = 0;
					if y < 60.5 {
						self.selectable_objects[0].left = i + 5;
						self.selectable_objects[3].right = i + 5;
						self.selectable_objects[i + 5].left = 3;
					}
				}
			}

			y += 55.0;
		}

		if self.my_levels.as_ref().unwrap().len() > self.selected_level {
			self.my_levels_grey_out[0].set_sprite(command_sender, -1);
			self.my_levels_grey_out[1].set_alpha(command_sender, 0.0);
			self.my_levels_grey_out[2].set_alpha(command_sender, 0.0);
		} else {
			self.my_levels_grey_out[0].set_sprite(command_sender, sprite::GREY_CIRCLE);
			self.my_levels_grey_out[1].set_alpha(command_sender, 0.75);
			self.my_levels_grey_out[2].set_alpha(command_sender, 0.75);
		}
	}

	fn set_browse_levels_pos(&mut self, command_sender: &mut CommandSender) {
		let mut y = 368.0 - self.search_scroll.floor();

		for (i, level) in self.search_results.iter_mut().enumerate() {
			level.set_selected(command_sender, i == self.selected_level);
			level.goto(command_sender, 600.0, y);
			self.selectable_objects[i + BROWSE_TABS + BROWSE_OTHER].shape = Shape::Rect(600.0, y.max(368.0), 825.0, y.min(507.0) + 45.0);
			y += 55.0;
		}

		self.selectable_objects[3].right = (BROWSE_OTHER + BROWSE_TABS).saturating_add(self.selected_level);
		self.selectable_objects[1].left = (BROWSE_OTHER + BROWSE_TABS).saturating_add(self.selected_level);

		if let Some(level) = self.search_results.get(self.selected_level) {
			self.browse_grey_out[1].set_alpha(command_sender, 0.0);
			self.browse_grey_out[2].set_alpha(command_sender, 0.0);
			if level.level_data.can_vote() {
				self.browse_grey_out[0].set_sprite(command_sender, -1);
			} else {
				self.browse_grey_out[0].set_sprite(command_sender, sprite::GREY_CIRCLE);
			}
		} else {
			self.browse_grey_out[0].set_sprite(command_sender, sprite::GREY_CIRCLE);
			self.browse_grey_out[1].set_alpha(command_sender, 0.75);
			self.browse_grey_out[2].set_alpha(command_sender, 0.75);
		}
	}

	fn set_browse_tab(&mut self, command_sender: &mut CommandSender, tab: BrowseTab) {
		if tab != self.current_browse_tab || (self.search.is_some() && (tab == BrowseTab::ById)) {
			sound::play(command_sender, sound::SE_DECIDE);
			self.browse_tab_backings[self.current_browse_tab as usize].set_real(command_sender, 0, 0xffffff as f32);
			self.browse_tab_backings[tab as usize].set_real(command_sender, 0, 0xffff as f32);
			self.current_browse_tab = tab;
			self.get_query();
			self.load_browse_results(command_sender);
		}
	}

	fn get_query(&mut self) {
		self.search = match self.current_browse_tab {
			BrowseTab::New => Some(SearchLevelsRQ {
				id: u32::MAX,
				order: LevelOrdering::New,
				tags: 0,
				neg_tags: 0,
				characters: !0,
				themes: !0,
			}),

			BrowseTab::Top => Some(SearchLevelsRQ {
				id: u32::MAX,
				order: LevelOrdering::Top,
				tags: 0,
				neg_tags: 0,
				characters: !0,
				themes: !0,
			}),

			_ => None
		}
	}

	fn load_browse_results(&mut self, command_sender: &mut CommandSender) {
		self.search_message.destroy(command_sender);
		for lgo in mem::take(&mut self.search_results) {
			lgo.destroy(command_sender);
		}
		self.selected_level = usize::MAX;
		for mut go in mem::take(&mut self.tag_objects) {
			go.destroy(command_sender);
		}
		self.selectable_objects.truncate(BROWSE_OTHER + BROWSE_TABS);

		if self.search.is_none() {
			self.load_browse_sub_page(command_sender);
			return;
		}

		self.search_message = GameObject::new(game_object::OBJ_THROBBER, 0);
		self.search_message.x = 712.0;
		self.search_message.y = 388.0;
		self.search_message.create(command_sender);

		let (sender, receiver) = sync_channel(1);
		self.search_channel = Some(receiver);

		let search = self.search.as_ref().unwrap().clone();
		tokio::spawn(async move {
			if let Ok(rs) = query(&search).await {
				let _ = sender.send(
					rs.results.into_iter()
					.filter_map(|rs|
						bincode::decode_from_slice::<LevelMetadata, _>(&rs.metadata_blob, bincode::config::standard())
						.ok()
						.map(|p| {
							net::level::set_time(p.0.online_id, rs.wr_holder, rs.wr_time);
							net::level::set_likes(p.0.online_id, rs.likes);
							p.0
						})
					).collect()
				);
			}
		});
	}

	fn load_browse_sub_page(&mut self, command_sender: &mut CommandSender) {
		self.tag_objects = match self.current_browse_tab {
			BrowseTab::ById => vec![
				{
					self.selectable_objects.push(
						Selectable::new(Shape::rect_from_pos_size(698.0, 388.0, 134.0, 28.0))
						.attach_up(BROWSE_OTHER + self.current_browse_tab as usize)
						.attach_down(BROWSE_OTHER + BROWSE_TABS + 1)
					);

					self.selectable_objects.push(
						Selectable::new(Shape::rect_from_pos_size(730.0, 424.0, 70.0, 16.0))
						.attach_up(BROWSE_OTHER + BROWSE_TABS)
					);

					let mut input = GameObject::new(game_object::OBJ_TEXTBOX, 0);
					input.x = 702.0;
					input.y = 402.0;
					input.create(command_sender);
					input.set_real(command_sender, 0, 1.0);
					input.set_real(command_sender, 1, 7.0);
					input.set_real(command_sender, 4, 2.0);
					input
				},
				{
					let mut rect = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 2);
					rect.x = 698.0;
					rect.y = 388.0;
					rect.create(command_sender);
					rect.set_real(command_sender, 1, 133.0);
					rect.set_real(command_sender, 2, 27.0);
					rect
				},
				{
					let mut rect = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1);
					rect.x = 700.0;
					rect.y = 390.0;
					rect.create(command_sender);
					rect.set_real(command_sender, 0, 0xffffff as f32);
					rect.set_real(command_sender, 1, 129.0);
					rect.set_real(command_sender, 2, 23.0);
					rect
				},
				{
					let mut rect = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 1);
					rect.x = 730.0;
					rect.y = 424.0;
					rect.create(command_sender);
					rect.set_real(command_sender, 0, 0xffffff as f32);
					rect.set_real(command_sender, 1, 69.0);
					rect.set_real(command_sender, 2, 15.0);
					rect
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
					text.x = 765.0;
					text.y = 368.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 1, 1.0);
					text.set_string(command_sender, 0, LOC_ENTER_ID_HEADER.for_current_locale_static());
					text
				},
				{
					let mut text = GameObject::new(game_object::OBJ_TEXT, 0);
					text.x = 765.0;
					text.y = 425.0;
					text.create(command_sender);
					text.set_real(command_sender, 0, 1.0);
					text.set_real(command_sender, 1, 1.0);
					text.set_colour(command_sender, 0);
					text.set_string(command_sender, 0, LOC_ENTER_ID_SUBMIT.for_current_locale_static());
					text
				}
			],
			_ => vec![]
		}
	}

	fn scroll(&self) -> Option<(f32, f32, Shape)> {
		match self.sub_menu {
			SubMenu::Create => Some((
				self.my_levels_scroll,
				(self.my_levels.as_ref().map(|l| l.len()).unwrap_or_default() as isize * 55 - 190).max(0) as f32,
				Shape::rect_from_pos_size(67.0, 60.0, 225.0, 180.0)
			)),
			SubMenu::Browse => Some((
				self.search_scroll,
				(self.search_results.len() as isize * 55 - 194).max(0) as f32,
				Shape::rect_from_pos_size(600.0, 368.0, 225.0, 184.0)
			)),
			_ => None
		}
	}

	fn set_scroll(&mut self, command_sender: &mut CommandSender, scroll: f32) {
		match self.sub_menu {
			SubMenu::Create => {
				self.my_levels_scroll = scroll;
				self.set_my_levels_pos(command_sender);
			}
			SubMenu::Browse => {
				self.search_scroll = scroll;
				self.set_browse_levels_pos(command_sender);
			}
			_ => ()
		}
	}
}

fn time_text(ticks: u32) -> String {
	if ticks >= 360000 {
		"99:59.99".to_owned()
	} else {
		format!("{}:{:02}.{:02}", ticks / 3600, ticks % 3600 / 60, ((ticks % 60 * 100) as f32 / 60.0).round() as u32)
	}
}