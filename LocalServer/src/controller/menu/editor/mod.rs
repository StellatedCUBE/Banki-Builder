use std::{collections::{HashMap, HashSet}, fs::{rename, File}, io::Write, mem, sync::{atomic::Ordering, Arc, Mutex}, time::Instant, usize};

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use context_menu::{ContextMenu, ContextMenuItem, ContextMenuTarget};
use copypasta::{ClipboardContext, ClipboardProvider};
use flate2::{Compress, Compression, Decompress, FlushCompress, FlushDecompress};
use t_simple_object::ToolSimple;
use tool::{ArcTool, ROW_COUNT};
use tool_icon::ToolIcon;

use crate::controller::{self, command_handler::{Command, CommandOutput, CommandSender}, control_settings::{self, ControlMap, MenuIntent}, event::Event, game_object::{self, GameObject}, global_state::ControllerGlobalState, internal_command::InternalCommand, level::{flipper, simple_object::{ObjectType, SeijaItem, SEIJA_COLOUR_OFF}, tile_manager::{Tile, TileManager}, Character, Level, LevelObject, LevelTheme, ObjectButton, ObjectID, ObjectKey, SubObjectDeleteUndoAction, AABB}, loc::{data::*, Locale}, sound, sprite, undo::{UndoAction, UndoFrame}};

use self::{tool::{Tool, TOOLS}, ui_top_bar::UITopBar, controls::EditorIntent};

use super::{main_menu::MainMenu, play::{Play, PlayingFrom}, pre_publish::PrePublish, shape::Shape, Menu};

mod tool;
mod t_cannon;
mod t_chandelier;
mod t_cursor;
mod t_delete;
mod t_extra_head;
mod t_flipper;
mod t_mochi;
mod t_move;
mod t_onmyoudama_crawl;
mod t_onmyoudama_shoot;
mod t_paired_object;
mod t_pan;
mod t_select;
mod t_spike;
mod t_tile;
mod t_tile_split;
mod t_simple_object;
mod t_undo;
mod t_symbol;
mod ui_top_bar;
mod tool_icon;
mod controls;
pub mod context_menu;

const HOLD_FOR_CONTEXT: u32 = 50;
const ZOOM_FACTOR: u32 = 6;
const PAN_SPEED: f32 = 12.0;

const CONTINUOUS_INTENTS: usize = 8;
const CONTINUOUS_INTENTS_ARR: [EditorIntent; CONTINUOUS_INTENTS] = [
	EditorIntent::CursorLeft,
	EditorIntent::CursorRight,
	EditorIntent::CursorUp,
	EditorIntent::CursorDown,
	EditorIntent::PanLeft,
	EditorIntent::PanRight,
	EditorIntent::PanUp,
	EditorIntent::PanDown,
];

static SAVED_HISTORY: Mutex<(Vec<UndoFrame>, Vec<UndoFrame>)> = Mutex::new((vec![], vec![]));

#[derive(Clone, Copy, PartialEq, Debug)]
enum MouseState {
	World(f32, f32),
	UI(f32, f32),
	ToolDropdown(usize),
	None
}

impl MouseState {
	pub fn is_world(&self) -> bool {
		match &self {
			Self::World(_, _) => true,
			_ => false
		}
	}

	pub fn ddtool(&self) -> Option<ArcTool> {
		match self {
			Self::ToolDropdown(i) => Some(TOOLS.rows[i / 10][i % 10].clone()),
			_ => None
		}
	}
}

#[derive(Clone)]
enum Button {
	Tool(ArcTool),
	Play,
	SettingsMenu,
	Dropdown,
	ObjectButton(usize),
	Zoom,
}

impl PartialEq for Button {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tool(l0), Self::Tool(r0)) => Arc::ptr_eq(&l0, &r0),
			(Self::Play, Self::Play) => true,
			(Self::SettingsMenu, Self::SettingsMenu) => true,
			(Self::Dropdown, Self::Dropdown) => true,
			(Self::ObjectButton(x), Self::ObjectButton(y)) => *x == *y,
			(Self::Zoom, Self::Zoom) => true,
			_ => false
        }
    }
}

impl Button {
	fn tool(self, editor: &Editor) -> Option<ArcTool> {
		match self {
			Self::Tool(tool) => Some(tool),
			Self::ObjectButton(i) => if editor.object_buttons[i].game_object.colour == SEIJA_COLOUR_OFF {
				match editor.object_buttons[i].game_object.sprite {
					sprite::SEIJA_BUTTON_LEFT => Some(Arc::new(ToolSimple::new(ObjectType::SeijaItem(SeijaItem::Hirarinuno)))),
					sprite::SEIJA_BUTTON_RIGHT => Some(Arc::new(ToolSimple::new(ObjectType::SeijaItem(SeijaItem::Camera)))),
					sprite::SEIJA_BUTTON_UP => Some(Arc::new(ToolSimple::new(ObjectType::SeijaItem(SeijaItem::Bomb)))),
					sprite::SEIJA_BUTTON_DOWN => Some(Arc::new(ToolSimple::new(ObjectType::SeijaItem(SeijaItem::Hammer)))),
					_ => None
				}
			} else { None }
			_ => None
		}
	}
}

enum PasteGhostData {
	Tile(i16, i16, Tile),
	Object(Box<dyn LevelObject + Send>),
	ChainInserter(ObjectKey, usize),
}

struct PasteGhost {
	ghosts: [GameObject; 2],
	data: PasteGhostData
}

const SETTINGS_MENU_MAX_ROW: i32 = 4;

struct SettingsMenu {
	objects: Vec<GameObject>,
	selected_row: i32,
	selected_col: u32,
	name_focused: bool,
	sent_name_unfocus: bool,
	has_ticked: bool,
	theme: LevelTheme,
	initial_name: String,
	key_down: bool,
}

pub struct Editor {
	pub level: Arc<Mutex<Level>>,
	pub objects: Vec<Vec<GameObject>>,
	continuous_intents: [u32; CONTINUOUS_INTENTS],
	bg_object: GameObject,
	cursor_go: GameObject,
	editor_go: GameObject,
	selected_tool: Arc<dyn Tool + Send + Sync>,
	mx: f32,
	my: f32,
	vx: f32,
	vy: f32,
	ix: i32,
	iy: i32,
	grab_start: MouseState,
	grab_duration: u32,
	ui_top_bar: UITopBar,
	tool_row_select: (usize, usize, Vec<ToolIcon>, GameObject),
	drag_ghost: GameObject,
	drag_tool: Option<ArcTool>,
	last_save: Instant,
	controls: ControlMap<EditorIntent>,
	active_tool: Option<Arc<dyn Tool + Send + Sync>>,
	undo_frames: Vec<UndoFrame>,
	redo_frames: Vec<UndoFrame>,
	current_undo_frame_open: bool,
	selection_start: (f32, f32),
	selection_box: Option<GameObject>,
	selection: HashMap<ObjectID, GameObject>,
	settings_menu: Option<SettingsMenu>,
	context_menu: ContextMenu,
	music_preview: bool,
	pub object_buttons: Vec<ObjectButton>,
	zoomed_out: bool,
	block_zoom: bool,
	bg_rect: GameObject,
	clipboard: ClipboardContext,
	paste_ghosts: Vec<PasteGhost>,
	reset_camera: bool,
}

impl Menu for Editor {
	fn name(&self) -> &'static str { "Editor" }

    fn on_enter(&mut self, command_sender: &mut CommandSender) {
		self.cursor_go.create(command_sender);
		self.cursor_go.set_sprite(command_sender, sprite::CURSOR);
		self.editor_go.create(command_sender);
		self.drag_ghost.create(command_sender);
		self.drag_ghost.set_real(command_sender, 0, -16.0);
		self.drag_ghost.set_real(command_sender, 1, -16.0);
		self.drag_ghost.set_alpha(command_sender, 0.5);

		self.ui_top_bar.init(command_sender);

		self.build_level(command_sender);

		if self.reset_camera {
			command_sender.send(Command::F32(vec![self.objects[1][0].x - 224.0, self.objects[1][0].y - 200.0]));
			command_sender.send(Command::MoveCamera);
		}
	}

    fn on_leave(&mut self, command_sender: &mut CommandSender) {
        self.save_with_log(command_sender);
    }

    fn on_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {

        self.mx = global_state.mouse_x;
		self.my = global_state.mouse_y;
		self.vx = global_state.view_x;
		self.vy = global_state.view_y;

		if event != Event::GameQuit && event != Event::ClearObjectButtons && event != Event::CreateObjectButton {
			let ctx_menu = self.context_menu.handle_event(command_sender, event, global_state);

			if ctx_menu != 0 {
				if ctx_menu < 0 {
					let action = -ctx_menu-1;
					if action == sprite::DELETE {
						match &self.context_menu.target {
							ContextMenuTarget::Object(obj) => {
								let action = self.delete(command_sender, *obj);

								if action.is_some() {
									sound::play(command_sender, sound::SE_HOLD);
								}

								self.add_undo_frame_complete(action.into_iter().collect());
							}
							ContextMenuTarget::Selection => self.handle_intent(command_sender, EditorIntent::MassDelete),
							_ => ()
						}
					} else {
						match &self.context_menu.target {
							&ContextMenuTarget::Object(ObjectID::Object(id)) => {
								let mut level = self.level.lock().unwrap();
								let theme = level.theme;
								level.objects[id].destroy_editor_view(command_sender, &mut self.objects[id], &level);
								let actions = level.objects[id].handle_context_menu_action(command_sender, id, action, theme);
								self.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
								
								if id == 1 {
									for (i, lo) in level.objects.iter().enumerate() {
										if lo.recreate_on_character_change() {
											lo.destroy_editor_view(command_sender, &mut self.objects[i], &level);
											self.objects[i] = lo.create_editor_view(command_sender, &level);
										}
									}	
								}

								drop(level);
								self.add_undo_frame_complete(actions);
							}
							&ContextMenuTarget::Object(ObjectID::SubObject(id, sub_object)) => {
								let mut level = self.level.lock().unwrap();
								let theme = level.theme;
								let (recreate, actions) = level.objects[id].handle_sub_object_context_menu_action(command_sender, id, sub_object, action, theme);
								if recreate {
									level.objects[id].destroy_editor_view(command_sender, &mut self.objects[id], &level);
									self.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
								}
								drop(level);
								self.add_undo_frame_complete(actions);
							}
							ContextMenuTarget::Selection => {
								let mut actions = vec![];
								let mut level = self.level.lock().unwrap();
								let theme = level.theme;
								for obj in self.selection.keys() {
									match *obj {
										ObjectID::Object(id) => {
											level.objects[id].destroy_editor_view(command_sender, &mut self.objects[id], &level);
											actions.append(&mut level.objects[id].handle_context_menu_action(command_sender, id, action, theme));
											self.objects[id] = level.objects[id].create_editor_view(command_sender, &level);

											if id == 1 {
												for (i, lo) in level.objects.iter().enumerate() {
													if lo.recreate_on_character_change() {
														lo.destroy_editor_view(command_sender, &mut self.objects[i], &level);
														self.objects[i] = lo.create_editor_view(command_sender, &level);
													}
												}	
											}
										}
										ObjectID::SubObject(id, sub_object) => {
											let (recreate, mut this_actions) = level.objects[id].handle_sub_object_context_menu_action(command_sender, id, sub_object, action, theme);
											actions.append(&mut this_actions);
											if recreate {
												level.objects[id].destroy_editor_view(command_sender, &mut self.objects[id], &level);
												self.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
											}
										}
									}
									sound::disable();
								}
								drop(level);
								self.add_undo_frame_complete(actions);
								sound::enable();
							}
							ContextMenuTarget::Tool(tool) => {
								tool.handle_context_menu_action(action, self.level.lock().unwrap().theme);
								self.selected_tool = tool.clone();
							}
						}
					}
				}
				return;
			}

			if self.settings_menu.is_some() {
				return self.on_event_settings_menu(command_sender, event, global_state);
			}
		}

		match event {
			Event::KeyDown |
			Event::MouseDown |
			Event::ButtonDown => if let Some(intent) = self.controls.get_intent(global_state.last_mod_input) {
				if intent.discriminant() < CONTINUOUS_INTENTS {
					self.continuous_intents[intent.discriminant()] = 1;
				} else {
					self.handle_intent(command_sender, intent);
				}
			},
			
			Event::KeyUp |
			Event::MouseUp |
			Event::ButtonUp => if let Some(intent) = self.controls.get_intent(global_state.last_mod_input) {
				if intent.discriminant() < CONTINUOUS_INTENTS {
					self.continuous_intents[intent.discriminant()] = 0;
				}

				match intent {
					EditorIntent::Primary => {
						self.handle_release(command_sender);
						self.grab_start = MouseState::None;
						if self.drag_tool.is_some() {
							self.drag_tool = None;
							self.drag_ghost.set_sprite(command_sender, -1);
						}
		
						self.end_tool(command_sender);
					}

					EditorIntent::UseTool(tool_id) => if let Some(active_tool) = self.active_tool.as_ref() {
						if Arc::ptr_eq(active_tool, &TOOLS.from_id(tool_id).unwrap()) {
							active_tool.clone().use_end(command_sender, self);
							self.active_tool = None;
							self.current_undo_frame_open = false;
						}
					}

					_ => ()
				}
			},

			Event::FocusIn |
			Event::FocusOut => if let Some(active_tool) = self.active_tool.as_ref() {
				active_tool.clone().use_end(command_sender, self);
				self.active_tool = None;
				self.current_undo_frame_open = false;
			}

			Event::MouseMove => {
				self.tool_row_select.0 = usize::MAX;

				if global_state.mouse_dx.abs() > 1.1 || global_state.mouse_dy.abs() > 1.1 {
					self.grab_duration = HOLD_FOR_CONTEXT;
				}

				let ix = (self.mx / 32.0).floor() as i32;
				let iy = (self.my / 32.0).floor() as i32;

				if let Some(tool) = self.active_tool.as_ref() {
					tool.clone().use_frame(command_sender, self, self.mx / 32.0, self.my / 32.0);
				}

				if ix != self.ix || iy != self.iy {
					self.ix = ix;
					self.iy = iy;

					if let Some(tool) = self.active_tool.as_ref() {
						let actions = tool.clone().use_new_tile(command_sender, self, self.mx / 32.0, self.my / 32.0);
						self.add_undo_frame(actions);
					}
				}

				if self.mouse_state().is_world() {
					if self.drag_tool.is_none() {
						if let Some(tool) = self.grab_start.ddtool().filter(|_| {
							let (mx, my) = self.mouse_screen_pos();
							let (mx, my) = (mx - 35.0, my - 41.0);
							!self.tool_row_select.2.iter().any(|i| i.contains(mx, my))
						}).or(self.button(self.grab_start).and_then(|b| b.tool(self))) {
							if self.is_tool_dropdown_open() {
								self.toggle_tool_select_dropdown(command_sender);
							}
							self.drag_tool = Some(tool.clone());
							let level = self.level.lock().unwrap();
							self.drag_ghost.set_sprite(command_sender, tool.sprite(level.theme, level.character()));
							if let MouseState::ToolDropdown(_) = self.grab_start {
								self.grab_start = MouseState::None;
							}
						}
					}
				}
			}

			Event::GameQuit => {
				self.save().ok();
			}

			Event::Tick => {
				for i in 0..CONTINUOUS_INTENTS {
					if self.continuous_intents[i] > 0 {
						self.continuous_intents[i] += 1;
						self.handle_intent(command_sender, CONTINUOUS_INTENTS_ARR[i]);
					}
				}

				if self.grab_start != MouseState::None {
					self.grab_duration += 1;
					if self.grab_duration == HOLD_FOR_CONTEXT && !self.current_undo_frame_open && match &self.active_tool {
						Some(tool) => !tool.block_context_menu(),
						None => true
					}{
						self.handle_release(command_sender);
						self.handle_intent(command_sender, EditorIntent::ContextMenu);
					}
				} else {
					self.grab_duration = 0;
				}

				self.level.lock().unwrap().tile_manager_mut().maybe_update_deco(command_sender);
			}

			Event::ClearObjectButtons => {
				let id: ObjectKey = unsafe { mem::transmute(global_state.recieved_real) };
				for button in &mut self.object_buttons {
					if button.object == id {
						button.game_object.destroy(command_sender);
					}
				}
				self.object_buttons.retain(|p| p.object != id);
			}

			Event::CreateObjectButton => self.object_buttons.push(mem::take(&mut global_state.object_button_massive_hack).unwrap()),

			Event::ViewMove => self.block_zoom = false,

			_ => ()
		}

		let (mx, my) = self.mouse_screen_pos();
		let level = self.level.lock().unwrap();
		let theme = level.theme;
		let character = level.character();
		drop(level);
		self.ui_top_bar.highlight_appropriate(command_sender, theme, character, mx, my, self.selected_tool.clone());

		let (mx, my) = (mx - 35.0, my - 41.0);
		for (i, icon) in self.tool_row_select.2.iter_mut().enumerate() {
			icon.update(command_sender, theme, character, &self.selected_tool, mx, my,
				if self.tool_row_select.0 == usize::MAX { None } else
				{ Some(self.tool_row_select.0 == i / 10 && self.tool_row_select.1 == i % 10) });
		}
    }

	fn create_chain_object_inserter(&mut self, command_sender: &mut CommandSender, object: usize, index: usize, sprite: i32) {
		if self.paste_ghosts.len() == 0 && !self.zoomed_out && !self.is_tool_dropdown_open() && self.settings_menu.is_none() {
			let mut ghost = GameObject::new(game_object::OBJ_CURSOR, -8000);
			ghost.create(command_sender);
			ghost.set_alpha(command_sender, 0.8);
			ghost.set_sprite(command_sender, sprite);
			ghost.set_real(command_sender, 0, -16.0);
			ghost.set_real(command_sender, 1, -16.0);

			self.paste_ghosts.push(PasteGhost {
				ghosts: [ghost, GameObject::null()],
				data: PasteGhostData::ChainInserter(self.level.lock().unwrap().object_keys[object], index),
			});
			self.ui_top_bar.disable(command_sender);
		}
	}
}

impl Editor {
	pub fn new(level: Arc<Mutex<Level>>, reset_camera: bool) -> Self {
		let mut lock = SAVED_HISTORY.lock().unwrap();

		let undo_frames = mem::take(&mut lock.0);
		let redo_frames = mem::take(&mut lock.1);

		Self {
			level,
			objects: vec![],
			continuous_intents: [0; CONTINUOUS_INTENTS],
			bg_object: GameObject::null(),
			cursor_go: GameObject::new(game_object::OBJ_CURSOR, -15000),
			editor_go: GameObject::new(game_object::OBJ_EDITOR_ACTUAL, -9000),
			selected_tool: TOOLS.cursor.clone(),
			mx: 0.0, my: 0.0, vx: 0.0, vy: 0.0, ix: 0, iy:0,
			grab_start: MouseState::None,
			grab_duration: 0,
			ui_top_bar: UITopBar::new(),
			tool_row_select: (usize::MAX, usize::MAX, vec![], GameObject::null()),
			drag_ghost: GameObject::new(game_object::OBJ_CURSOR, -14990),
			drag_tool: None,
			last_save: Instant::now(),
			controls: controls::get(),
			active_tool: None,
			undo_frames,
			redo_frames,
			current_undo_frame_open: false,
			selection_start: (0.0, 0.0),
			selection_box: None,
			selection: HashMap::new(),
			settings_menu: None,
			context_menu: ContextMenu::default(),
			music_preview: false,
			object_buttons: vec![],
			zoomed_out: false,
			block_zoom: true,
			bg_rect: GameObject::new(game_object::OBJ_FILLED_RECTANGLE, 16380),
			clipboard: ClipboardContext::new().unwrap(),
			paste_ghosts: vec![],
			reset_camera,
		}
	}

	fn update_bg(&mut self, command_sender: &mut CommandSender, level: &Level) {
		self.bg_object.destroy(command_sender);
		self.bg_object = level.create_bg(command_sender);

		if !self.bg_rect.exists() {
			self.bg_rect.x = -480.0;
			self.bg_rect.y = -480.0;
			self.bg_rect.create(command_sender);
			self.bg_rect.set_alpha(command_sender, 0.0);
			self.bg_rect.set_real(command_sender, 1, (1000 * ZOOM_FACTOR) as f32);
			self.bg_rect.set_real(command_sender, 2, (1000 * ZOOM_FACTOR) as f32);
			self.bg_rect.set_real(command_sender, 3, 1.0);
		}

		self.bg_rect.set_real(command_sender, 0, level.theme.bg_colour());
	}

	fn build_level(&mut self, command_sender: &mut CommandSender) {
		let arc = self.level.clone();
        let mut level = arc.lock().unwrap();

		if !self.objects.is_empty() {
			panic!("build_level with non-empty objects vector");
		}

		self.update_bg(command_sender, &level);
		
		for object in &level.objects {
			self.objects.push(object.create_editor_view(command_sender, &level));
		}

		level.speedrun_techniques = false;
    }

	fn regenerate(&mut self, command_sender: &mut CommandSender) {
		let level: std::sync::MutexGuard<'_, Level> = self.level.lock().unwrap();
		for i in 0..self.objects.len() {
			level.objects[i].destroy_editor_view(command_sender, &mut self.objects[i], &level);
		}
		self.objects.clear();
		drop(level);
		self.build_level(command_sender);
	}

	fn add_undo_frame(&mut self, mut actions: Vec<UndoAction>) {
		if actions.len() != 0 {
			self.redo_frames = vec![];

			if self.current_undo_frame_open {
				self.undo_frames.last_mut().unwrap().actions.append(&mut actions);
			} else {
				self.undo_frames.push(UndoFrame {
					actions
				});
				self.current_undo_frame_open = true;
			}
		}
	}

	fn add_undo_frame_complete(&mut self, actions: Vec<UndoAction>) {
		self.add_undo_frame(actions);
		self.current_undo_frame_open = false;
	}

	fn mouse_screen_pos(&self) -> (f32, f32) {
		if self.zoomed_out {
			((self.mx - self.vx) / ZOOM_FACTOR as f32, (self.my - self.vy) / ZOOM_FACTOR as f32)
		} else {
			(self.mx - self.vx, self.my - self.vy)
		}
	}

	fn mouse_state(&self) -> MouseState {
		let (sx, sy) = self.mouse_screen_pos();

		if (self.ui_top_bar.enabled() && (sy < 40.0 || (!self.is_tool_dropdown_open() &&
		self.object_buttons.iter().any(|b| b.bounds.contains(self.mx, self.my))))) ||
		(self.zoomed_out && sy < 19.0 && sx > 404.0) {
			MouseState::UI(sx, sy)
		} else {
			MouseState::World(self.mx / 32.0, self.my / 32.0)
		}
	}

	fn button(&self, mouse: MouseState) -> Option<Button> {
		match mouse {
			MouseState::UI(x, y) => {
				if Shape::rect_from_pos_size(443.0, 2.0, 16.0, 16.0).contains(x, y) { Some(Button::Zoom) }
				else if self.ui_top_bar.enabled() {
					if Shape::rect_from_pos_size(2.0, 2.0, 16.0, 16.0).contains(x, y) { Some(Button::Tool(TOOLS.move_.clone())) }
					else if Shape::rect_from_pos_size(22.0, 2.0, 16.0, 16.0).contains(x, y) { Some(Button::Tool(TOOLS.pan.clone())) }
					else if Shape::rect_from_pos_size(2.0, 21.0, 16.0, 16.0).contains(x, y) { Some(Button::Tool(TOOLS.select.clone())) }
					else if Shape::rect_from_pos_size(22.0, 21.0, 16.0, 16.0).contains(x, y) { Some(Button::Tool(TOOLS.delete.clone())) }
					else if Shape::Rect(42.0, 2.0, 438.0, 38.0).contains(x, y) && (x as i32 - 2) % 40 <= 36 { Some(Button::Tool(self.ui_top_bar.tool_bar[x as usize / 40 - 1].tool.clone())) }
					else if Shape::rect_from_pos_size(462.0, 2.0, 16.0, 16.0).contains(x, y) { Some(Button::SettingsMenu) }
					else if Shape::rect_from_pos_size(462.0, 21.0, 16.0, 16.0).contains(x, y) { Some(Button::Play) }
					else if Shape::rect_from_pos_size(443.0, 21.0, 16.0, 16.0).contains(x, y) { Some(Button::Dropdown) }
					else {
						let x = x + self.vx;
						let y = y + self.vy;
						self.object_buttons.iter()
							.enumerate()
							.filter(|b| b.1.bounds.contains(x, y))
							.next()
							.and_then(|b| Some(Button::ObjectButton(b.0)))
					}
				}
				else if Shape::rect_from_pos_size(404.0, 1.0, 18.0, 18.0).contains(x, y) { Some(Button::Tool(TOOLS.select.clone())) }
				else if Shape::rect_from_pos_size(423.0, 1.0, 18.0, 18.0).contains(x, y) { Some(Button::Tool(TOOLS.move_.clone())) }
				else if Shape::rect_from_pos_size(461.0, 1.0, 18.0, 18.0).contains(x, y) { Some(Button::Tool(TOOLS.pan.clone())) }
				else {None}
			}
			_ => None
		}
	}

	fn handle_release(&mut self, command_sender: &mut CommandSender) {
		let end = self.mouse_state();
		if let Some(start_button) = self.button(self.grab_start) {
			if let Some(end_button) = self.button(end) {
				if start_button == end_button {
					match end_button {
						Button::Tool(tool) => self.set_tool(command_sender, tool),
						Button::Play => self.handle_intent(command_sender, EditorIntent::Play),
						Button::SettingsMenu => self.handle_intent(command_sender, EditorIntent::ToggleSettingsMenu),
						Button::Dropdown => self.handle_intent(command_sender, EditorIntent::ToggleToolRowSelectDropdown),
						Button::Zoom => self.handle_intent(command_sender, EditorIntent::ZoomToggle),
						Button::ObjectButton(id) => {
							let actions = (self.object_buttons[id].callback)(command_sender, self, id);
							self.add_undo_frame_complete(actions);
						}
					}
				}
			}
		}

		if let MouseState::World(x, y) = end {
			if let Some(tool) = self.drag_tool.clone() {
				if tool.clear_selection() {
					self.set_selected(command_sender, vec![]);
				}
				let actions = tool.use_start(command_sender, self, x, y);
				self.add_undo_frame_complete(actions);
				tool.use_end(command_sender, self);
			}

			else if let MouseState::ToolDropdown(i) = self.grab_start {
				self.set_tool(command_sender, self.tool_row_select.2[i].tool.clone());
				self.ui_top_bar.set_row(command_sender, i / 10);
				sound::play(command_sender, sound::SE_BREAK);
				self.toggle_tool_select_dropdown(command_sender);
			}
		}
	}

	fn handle_intent(&mut self, command_sender: &mut CommandSender, intent: EditorIntent) {
		match intent {
			EditorIntent::Primary => if self.paste_ghosts.len() == 0 {
				self.grab_start = self.mouse_state();
				
				if self.tool_row_select.1 != usize::MAX {
					let (mx, my) = self.mouse_screen_pos();
					let (mx, my) = (mx - 35.0, my - 41.0);

					for (i, icon) in self.tool_row_select.2.iter().enumerate() {
						if icon.contains(mx, my) {
							self.grab_start = MouseState::ToolDropdown(i);
							break;
						}
					}

					if mx < 0.0 || my < 0.0 || mx > 410.0 || my > 170.0 {
						sound::play(command_sender, sound::SE_BREAK);
						self.toggle_tool_select_dropdown(command_sender);
					}

					if self.grab_start.is_world() || self.button(self.grab_start) == Some(Button::Dropdown) {
						self.grab_start = MouseState::None;
					}
				}

				match self.grab_start {
					MouseState::World(x, y) => {
						if self.active_tool.is_none() {
							let mut tool = self.selected_tool.clone();
							if self.zoomed_out && !tool.can_be_used_zoomed_out() {
								tool = TOOLS.cursor.clone();
							}
							if tool.clear_selection() {
								self.set_selected(command_sender, vec![]);
							}
							let actions = tool.use_start(command_sender, self, x, y);
							self.add_undo_frame(actions);
							self.active_tool = Some(tool);
						}
					}
					_ => ()
				}
			} else if let MouseState::World(x, y) = self.mouse_state() {
				let x = x.floor() as i32;
				let y = y.floor() as i32;
				let mut level = self.level.lock().unwrap();
				let mut undo = vec![];
				let mut delete = vec![];
				flipper::SHIFT_HITBOX.store(true, Ordering::Relaxed);

				for mut pg in mem::take(&mut self.paste_ghosts) {
					pg.ghosts[0].destroy(command_sender);
					pg.ghosts[1].destroy(command_sender);
					match pg.data {
						PasteGhostData::Tile(tx, ty, tile) => {
							let x = x + tx as i32;
							let y = y + ty as i32;
							let tt = tile.types();
							let mut conflict: Vec<_> = level.at(x, y).into_iter().filter(|o| match *o {
								ObjectID::SubObject(0, _) => false,
								object => level.object_types(object) & tt != 0
							}).collect();

							if conflict.iter().any(|o| !level.objects[o.id()].can_delete() /*||
								self.object_buttons.iter().any(|b| b.object == o.id())*/) {
								continue;
							}

							delete.append(&mut conflict);

							undo.push(UndoAction::SetTile(x, y, level.tile_manager().get(x, y)));
							level.tile_manager_mut().set_and_update(command_sender, x, y, tile);
						}

						PasteGhostData::Object(mut object) => {
							if let Some(ot) = object.simple_object_type() {
								if ot.unique() && level.objects.iter().any(|o| o.simple_object_type() == Some(ot)) {
									continue;
								}
							}

							object.move_by(x, y);
							let bb = object.bounding_box().expand_to_tile();
							let mut collides = vec![];
							for x in bb.x as i32..(bb.x + bb.width) as i32 {
								for y in bb.y as i32..(bb.y + bb.height) as i32 {
									collides.append(&mut level.at(x, y));
								}
							}

							let ot = object.types();
							if collides.iter().any(|o|
								((!level.objects[o.id()].can_delete() && o.id() > 0) /*|| self.object_buttons.iter().any(|b| b.object == o.id())*/) &&
								level.object_types(*o) & ot != 0) {
								continue;
							}

							delete.extend(collides.into_iter().filter(|o| match *o {
								ObjectID::SubObject(0, sub_object) => {
									let (x, y) = TileManager::from_sub_id(sub_object);
									let tile = level.tile_manager().get(x, y);
									if tile.types() & ot != 0 {
										undo.push(UndoAction::SetTile(x, y, tile));
										level.tile_manager_mut().set_and_update(command_sender, x, y, Tile::None);
									}
									false
								}
								object => level.object_types(object) & ot != 0,
							}));

							self.objects.push(object.create_editor_view(command_sender, &level));
							level.objects.push(object);
							level.fill_object_keys();
							undo.push(UndoAction::Delete(ObjectID::Object(self.objects.len() - 1)));
						}

						PasteGhostData::ChainInserter(key, index) => {
							let oi = level.index_by_key(key);
							level.objects[oi].destroy_editor_view(command_sender, &mut self.objects[oi], &level);
							level.objects[oi].to_chain().insert(index, x, y);
							self.objects[oi] = level.objects[oi].create_editor_view(command_sender, &level);
							undo.push(UndoAction::Delete(ObjectID::SubObject(oi, index)));
						}
					}
				}

				if !self.zoomed_out {
					self.ui_top_bar.enable(command_sender);
				}

				drop(level);
				flipper::SHIFT_HITBOX.store(false, Ordering::Relaxed);
				self.set_selected(command_sender, delete);
				self.add_undo_frame(undo);
				sound::play(command_sender, sound::SE_HOLD);
				sound::disable();
				self.handle_intent(command_sender, EditorIntent::MassDelete);
				sound::enable();
			}

			EditorIntent::Save => {
				sound::play(command_sender, sound::SE_SYOUKAI);
				self.save_with_log(command_sender);
			}

			EditorIntent::Play => {
				self.end_tool(command_sender);
				sound::play(command_sender, sound::SE_DECIDE);
				let mut goal = GameObject::new(game_object::OBJ_GOAL, 0);
				goal.x = 3200000.0;
				goal.create(command_sender);
				goal.destroy_server_only();
				let mut transition = GameObject::new(game_object::OBJ_QUICKRETRY, -15990);
				transition.create(command_sender);
				for _ in 0..30 {
					command_sender.send(Command::Yield);
				}
				*SAVED_HISTORY.lock().unwrap() = (
					mem::take(&mut self.undo_frames),
					mem::take(&mut self.redo_frames)
				);
				InternalCommand::SwitchToMenu(Box::from(Play::new(self.level.clone(), PlayingFrom::Editor, false))).run();
			}

			EditorIntent::Exit => {
				self.end_tool(command_sender);
				sound::play(command_sender, sound::SE_CANCEL);

				GameObject::new(game_object::OBJ_BLACK_TRANSITION_1, -15555).create(command_sender);
				GameObject::new(game_object::OBJ_BLACK_TRANSITION_2, -15555).create(command_sender);

				for _ in 0..120 {
					command_sender.send(Command::Yield);
				}

				command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
				InternalCommand::SwitchToMenu(MainMenu::new()).run();
			}

			EditorIntent::SetTool(id) => self.set_tool(command_sender, TOOLS.from_id(id).unwrap()),

			EditorIntent::SetToolFromRow(num) => self.set_tool(command_sender, self.ui_top_bar.tool_bar[num].tool.clone()),

			EditorIntent::SetToolRow(row) => self.ui_top_bar.set_row(command_sender, row),

			EditorIntent::PrevToolRow => self.ui_top_bar.set_row(command_sender, (ROW_COUNT - 1 + self.ui_top_bar.tool_bar_index) % ROW_COUNT),

			EditorIntent::NextToolRow => self.ui_top_bar.set_row(command_sender, (1 + self.ui_top_bar.tool_bar_index) % ROW_COUNT),

			EditorIntent::ToggleToolRowSelectDropdown => if self.ui_top_bar.enabled() {
				self.end_tool(command_sender);
				sound::play(command_sender, sound::SE_BREAK);
				self.toggle_tool_select_dropdown(command_sender);
			}

			EditorIntent::UseTool(tool_id) => {
				let tool = TOOLS.from_id(tool_id).unwrap();
				if tool.clear_selection() {
					self.set_selected(command_sender, vec![]);
					self.clear_paste_ghosts(command_sender);
				}
				let actions = tool.use_start(command_sender, self, self.mx / 32.0, self.my / 32.0);
				self.add_undo_frame(actions);
				if self.active_tool.is_none() {
					self.active_tool = Some(tool);
				} else {
					tool.use_end(command_sender, self);
				}
			}

			EditorIntent::SelectAll => {
				self.end_tool(command_sender);
				sound::play(command_sender, sound::SE_MESSAGE);

				let level = self.level.lock().unwrap();
				let objlen = level.objects.len();
				let mut selection = Vec::with_capacity(objlen - 1);

				for i in 1..objlen {
					match level.objects[i].sub_object_count() {
						0 => selection.push(ObjectID::Object(i)),
						soc => for j in 0..soc {
							selection.push(ObjectID::SubObject(i, j));
						}
					}
				}

				let tm = level.tile_manager();
				for (x, y) in tm.tiles.keys() {
					for i in 0..16 {
						for j in 0..16 {
							if tm.get(x + i, y + j) != Tile::None {
								selection.push(ObjectID::SubObject(0, TileManager::to_sub_id(x + i, y + j)));
							}
						}
					}
				}

				drop(level);

				self.set_selected(command_sender, selection);
			}

			EditorIntent::MassDelete => if self.paste_ghosts.len() == 0 {
				self.end_tool(command_sender);

				if self.selection.len() > 0 {
					sound::play(command_sender, sound::SE_HOLD);
				}

				let mut ids: Vec<_> = self.selection.keys().cloned().collect();
				ids.sort_unstable();

				let mut actions = vec![];
				let mut skip = HashSet::new();

				for id in ids {
					if skip.contains(&id.id()) {
						continue;
					}

					match self.delete(command_sender, id) {
						Some(ua) => {
							if let UndoAction::AddObject(_, _) = ua {
								skip.insert(id.id());
							}
							actions.push(ua);
						}
						None => ()
					}
				}

				self.add_undo_frame_complete(actions);
			} else {
				self.clear_paste_ghosts(command_sender);
			}

			EditorIntent::ToggleSettingsMenu => {
				sound::play(command_sender, sound::SE_BREAK);

				if self.paste_ghosts.len() > 0 {
					self.clear_paste_ghosts(command_sender);
				}

				else if self.is_tool_dropdown_open() {
					self.toggle_tool_select_dropdown(command_sender);
				}

				else if self.zoomed_out {
					self.handle_intent(command_sender, EditorIntent::ZoomIn);
				}

				else {
					self.end_tool(command_sender);

					match self.settings_menu {
						Some(_) => self.close_settings_menu(command_sender),
						None => self.open_settings_menu(command_sender),
					}
				}
			}

			EditorIntent::ContextMenu => if !self.zoomed_out && self.paste_ghosts.len() == 0 {
				self.end_tool(command_sender);
				
				let ms = if self.tool_row_select.1 == usize::MAX {
					self.mouse_state()
				} else {
					MouseState::None
				};

				match ms {
					MouseState::World(x, y) => {
						if self.selection.len() == 0 {
							match self.top_at(x, y) {
								Some(obj) => {
									let mut items = self.get_context_items(obj);
									if let ObjectID::Object(id) = obj {
										let level = self.level.lock().unwrap();
										if level.objects[id].can_delete() {
											items.push(ContextMenuItem::LabeledIcon(sprite::DELETE, LOC_DELETE.for_current_locale_static(), 0x5555ff));
										}
									} else {
										items.push(ContextMenuItem::LabeledIcon(sprite::DELETE, LOC_DELETE.for_current_locale_static(), 0x5555ff));
									}
									if items.len() != 0 {
										let (msx, msy) = self.mouse_screen_pos();
										self.context_menu.target = ContextMenuTarget::Object(obj);
										self.context_menu.open(command_sender, msx, msy, items);
									}
								}
								None => ()
							}
						} else {
							let mut items = vec![];
							for obj in self.selection.keys() {
								if items.len() == 0 {
									items = self.get_context_items(*obj);
								} else {
									let items2 = self.get_context_items(*obj);
									items = items.into_iter()
										.filter_map(|i| items2
											.iter()
											.filter_map(|i2| i.combine(i2))
											.next()
										).collect();
								}

								if items.len() == 0 {
									break;
								}
							}

							let level = self.level.lock().unwrap();

							if self.selection.keys().any(|k| match k {
								ObjectID::Object(id) => level.objects[*id].can_delete(),
								ObjectID::SubObject(_, _) => true
							}) {
								items.push(ContextMenuItem::LabeledIcon(sprite::DELETE, LOC_DELETE.for_current_locale_static(), 0x5555ff))
							}

							if items.len() != 0 {
								let (msx, msy) = self.mouse_screen_pos();
								self.context_menu.target = ContextMenuTarget::Selection;
								self.context_menu.open(command_sender, msx, msy, items);
							}
						}
					}
					MouseState::UI(x, y) => {
						if let Some(tool) = self.ui_top_bar.get_tool_at(x, y) {
							let items = tool.context_menu_items(self.level.lock().unwrap().theme);
							if items.len() != 0 {
								self.context_menu.target = ContextMenuTarget::Tool(tool);
								self.context_menu.open(command_sender, x, y, items);
							}
						}
					}
					_ => ()
				}
			}

			EditorIntent::ZoomToggle => self.handle_intent(command_sender, if self.zoomed_out { EditorIntent::ZoomIn } else { EditorIntent::ZoomOut }),

			EditorIntent::ZoomIn => if self.zoomed_out && !self.block_zoom && self.active_tool.is_none() {
				self.block_zoom = true;

				let mut current = 480 * ZOOM_FACTOR;

				let focus = match self.mouse_state() {
					MouseState::World(x, y) => (
						(x * 32.0 - self.vx) / current as f32,
						(y * 32.0 - self.vy) / (current * 9 / 16) as f32
					),
					_ => (0.5, 0.5)
				};

				self.zoomed_out = false;
				self.ui_top_bar.disable_zoom(command_sender);
				
				for i in 1..ZOOM_FACTOR {
					if i != 1 {
						command_sender.send(Command::Yield);
					}
					self.zoom_from_to(command_sender, current, current - 480, focus);
					current -= 480;
				}

				self.ui_top_bar.enable(command_sender);
				self.bg_rect.set_alpha(command_sender, 0.0);
			}

			EditorIntent::ZoomOut => if !self.zoomed_out && self.ui_top_bar.enabled() && !self.block_zoom && self.active_tool.is_none() {
				self.block_zoom = true;

				let focus = match self.mouse_state() {
					MouseState::World(x, y) => (
						(x * 32.0 - self.vx) / 480.0,
						(y * 32.0 - self.vy) / 270.0
					),
					_ => (0.5, 0.5)
				};

				self.zoomed_out = true;
				self.ui_top_bar.disable(command_sender);
				self.bg_rect.set_alpha(command_sender, 1.0);

				let mut current = 480;
				for i in 1..ZOOM_FACTOR {
					if i != 1 {
						command_sender.send(Command::Yield);
					}
					self.zoom_from_to(command_sender, current, current + 480, focus);
					current += 480;
				}

				self.ui_top_bar.enable_zoom(command_sender);
			}

			EditorIntent::Copy |
			EditorIntent::Cut => if self.selection.len() != 0 {
				let mut selection_bounds = AABB::null();
				let mut level = self.level.lock().unwrap();
				let mut socs = vec![0; self.objects.len()];

				for object in self.selection.keys() {
					selection_bounds |= match *object {
						ObjectID::Object(id) => level.objects[id].bounding_box(),
						ObjectID::SubObject(id, sub_object) => {
							socs[id] += 1;
							level.objects[id].sub_object_bounding_box(sub_object)
						}
					}
				}

				selection_bounds = selection_bounds.expand_to_tile();

				let cx = (selection_bounds.x + selection_bounds.width * 0.5).floor() as i32;
				let cy = (selection_bounds.y + selection_bounds.height * 0.5).floor() as i32;

				let mut buf = vec![];

				for object in self.selection.keys() {
					match *object {
						ObjectID::SubObject(0, sub_object) => {
							buf.push(255);
							let (x, y) = TileManager::from_sub_id(sub_object);
							let _ = buf.write(&((x - cx) as i16).to_le_bytes());
							let _ = buf.write(&((y - cy) as i16).to_le_bytes());
							buf.push(level.tile_manager().get(x, y) as u8);
						}
						
						ObjectID::Object(id) => {
							let object = &mut level.objects[id];
							if object.can_delete() {
								object.move_by(-cx, -cy);
								buf.push(object.serialized_type());
								buf.push(0);
								let len_start = buf.len();
								let _ = object.serialize_inner(&mut buf);
								match (buf.len() - len_start).try_into() {
									Ok(byte) => buf[len_start - 1] = byte,
									Err(_) => buf.truncate(len_start - 2)
								}
								object.move_by(cx, cy);
							}
						}

						ObjectID::SubObject(id, 0) => {
							let object = &mut level.objects[id];
							if socs[id] == object.sub_object_count() {
								object.move_by(-cx, -cy);
								buf.push(object.serialized_type());
								buf.push(0);
								let len_start = buf.len();
								let _ = object.serialize_inner(&mut buf);
								match (buf.len() - len_start).try_into() {
									Ok(byte) => buf[len_start - 1] = byte,
									Err(_) => buf.truncate(len_start - 2)
								}
								object.move_by(cx, cy);
							}
						}

						_ => continue
					}
				}

				drop(level);

				if buf.len() != 0 {
					let mut compressed_vec = Vec::with_capacity(buf.len() + 8);
					if let Ok(_) = Compress::new(Compression::best(), false)
						.compress_vec(&buf, &mut compressed_vec, FlushCompress::Finish) {
						
						let _ = compressed_vec.write(&(buf.len() as u32).to_le_bytes());
						let _ = self.clipboard.set_contents(format!("<BANKI>{}</BANKI>", BASE64_STANDARD_NO_PAD.encode(compressed_vec)));

						if intent == EditorIntent::Cut {
							self.handle_intent(command_sender, EditorIntent::MassDelete);
						}
					}
				}
			}

			EditorIntent::Paste => if !self.is_tool_dropdown_open() && self.paste_ghosts.len() == 0 {
				let Ok(data) = self.clipboard.get_contents() else {return};
				let Some(start) = data.find("<BANKI>") else {return};
				let Some(end) = data.find("</BANKI>") else {return};
				if end < start {return}
				let Ok(cdata) = BASE64_STANDARD_NO_PAD.decode(data[start+7..end].replace(char::is_whitespace, "")) else {return};
				if cdata.len() < 5 {return}
				let lm4 = cdata.len() - 4;
				let mut data = Vec::with_capacity(u32::from_le_bytes(cdata[lm4..].try_into().unwrap()).min(1 << 20) as usize);
				if Decompress::new(false).decompress_vec(&cdata[..lm4], &mut data, FlushDecompress::Finish).is_err() {return}

				self.end_tool(command_sender);
				let level = self.level.lock().unwrap();

				let mut i = 0;
				while i + 1 < data.len() {
					match data[i] {
						255 => {
							if i + 5 < data.len() {
								let tile = Tile::parse(data[i + 5]);
								let x = i16::from_le_bytes(data[i + 1..i + 3].try_into().unwrap());
								let y = i16::from_le_bytes(data[i + 3..i + 5].try_into().unwrap());
								let mut ghost = GameObject::new(game_object::OBJ_CURSOR, -8000);
								ghost.create(command_sender);
								ghost.set_alpha(command_sender, 0.8);
								ghost.set_sprite(command_sender, tile.sprite(level.theme));
								ghost.set_real(command_sender, 0, x as f32 * 32.0 - 16.0);
								ghost.set_real(command_sender, 1, y as f32 * 32.0 - 16.0);
								self.paste_ghosts.push(PasteGhost { ghosts: [ghost, GameObject::null()], data: PasteGhostData::Tile(x, y, tile)});
								i += 6;
							} else {break}
						}

						object_type => {
							let len = data[i + 1] as usize;
							if i + len + 1 < data.len() {
								let Ok(level_object) =
									Level::deserialize_object(object_type, &data[i + 2..i + 2 + len]) else {break};
								if level_object.can_delete() {
									let mut eo = level_object.create_editor_view(command_sender, &level);
									let mut ghosts;
									if level_object.sub_object_count() == 2 {
										ghosts = [
											GameObject::new(game_object::OBJ_CURSOR, -9100),
											GameObject::new(game_object::OBJ_CURSOR, -9100)
										];
										for i in 0..2 {
											ghosts[i].create(command_sender);
											ghosts[i].set_alpha(command_sender, 0.8);
											let ((x, y), sprite) = level_object.ghost_sub_object(command_sender, i, &mut eo);
											ghosts[i].set_real(command_sender, 0, (x * 32 - 16) as f32);
											ghosts[i].set_real(command_sender, 1, (y * 32 - 16) as f32);
											ghosts[i].set_sprite(command_sender, sprite);
										}
									} else if eo.len() > 0 && eo[0].object_type == game_object::OBJ_NO_ANIM {
										let go = &eo[0];
										ghosts = [GameObject::new(game_object::OBJ_CURSOR, -9000 + go.depth), GameObject::null()];
										ghosts[0].create(command_sender);
										ghosts[0].set_alpha(command_sender, 0.8);
										ghosts[0].set_colour(command_sender, go.colour);
										ghosts[0].set_rotation(command_sender, go.rotation);
										ghosts[0].set_real(command_sender, 0, go.x - 16.0);
										ghosts[0].set_real(command_sender, 1, go.y - 16.0);
										ghosts[0].set_sprite(command_sender, go.sprite);
									} else {
										ghosts = [GameObject::null(), GameObject::null()];
									}
									level_object.destroy_editor_view(command_sender, &mut eo, &level);
									self.paste_ghosts.push(PasteGhost { ghosts, data: PasteGhostData::Object(level_object)});
								}
								i += len + 2;
							}
						}
					}
				}

				if self.paste_ghosts.len() > 0 {
					self.ui_top_bar.disable(command_sender);
				}
			}

			EditorIntent::PanUp => self.pan(command_sender, 0.0, -PAN_SPEED),
			EditorIntent::PanDown => self.pan(command_sender, 0.0, PAN_SPEED),
			EditorIntent::PanLeft => self.pan(command_sender, -PAN_SPEED, 0.0),
			EditorIntent::PanRight => self.pan(command_sender, PAN_SPEED, 0.0),

			_ => ()
		}
	}

	fn pan(&mut self, command_sender: &mut CommandSender, dx: f32, dy: f32) {
		self.vx += dx;
		self.vy += dy;
		command_sender.send(Command::F32(vec![self.vx, self.vy]));
		command_sender.send(Command::MoveCamera);
	}

	fn clear_paste_ghosts(&mut self, command_sender: &mut CommandSender) {
		for mut pg in mem::take(&mut self.paste_ghosts) {
			pg.ghosts[0].destroy(command_sender);
			pg.ghosts[1].destroy(command_sender);
		}
	}

	fn is_tool_dropdown_open(&self) -> bool {
		self.tool_row_select.1 != usize::MAX
	}

	fn set_tool(&mut self, _command_sender: &mut CommandSender, tool: Arc<dyn Tool + Send + Sync>) {
		self.selected_tool = tool;
	}

	fn end_tool(&mut self, command_sender: &mut CommandSender) {
		if let Some(tool) = self.active_tool.take() {
			if Arc::ptr_eq(&tool, &self.selected_tool) {
				tool.use_end(command_sender, self);
				self.current_undo_frame_open = false;
			}
		}

		if self.drag_tool.is_some() {
			self.drag_tool = None;
			self.drag_ghost.set_sprite(command_sender, -1);
		}
	}

	pub fn delete(&mut self, command_sender: &mut CommandSender, obj: ObjectID) -> Option<UndoAction> {
		self.set_selected(command_sender, vec![]);

		let mut level = self.level.lock().unwrap();
		match obj {
			ObjectID::Object(id) => {
				if level.objects[id].can_delete() {
					level.objects[id].destroy_editor_view(command_sender, &mut self.objects[id], &level);
					self.objects.remove(id);
					level.object_keys.remove(id);
					Some(UndoAction::AddObject(id, level.objects.remove(id)))
				} else {None}
			}
			
			ObjectID::SubObject(id, sub_id) =>
			match level.objects[id].delete_sub_object(command_sender, &mut self.objects[id], id, sub_id) {
				SubObjectDeleteUndoAction::None => None,
				SubObjectDeleteUndoAction::Some(x) => Some(x),
				SubObjectDeleteUndoAction::DeleteMain => {
					drop(level);
					self.delete(command_sender, ObjectID::Object(id))
				}
			}
		}
	}

	pub fn add(&mut self, command_sender: &mut CommandSender, obj: Box<dyn LevelObject + Send>) -> ObjectID {
		let mut level = self.level.lock().unwrap();
		self.objects.push(obj.create_editor_view(command_sender, &level));
		level.objects.push(obj);
		level.fill_object_keys();
		ObjectID::Object(level.objects.len() - 1)
	}

	pub fn top_at(&self, x: f32, y: f32) -> Option<ObjectID> {
		let level = self.level.lock().unwrap();
		level.atf(x, y).into_iter().min_by_key(|o| match self.objects[o.id()].first() {
			Some(go) => go.depth,
			None => 32000
		})
	}

	fn save(&mut self) -> anyhow::Result<()> {
		self.last_save = Instant::now();
		let mut level = self.level.lock().unwrap();
		let path = level.get_filepath().to_path_buf();
		let tpath = path.with_extension("tmp");

		let mut file = File::create(&tpath)?;
		level.serialize(&mut file)?;
		rename(tpath, path)?;

		Ok(())
	}

	fn save_with_log(&mut self, command_sender: &mut CommandSender) {
		if let Err(err) = self.save() {
			command_sender.send(Command::Log(format!("Failed to save level: {}", err)))
		}
	}

	fn zoom_from_to(&mut self, command_sender: &mut CommandSender, from: u32, to: u32, focus: (f32, f32)) {
		self.vx += focus.0 * (from as i32 - to as i32) as f32;
		self.vy += focus.1 * (from as i32 - to as i32) as f32 * (9.0 / 16.0);
		command_sender.send(Command::F32(vec![self.vx, self.vy]));
		command_sender.send(Command::MoveCamera);
		self.cursor_go.set_scale(command_sender, to as f32 / 480.0, to as f32 / 480.0);
		command_sender.send(Command::Zoom(to));
	}

	fn set_selected(&mut self, command_sender: &mut CommandSender, selection: Vec<ObjectID>) {
		if selection.len() == 0 && self.selection.len() == 0 {
			return;
		}

		let mut to_remove: HashSet<ObjectID> = self.selection.keys().cloned().collect();

		let level = self.level.lock().unwrap();

		for obj in selection {
			to_remove.remove(&obj);
			self.selection.entry(obj).or_insert_with_key(|oid| {
				let mut go = GameObject::new(game_object::OBJ_YELLOW_BOX, -8000);
				go.create(command_sender);
				
				let mut bbox = match oid {
					ObjectID::Object(id) => {
						let obj = &level.objects[*id];
						obj.bounding_box()
						
					}
					ObjectID::SubObject(id, sid) => {
						let obj = &level.objects[*id];
						obj.sub_object_bounding_box(*sid)
					}
				};

				bbox.shrink_if_tile();
				bbox = bbox.times(32.0);
				//bbox.x -= 1.0;
				//bbox.y -= 1.0;

				go.set_real(command_sender, 0, bbox.x);
				go.set_real(command_sender, 1, bbox.y);
				go.set_real(command_sender, 2, bbox.x + bbox.width);
				go.set_real(command_sender, 3, bbox.y + bbox.height);

				go
			});
		}

		for obj in to_remove {
			self.selection.remove(&obj).unwrap().destroy(command_sender);
		}
	}

	fn settings_menu_primary(&mut self, command_sender: &mut CommandSender, row: i32, col: u32) {
		let mut flash_button = usize::MAX;

		match (col, row) {
			(0, 0) => {
				self.handle_intent(command_sender, EditorIntent::Save);
				flash_button = 2;
			}

			(0, 1) => self.handle_intent(command_sender, EditorIntent::Exit),

			(0, 2) => {
				self.handle_intent(command_sender, EditorIntent::UseTool(4));
				flash_button = 4;
			}

			(0, 3) => {
				self.handle_intent(command_sender, EditorIntent::UseTool(5));
				flash_button = 5;
			}

			(0, 4) => if self.level.lock().unwrap().can_publish() {
				sound::play(command_sender, sound::SE_DECIDE);
				let mut goal = GameObject::new(game_object::OBJ_GOAL, 0);
				goal.x = 3200000.0;
				goal.create(command_sender);
				goal.destroy_server_only();
				let mut transition = GameObject::new(game_object::OBJ_QUICKRETRY, -15990);
				transition.create(command_sender);
				for _ in 0..30 {
					command_sender.send(Command::Yield);
				}
				*SAVED_HISTORY.lock().unwrap() = (
					mem::take(&mut self.undo_frames),
					mem::take(&mut self.redo_frames)
				);
				command_sender.send(Command::GotoRoom(controller::EDITOR_ROOM));
				InternalCommand::SwitchToMenu(PrePublish::new(self.level.clone(), true)).run();
				//InternalCommand::SwitchToMenu(super::publish::Publish::new(self.level.clone())).run();
			} else {
				sound::play(command_sender, sound::SE_NOT);
				flash_button = 18;
			}

			(1, 0) => {
				let settings_menu = self.settings_menu.as_mut().unwrap();
				settings_menu.name_focused = true;
				settings_menu.sent_name_unfocus = false;
				settings_menu.initial_name = self.level.lock().unwrap().name.clone();
				settings_menu.objects[14].set_real(command_sender, 0, 1.0);
				sound::play(command_sender, sound::SE_MESSAGE);
			}

			(2, 1) => {
				let mut level = self.level.lock().unwrap();
				if level.heads != 0 {
					level.heads -= 1;
					sound::play(command_sender, sound::SE_SELECT);
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
				flash_button = 6;
			}

			(1, 1) => {
				let mut level = self.level.lock().unwrap();
				if level.heads < 9 || level.character() == Character::Seija {
					level.heads += 1;
					sound::play(command_sender, sound::SE_SELECT);
				} else {
					sound::play(command_sender, sound::SE_NOT);
				}
				flash_button = 7;
			}

			(2, 2) => {
				let mut level = self.level.lock().unwrap();
				let second = level.music != level.theme.bgm(false);
				level.theme = level.theme.prev();
				flash_button = 10;
				if level.music != 0 {
					level.music = level.theme.bgm(second);
					if self.music_preview {
						self.music_preview = false;
						sound::set_bgm(command_sender, sound::BGM_EDITOR_MENU);
					}
				}
				sound::play(command_sender, sound::SE_SELECT);
			}

			(1, 2) => {
				let mut level = self.level.lock().unwrap();
				let second = level.music != level.theme.bgm(false);
				level.theme = level.theme.next();
				flash_button = 11;
				if level.music != 0 {
					level.music = level.theme.bgm(second);
					if self.music_preview {
						self.music_preview = false;
						sound::set_bgm(command_sender, sound::BGM_EDITOR_MENU);
					}
				}
				sound::play(command_sender, sound::SE_SELECT);
			}

			(x, 3) => {
				let mut level = self.level.lock().unwrap();
				level.music = match x {
					1 => level.theme.bgm(true),
					2 => level.theme.bgm(false),
					_ => 0
				};
				sound::set_bgm(command_sender, level.music);
				self.music_preview = true;
			}

			_ => ()
		}

		if flash_button != usize::MAX {
			self.settings_menu.as_mut().unwrap().objects[flash_button].set_colour(command_sender, 0xffff);
			command_sender.send(Command::Yield);
			command_sender.send(Command::Yield);
			command_sender.send(Command::Yield);
			command_sender.send(Command::Yield);
		}
	}

	fn on_event_settings_menu(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) {
		let mpos = self.mouse_screen_pos();
		let mut settings_menu = self.settings_menu.as_mut().unwrap();

		let mut ignore = false;

		match event {
			Event::MouseDown => {
				if mpos.0 < 80.0 && !settings_menu.name_focused {
					return self.close_settings_menu(command_sender);
				}

				if settings_menu.name_focused {
					if settings_menu.selected_row == 0 && settings_menu.selected_col == 1 {
						settings_menu.objects[14].set_real(command_sender, 3, 2.0);
					} else {
						settings_menu.name_focused = false;
					}
				}

				if settings_menu.selected_row >= 0 {	
					self.on_event_settings_menu(command_sender, Event::MouseMove, global_state);
					settings_menu = self.settings_menu.as_mut().unwrap();
					let row = settings_menu.selected_row;
					let col = settings_menu.selected_col;
					self.settings_menu_primary(command_sender, row, col);
					settings_menu = self.settings_menu.as_mut().unwrap();
				}
			}

			Event::KeyDown |
			Event::ButtonDown => {
				if !settings_menu.name_focused &&
				control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) == Some(MenuIntent::GoBack) {
					sound::play(command_sender, sound::SE_BREAK);
					return self.close_settings_menu(command_sender);
				}
				if event == Event::KeyDown {
					settings_menu.key_down = true;
				}
			},

			Event::KeyUp |
			Event::ButtonUp => if settings_menu.name_focused {
				match control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) {
					Some(MenuIntent::Primary) |
					Some(MenuIntent::GoBack) => {
						settings_menu.name_focused = false;
						settings_menu.selected_row = 0;
						settings_menu.selected_col = 1;
					}
					_ => ignore = true
				}
			} else {
				match control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) {
					Some(MenuIntent::SelectLeft) => if settings_menu.selected_col < settings_menu_row_size(settings_menu.selected_row) {
						settings_menu.selected_col += 1;
					}

					Some(MenuIntent::SelectRight) => if settings_menu.selected_col > 0 {
						settings_menu.selected_col -= 1;
					}

					Some(MenuIntent::SelectUp) => {
						if settings_menu.selected_row > 0 {
							settings_menu.selected_row -= 1;
						} else {
							settings_menu.selected_row = SETTINGS_MENU_MAX_ROW;
						}

						settings_menu.selected_col = settings_menu.selected_col.min(settings_menu_row_size(settings_menu.selected_row));
					}

					Some(MenuIntent::SelectDown) => {
						let old_row_size = settings_menu_row_size(settings_menu.selected_row);
						settings_menu.selected_row = (settings_menu.selected_row + 1) % (SETTINGS_MENU_MAX_ROW + 1);
						settings_menu.selected_col = if old_row_size == settings_menu.selected_col {
							settings_menu_row_size(settings_menu.selected_row)
						} else {
							settings_menu.selected_col.min(settings_menu_row_size(settings_menu.selected_row))
						};
					}

					Some(MenuIntent::Primary) => if event == Event::ButtonUp || settings_menu.key_down {
						let row = settings_menu.selected_row;
						let col = settings_menu.selected_col;
						self.settings_menu_primary(command_sender, row, col);
						settings_menu = self.settings_menu.as_mut().unwrap();
					}

					_ => ignore = true
				}
				if event == Event::KeyUp {
					settings_menu.key_down = false;
				}
			}

			Event::InputUnfocus => settings_menu.name_focused = false,

			Event::MouseMove => {
				let was_anything_selected_before = settings_menu.selected_row >= 0;

				let iy = mpos.1 as i32 - 8;

				settings_menu.selected_row = if iy < 168 && iy % 44 < 36 {
					iy / 44
				} else if iy > 196 && iy < 234 && mpos.0 >= 178.0 && mpos.0 < 386.0 { 4 } else { -1 };
				
				if (mpos.0 >= 436.0 && mpos.0 < 472.0) || settings_menu.selected_row == 4 {
					settings_menu.selected_col = 0;
				}

				else if (mpos.0 >= 344.0 || (settings_menu.selected_row == 0 && mpos.0 >= 160.0)) && mpos.0 < 362.0 {
					settings_menu.selected_col = 1;
				}

				else if settings_menu.selected_row == 3 && mpos.0 >= 160.0 && mpos.0 < 365.0 && ((mpos.0 as u32 - 77) % 83 < 36) {
					settings_menu.selected_col = (446 - mpos.0 as u32) / 83;
				}

				else if mpos.0 >= 160.0 && mpos.0 < 178.0 {
					settings_menu.selected_col = 2;
				}

				else {
					settings_menu.selected_row = -1
				}

				ignore = !was_anything_selected_before && settings_menu.selected_row < 0;
			}

			Event::GetString => if Level::is_name_valid(&global_state.recieved_string) {
				self.level.lock().unwrap().name = mem::take(&mut global_state.recieved_string);
			}

			Event::Tick => if settings_menu.has_ticked {
				ignore = true;
				if settings_menu.name_focused {
					settings_menu.objects[14].query_string(command_sender, 0);
				} else if !settings_menu.sent_name_unfocus {
					settings_menu.objects[14].set_real(command_sender, 0, 0.0);
					settings_menu.sent_name_unfocus = true;
				}
			} else {
				settings_menu.has_ticked = true;
			}

			_ => ignore = true
		}

		if !ignore {
			for i in 0..4 {
				settings_menu.objects[i as usize + 2].set_colour(command_sender,
					if (i == 2 && self.undo_frames.len() == 0) || (i == 3 && self.redo_frames.len() == 0) { 0x808080 }
					else if settings_menu.selected_col == 0 && settings_menu.selected_row == i { 0xf2d5e3 }
					else { 0xffffff }
				);
			}

			let level = self.level.lock().unwrap();

			settings_menu.objects[6].set_colour(command_sender,
				if level.heads == 0 { 0x808080 }
				else if settings_menu.selected_col == 2 && settings_menu.selected_row == 1 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[7].set_colour(command_sender,
				if level.heads == 9 && level.character() != Character::Seija { 0x808080 }
				else if settings_menu.selected_col == 1 && settings_menu.selected_row == 1 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[10].set_colour(command_sender,
				if settings_menu.selected_col == 2 && settings_menu.selected_row == 2 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[11].set_colour(command_sender,
				if settings_menu.selected_col == 1 && settings_menu.selected_row == 2 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[9].set_string(command_sender, 0, &format!("{} {}", match level.character() {
				Character::Banki => "HEAD",
				Character::Cirno => "ICE",
				Character::Rumia => "DARK",
				Character::Seija => "COST",
			}, level.heads));
			update_theme_name(command_sender, level.theme, &mut settings_menu.objects[12]);

			settings_menu.objects[13].set_colour(command_sender,
				if settings_menu.name_focused { 0xffff }
				else if settings_menu.selected_col == 1 && settings_menu.selected_row == 0 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[15].set_colour(command_sender,
				if level.music == 0 { 0xffff }
				else if settings_menu.selected_row == 3 && settings_menu.selected_col == 3 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[16].set_colour(command_sender,
				if level.music == level.theme.bgm(false) { 0xffff }
				else if settings_menu.selected_row == 3 && settings_menu.selected_col == 2 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[17].set_colour(command_sender,
				if level.theme.bgm(false) == level.theme.bgm(true) { 0x808080 }
				else if level.music == level.theme.bgm(true) { 0xffff }
				else if settings_menu.selected_row == 3 && settings_menu.selected_col == 1 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[18].set_colour(command_sender,
				if !level.can_publish() { 0x808080 }
				else if settings_menu.selected_col == 0 && settings_menu.selected_row == 4 { 0xf2d5e3 }
				else { 0xffffff }
			);

			settings_menu.objects[19].set_string(command_sender, 0, LOC_PUBLISH.for_current_locale_static());
		}
	}

	fn open_settings_menu(&mut self, command_sender: &mut CommandSender) {
		let mut bg = GameObject::new(game_object::OBJ_UI, -14010);
		let mut bg2 = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, -14000);

		let bg_id = bg.create(command_sender) as f32;

		bg2.create(command_sender);
		bg2.set_real(command_sender, 1, 480.0);
		bg2.set_real(command_sender, 2, 480.0);
		bg2.set_real(command_sender, 3, 1.0);

		let rb1 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 356.0);
			btn.set_real(command_sender, 1, 8.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let rb2 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 356.0);
			btn.set_real(command_sender, 1, 52.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let rb3 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 356.0);
			btn.set_real(command_sender, 1, 96.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let rb4 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 356.0);
			btn.set_real(command_sender, 1, 140.0);
			btn.set_real(command_sender, 2, bg_id);
			btn.set_colour(command_sender, 0x808080);
			btn
		};

		let head_left = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 72.0);
			btn.set_real(command_sender, 1, 52.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let head_right = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 260.0);
			btn.set_real(command_sender, 1, 52.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let level = self.level.lock().unwrap();
		let mut head_icon = GameObject::new(game_object::OBJ_UI, -14020);
		head_icon.create(command_sender);
		head_icon.set_sprite(command_sender, match level.character() {
			Character::Banki => sprite::HEAD,
			c => sprite::CIRNO_HEAD - 1 + c as i32
		});
		head_icon.set_real(command_sender, 0, 160.0);
		head_icon.set_real(command_sender, 1, 44.0);
		head_icon.set_real(command_sender, 2, bg_id);

		let mut head_text = GameObject::new(game_object::OBJ_TEXT, -14020);
		head_text.x = 261.0 + self.vx;
		head_text.y = 74.0 + self.vy;
		head_text.create(command_sender);
		head_text.set_real(command_sender, 1, 1.0);

		let theme_left = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 72.0);
			btn.set_real(command_sender, 1, 96.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let theme_right = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 260.0);
			btn.set_real(command_sender, 1, 96.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let name_btn = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 72.0);
			btn.set_real(command_sender, 1, 0.0);
			btn.set_real(command_sender, 2, bg_id);
			btn.set_scale(command_sender, 6.0, 4.0 / 3.0);
			btn
		};

		let mut name_text = GameObject::new(game_object::OBJ_TEXTBOX, -14020);
		name_text.x = self.vx + 164.0;
		name_text.y = self.vy + 27.0;
		name_text.create(command_sender);
		name_text.set_real(command_sender, 2, 344.0);
		name_text.set_real(command_sender, 5, 200.0);

		let music_0 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 80.0);
			btn.set_real(command_sender, 1, 140.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let music_1 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0,163.0);
			btn.set_real(command_sender, 1, 140.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let music_2 = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 246.0);
			btn.set_real(command_sender, 1, 140.0);
			btn.set_real(command_sender, 2, bg_id);
			btn
		};

		let mut theme_text = GameObject::new(game_object::OBJ_UI, -14020);
		theme_text.create(command_sender);
		update_theme_name(command_sender, level.theme, &mut theme_text);
		theme_text.set_real(command_sender, 2, bg_id);

		let publish = {
			let mut btn = GameObject::new(game_object::OBJ_UI, -14001);
			btn.create(command_sender);
			btn.set_sprite(command_sender, sprite::TOOL);
			btn.set_real(command_sender, 0, 72.0);
			btn.set_real(command_sender, 1, 200.0);
			btn.set_real(command_sender, 2, bg_id);
			btn.set_scale(command_sender, 8.0, 4.0 / 3.0);
			btn
		};

		let mut publish_text = GameObject::new(game_object::OBJ_TEXT, -14020);
		publish_text.x = 310.0 + self.vx;
		publish_text.y = 223.0 + self.vy;
		publish_text.create(command_sender);
		publish_text.set_real(command_sender, 0, -1.0);
		publish_text.set_real(command_sender, 1, 1.0);
		publish_text.set_real(command_sender, 2, 1.0);
		publish_text.set_scale(command_sender, 1.0, 1.0);
		publish_text.set_colour(command_sender, 0);

		for i in 0..20 {
			let t = 19 - i;
			bg.set_real(command_sender, 0, (80 + t * t) as f32);
			bg2.set_alpha(command_sender, (i + 1) as f32 * 0.025);

			if i == 1 {
				bg.set_sprite(command_sender, sprite::LEVEL_SETTINGS_MENU_BG);
			}

			if i < 19 {
				command_sender.send(Command::Yield);
			} else {
				name_text.set_string(command_sender, 0, &level.name);
			}
		}

		self.settings_menu = Some(SettingsMenu {
			objects: vec![bg, bg2, rb1, rb2, rb3, rb4 /* <= 5 */,
				head_left, head_right, head_icon, head_text, theme_left /* <= 10 */,
				theme_right, theme_text, name_btn, name_text, music_0 /* <= 15 */,
				music_1, music_2, publish, publish_text,
			],
			selected_col: 0,
			selected_row: -1,
			has_ticked: false,
			name_focused: false,
			sent_name_unfocus: true,
			theme: level.theme,
			initial_name: String::new(),
			key_down: false,
		});
	}

	fn close_settings_menu(&mut self, command_sender: &mut CommandSender) {
		let mut settings_menu = None;
		mem::swap(&mut self.settings_menu, &mut settings_menu);
		let mut settings_menu = settings_menu.unwrap();
		settings_menu.objects[9].set_string(command_sender, 0, "");

		if settings_menu.theme != self.level.lock().unwrap().theme {
			self.regenerate(command_sender);
		}

		settings_menu.objects[14].destroy(command_sender);
		settings_menu.objects[19].destroy(command_sender);

		let bgs = settings_menu.objects.as_mut_slice().split_at_mut(1);
		let bg = &mut bgs.0[0];
		let bg2 = &mut bgs.1[0];

		for t in 0..20 {
			bg.set_real(command_sender, 0, (80 + t * t) as f32);
			bg2.set_alpha(command_sender, (20 - t) as f32 * 0.025);

			if t < 19 {
				command_sender.send(Command::Yield);
			}
		}

		for mut object in settings_menu.objects {
			object.destroy(command_sender);
		}

		self.active_tool = None;
		self.grab_start = MouseState::None;
	}

	fn get_context_items(&self, object: ObjectID) -> Vec<ContextMenuItem> {
		let level = self.level.lock().unwrap();
		match object {
			ObjectID::Object(id) => level.objects[id].context_menu_items(level.theme),
			ObjectID::SubObject(id, sub_object) => level.objects[id].sub_object_context_menu_items(sub_object, level.theme)
		}
	}

	fn toggle_tool_select_dropdown(&mut self, command_sender: &mut CommandSender) {
		if self.tool_row_select.1 == usize::MAX {
			self.tool_row_select.1 = 0;

			let mut bg = GameObject::new(game_object::OBJ_UI, -12000);
			let bg_id = bg.create(command_sender) as f32;

			bg.set_sprite(command_sender, sprite::TOOL_DROPDOWN_BG);
			bg.set_real(command_sender, 0, 35.0);
			bg.set_real(command_sender, 1, 41.0);

			for row in 0..ROW_COUNT {
				for col in 0..10 {
					let mut icon = ToolIcon::new(TOOLS.rows[row][col].clone());
					icon.create(command_sender, bg_id, (7 + col * 40) as f32, (7 + row * 40) as f32);
					self.tool_row_select.2.push(icon);
				}
			}

			self.ui_top_bar.dropdown_go.set_sprite(command_sender, sprite::TOOL_DROPDOWN_BUTTON_RETRACT);

			self.tool_row_select.3 = bg;
			self.block_zoom = true;
		} else {
			for icon in &mut self.tool_row_select.2 {
				icon.destroy(command_sender);
			}

			self.tool_row_select.3.destroy(command_sender);

			self.tool_row_select = (usize::MAX, usize::MAX, vec![], GameObject::null());

			self.ui_top_bar.dropdown_go.set_sprite(command_sender, sprite::TOOL_DROPDOWN_BUTTON);
			self.block_zoom = false;
		}
	}
}

fn settings_menu_row_size(row: i32) -> u32 {
	match row {
		0 => 1,
		3 => 3,
		4 => 0,
		_ => 2
	}
}

fn update_theme_name(command_sender: &mut CommandSender, theme: LevelTheme, object: &mut GameObject) {
	object.set_colour(command_sender, if theme == LevelTheme::Purple { 0xa53f74 } else { 0xffffff });
	let loc = Locale::get();
	let (sprite, hw, hh, scale) = match (theme, loc) {
		(LevelTheme::DreamFields, Locale::JP) => (305, 117.9, 22.6, 0.85),
		(LevelTheme::DreamFields, Locale::EN) => (305, 84.7, 22.1, 0.78),
		(LevelTheme::DreamFields, Locale::ZH) => (305, 127.7, 24.5, 0.91),
		(LevelTheme::BambooForest, Locale::JP) => (306, 95.8, 22.4, 0.83),
		(LevelTheme::BambooForest, Locale::EN) => (306, 81.5, 18.3, 0.76),
		(LevelTheme::BambooForest, Locale::ZH) => (306, 128.6, 24.5, 0.91),
		(LevelTheme::AzureWinter, Locale::JP) => (307, 117.8, 22.6, 0.87),
		(LevelTheme::AzureWinter, Locale::EN) => (307, 87.9, 22.6, 0.79),
		(LevelTheme::AzureWinter, Locale::ZH) => (307, 129.1, 24.5, 0.91),
		(LevelTheme::ForestOfMagic, Locale::JP) => (308, 117.9, 22.6, 0.85),
		(LevelTheme::ForestOfMagic, Locale::EN) => (308, 87.9, 23.0, 0.79),
		(LevelTheme::ForestOfMagic, Locale::ZH) => (308, 124.0, 23.6, 0.89),
		(LevelTheme::UltramarineRain, Locale::JP) => (309, 96.7, 22.9, 0.83),
		(LevelTheme::UltramarineRain, Locale::EN) => (309, 80.0, 20.8, 0.74),
		(LevelTheme::UltramarineRain, Locale::ZH) => (309, 102.4, 23.2, 0.86),
		(LevelTheme::OutsideWorld, Locale::JP) => (310, 120.9, 23.5, 0.87),
		(LevelTheme::OutsideWorld, Locale::EN) => (310, 102.4, 25.8, 0.86),
		(LevelTheme::OutsideWorld, Locale::ZH) => (310, 104.1, 23.2, 0.86),
		(LevelTheme::Koumakan, Locale::JP) => (311, 95.8, 23.8, 0.83),
		(LevelTheme::Koumakan, Locale::EN) => (311, 107.1, 18.8, 0.87),
		(LevelTheme::Koumakan, Locale::ZH) => (311, 102.4, 22.8, 0.86),
		(LevelTheme::ShiningNeedleCastle, Locale::JP) => (312, 135.7, 23.4, 0.85),
		(LevelTheme::ShiningNeedleCastle, Locale::EN) => (312, 118.2, 20.0, 0.91),
		(LevelTheme::ShiningNeedleCastle, Locale::ZH) => (312, 145.9, 24.5, 0.91),
		(LevelTheme::DreamScraps, Locale::JP) => (313, 95.0, 23.8, 0.83),
		(LevelTheme::DreamScraps, Locale::EN) => (313, 107.4, 24.8, 0.88),
		(LevelTheme::DreamScraps, Locale::ZH) => (313, 127.3, 24.5, 0.91),
		(LevelTheme::TheDepths, Locale::JP) => (314, 133.9, 23.5, 0.87),
		(LevelTheme::TheDepths, Locale::EN) => (314, 142.1, 29.5, 1.05),
		(LevelTheme::TheDepths, Locale::ZH) => (314, 144.1, 24.5, 0.91),
		(LevelTheme::JerryAttack, Locale::JP) => (394, 94.2, 19.7, 0.68),
		(LevelTheme::JerryAttack, Locale::EN) => (394, 87.9, 20.0, 0.66),
		(LevelTheme::JerryAttack, Locale::ZH) => (394, 113.9, 21.4, 0.71),
		(LevelTheme::FarawayLabyrinth, Locale::JP) => (388, 220.3, 12.2, 1.16),
		(LevelTheme::FarawayLabyrinth, Locale::EN) => (388, 201.9, 11.4, 1.09),
		(LevelTheme::FarawayLabyrinth, Locale::ZH) => (388, 401.9, 17.7, 1.86),
		(LevelTheme::DancingStars, Locale::JP) => (389, 396.3, 19.3, 1.84),
		(LevelTheme::DancingStars, Locale::EN) => (389, 122.1, 8.2, 0.78),
		(LevelTheme::DancingStars, Locale::ZH) => (389, 322.3, 15.5, 1.55),
		(LevelTheme::ReachOutToThatMoon, Locale::JP) => (390, 160.9, 9.8, 0.93),
		(LevelTheme::ReachOutToThatMoon, Locale::EN) => (390, 134.7, 7.9, 0.83),
		(LevelTheme::ReachOutToThatMoon, Locale::ZH) => (390, 318.5, 15.4, 1.54),
		(LevelTheme::MindBreak, Locale::JP) => (392, 225.9, 12.4, 1.18),
		(LevelTheme::MindBreak, Locale::EN) => (392, 382.0, 17.1, 1.80),
		(LevelTheme::MindBreak, Locale::ZH) => (392, 500.0, 22.2, 2.22),
		(LevelTheme::Fireflies, Locale::JP) => (393, 190.1, 10.9, 1.04),
		(LevelTheme::Fireflies, Locale::EN) => (393, 81.2, 6.5, 0.62),
		(LevelTheme::Fireflies, Locale::ZH) => (393, 122.9, 7.8, 0.78),
		(LevelTheme::Cirno, _) => (713, 56.7, 20.0, 0.74),
		(LevelTheme::Rumia, _) => (714, 80.0, 7.7, 0.70),
		(LevelTheme::Seija, _) => (658, 68.1, 20.0, 0.65),
		(LevelTheme::Rasobi, _) |
		(LevelTheme::Purple, _) => (398, 171.1, 29.5, 1.05),
		(LevelTheme::Entrance, Locale::JP) => (303, 82.3, 20.7, 0.77),
		(LevelTheme::Entrance, Locale::EN) => (303, 163.4, 32.6, 1.14),
		(LevelTheme::Entrance, Locale::ZH) => (303, 133.5, 24.7, 0.93),
	};

	object.set_sprite(command_sender, sprite);
	object.set_sprite_frame(command_sender, loc as u32);
	object.set_scale(command_sender, scale, scale);
	object.set_real(command_sender, 0, 181.0 - hw);
	object.set_real(command_sender, 1, 114.0 - hh);
}