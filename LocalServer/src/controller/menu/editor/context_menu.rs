use std::{usize, vec};

use crate::controller::{command_handler::CommandSender, control_settings::{self, MenuIntent}, event::Event, game_object::{self, GameObject}, global_state::ControllerGlobalState, level::ObjectID};

use super::tool::ArcTool;

pub struct ContextMenu {
	pub state: ContextMenuState,
	pub data: Option<ContextMenuData>,
	pub target: ContextMenuTarget,
}

impl Default for ContextMenu {
	fn default() -> Self {
		Self {
			state: ContextMenuState::Closed,
			data: None,
			target: ContextMenuTarget::Selection,
		}
	}
}

pub enum ContextMenuTarget {
	Selection,
	Object(ObjectID),
	Tool(ArcTool)
}

pub struct ContextMenuData {
	x: f32,
	y: f32,
	height: f32,
	bg: GameObject,
	top: GameObject,
	bottom: GameObject,
	left: GameObject,
	right: GameObject,
	items: Vec<ContextMenuItem>,
	item_objects: Vec<Vec<GameObject>>,
	focus_item: usize,
	focus_subitem: usize,
}

const OPEN_CLOSE_FRAMES: u8 = 4;
const DEPTH: i32 = -14300;
const WIDTH: f32 = 82.0;

#[derive(Clone, Copy, PartialEq)]
pub enum ContextMenuState {
	Closed, Open, Opening(u8), Closing(u8)
}

#[derive(Clone)]
pub enum ContextMenuItem {
	LabeledIcon(i32, &'static str, u32),
	IconList(Vec<i32>, usize, f32),
}

impl ContextMenuItem {
	pub fn combine(&self, other: &Self) -> Option<Self> {
		match (self, other) {
			(Self::LabeledIcon(s1, text, colour), Self::LabeledIcon(s2, _, _)) => if s1 == s2 {
				Some(Self::LabeledIcon(*s1, text, *colour))
			} else { None }
			(Self::IconList(v1, h1, scale), Self::IconList(v2, h2, _)) => if v1 == v2 {
				Some(Self::IconList(v1.clone(), if h1 == h2 { *h1 } else { usize::MAX }, *scale))
			} else { None }
			_ => None
		}
	}

	fn height(&self) -> f32 {
		match self {
			Self::IconList(v, _, _) => 20.0 * (((v.len() - 1) / 4) + 1) as f32,
			Self::LabeledIcon(_, _, _) => 20.0
		}
	}
}

#[derive(Clone, Copy)]
enum Direction {
	None, Mouse, Up, Down, Left, Right
}

impl ContextMenu {
	fn force_close_now(&mut self, command_sender: &mut CommandSender) {
		if self.state != ContextMenuState::Closed {
			let data = self.data.as_mut().unwrap();

			data.bg.destroy(command_sender);
			data.top.destroy(command_sender);
			data.left.destroy(command_sender);
			data.bottom.destroy(command_sender);
			data.right.destroy(command_sender);

			if self.state == ContextMenuState::Open {
				for outer in &mut data.item_objects {
					for inner in outer {
						inner.destroy(command_sender);
					}
				}
			}

			self.state = ContextMenuState::Closed;
			self.data = None;
		}
	}

	pub fn open(&mut self,
		command_sender: &mut CommandSender,
		mut x: f32,
		mut y: f32,
		items: Vec<ContextMenuItem>
	) {
		self.force_close_now(command_sender);

		let height = items.iter().map(|i| i.height()).sum::<f32>() + 2.0;

		if x > 477.0 - WIDTH {
			x -= WIDTH;
		}

		y = y.min(267.0 - height);

		let mut bg = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH);
		bg.x = x;
		bg.y = y;
		bg.create(command_sender);
		bg.set_real(command_sender, 3, 1.0);
		bg.set_alpha(command_sender, 0.8);
		bg.set_real(command_sender, 1, WIDTH);

		let mut top = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH);
		top.x = x;
		top.y = y - 1.0;
		top.create(command_sender);
		top.set_real(command_sender, 3, 1.0);
		top.set_real(command_sender, 2, 1.0);
		top.set_real(command_sender, 1, WIDTH);
		
		let mut bottom = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH);
		bottom.x = x;
		bottom.create(command_sender);
		bottom.set_real(command_sender, 3, 1.0);
		bottom.set_real(command_sender, 2, 1.0);
		bottom.set_real(command_sender, 1, WIDTH);

		let mut left = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH);
		left.x = x - 1.0;
		left.y = y;
		left.create(command_sender);
		left.set_real(command_sender, 3, 1.0);
		left.set_real(command_sender, 1, 1.0);

		let mut right = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH);
		right.x = x + WIDTH;
		right.y = y;
		right.create(command_sender);
		right.set_real(command_sender, 3, 1.0);
		right.set_real(command_sender, 1, 1.0);

		let il = items.len();

		self.state = ContextMenuState::Opening(1);
		self.data = Some(ContextMenuData {
			x, y, height,
			bg, top, bottom, left, right,
			items, 
			item_objects: Vec::with_capacity(il),
			focus_item: usize::MAX,
			focus_subitem: 0,
		});
	}

	pub fn handle_event(&mut self, command_sender: &mut CommandSender, event: Event, global_state: &mut ControllerGlobalState) -> i32 {
		match self.state {
			ContextMenuState::Closed => 0,
			ContextMenuState::Opening(x) => {
				if event == Event::Tick {
					let data = self.data.as_mut().unwrap();
					let height = data.height * x as f32 / OPEN_CLOSE_FRAMES as f32;

					data.bg.set_real(command_sender, 2, height);
					data.left.set_real(command_sender, 2, height);
					data.right.set_real(command_sender, 2, height);
					data.bottom.y = data.y + height;
					data.bottom.update_position(command_sender);

					self.state = if x == OPEN_CLOSE_FRAMES {

						let x = data.x + 3.0;
						let mut y = data.y + 2.0;
						for item in &data.items {
							data.item_objects.push(match item {
								ContextMenuItem::LabeledIcon(sprite, label, colour) => {
									let mut bg = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH - 1);
									bg.x = x;
									bg.y = y;
									bg.create(command_sender);
									bg.set_alpha(command_sender, 0.0);
									bg.set_real(command_sender, 0, 12303291.0);
									bg.set_real(command_sender, 1, WIDTH - 5.0);
									bg.set_real(command_sender, 2, 18.0);
									bg.set_real(command_sender, 3, 1.0);

									let mut icon = GameObject::new(game_object::OBJ_UI, DEPTH - 2);
									icon.x = 100000.0;
									icon.create(command_sender);
									icon.set_real(command_sender, 0, x + 1.0);
									icon.set_real(command_sender, 1, y + 1.0);
									icon.set_sprite(command_sender, *sprite);

									let mut text = GameObject::new(game_object::OBJ_TEXT, DEPTH - 2);
									text.x = x + 19.0 + global_state.view_x;
									text.y = y + 4.0 + global_state.view_y;
									text.create(command_sender);
									text.set_string(command_sender, 0, label);
									text.set_real(command_sender, 0, 1.0);
									text.set_colour(command_sender, *colour);

									vec![bg, icon, text]
								}

								ContextMenuItem::IconList(list, highlight, scale) => {
									let mut gos = Vec::with_capacity(list.len() * 2);

									for (i, item) in list.iter().enumerate() {
										let x = x + (i % 4 * 20) as f32;
										let y = y + (i / 4 * 20) as f32;

										let mut bg = GameObject::new(game_object::OBJ_FILLED_RECTANGLE, DEPTH - 1);
										bg.x = x;
										bg.y = y;
										bg.create(command_sender);
										bg.set_alpha(command_sender, if *highlight == i { 0.25 } else { 0.0 });
										bg.set_real(command_sender, 0, 12303291.0);
										bg.set_real(command_sender, 1, 17.0);
										bg.set_real(command_sender, 2, 17.0);
										bg.set_real(command_sender, 3, 1.0);
										gos.push(bg);

										let mut icon = GameObject::new(game_object::OBJ_UI, DEPTH - 2);
										icon.x = 100000.0;
										icon.create(command_sender);
										icon.set_scale(command_sender, *scale, *scale);
										icon.set_sprite(command_sender, *item);
										icon.set_real(command_sender, 0, x + 1.0);
										icon.set_real(command_sender, 1, y + 1.0);
										gos.push(icon);
									}

									gos
								}
							});

							y += item.height();
						}

						ContextMenuState::Open
					} else {
						ContextMenuState::Opening(x + 1)
					};
				}

				1
			}

			ContextMenuState::Closing(x) => {
				if event == Event::Tick {
					if x == 0 {
						self.force_close_now(command_sender);
					} else {
						let data = self.data.as_mut().unwrap();
						let height = data.height * x as f32 / OPEN_CLOSE_FRAMES as f32;

						data.bg.set_real(command_sender, 2, height);
						data.left.set_real(command_sender, 2, height);
						data.right.set_real(command_sender, 2, height);
						data.bottom.y = data.y + height;
						data.bottom.update_position(command_sender);
						self.state = ContextMenuState::Closing(x - 1);
					}
				}

				1
			}
			
			ContextMenuState::Open => {
				let data = self.data.as_mut().unwrap();
				let mut move_dir = Direction::None;

				match event {
					Event::MouseDown => {
						let mx = global_state.mouse_x - global_state.view_x;
						let my = global_state.mouse_y - global_state.view_y;

						if mx < data.x - 1.0 || my < data.y - 1.0 || mx > data.x + WIDTH || my > data.y + data.height {
							self.close(command_sender);
							return if control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) == Some(MenuIntent::Primary) {1} else {0};
						}

						move_dir = match control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) {
							Some(MenuIntent::SelectLeft) => Direction::Left,
							Some(MenuIntent::SelectRight) => Direction::Right,
							Some(MenuIntent::SelectUp) => Direction::Up,
							Some(MenuIntent::SelectDown) => Direction::Down,
							_ => Direction::None
						}
					}

					Event::MouseMove => {
						let mx = global_state.mouse_x - global_state.view_x - data.x;
						let mut my = global_state.mouse_y - global_state.view_y - data.y;
						let old = (data.focus_item, data.focus_subitem);

						if mx < 2.0 || mx > WIDTH - 2.0 || my < 2.0 || my > data.height - 2.0 {
							data.focus_item = usize::MAX;
						} else {
							my -= 2.0;
							for (i, item) in data.items.iter().enumerate() {
								my -= item.height();
								if my < -2.0 {
									match item {
										ContextMenuItem::LabeledIcon(_, _, _) => data.focus_item = i,
										ContextMenuItem::IconList(vec, _, _) => {
											let x = mx as i32 / 20;
											let y = (my + item.height()) as i32 / 20;
											data.focus_subitem = (x.min(3) + 4 * y) as usize;
											data.focus_item = if data.focus_subitem >= vec.len() {
												usize::MAX
											} else { i };
										}
									}
									break;
								} else if my < 0.0 {
									data.focus_item = usize::MAX;
									break;
								}
							}
						}

						if old != (data.focus_item, data.focus_subitem) {
							move_dir = Direction::Mouse;
						}
					}

					Event::MouseUp |
					Event::ButtonUp |
					Event::KeyUp => match control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) {
						Some(MenuIntent::GoBack) => {
							self.close(command_sender);
							return 1;
						}
						Some(MenuIntent::Primary) => {
							let ret = if data.focus_item == usize::MAX {1} else {
								match &data.items[data.focus_item] {
									ContextMenuItem::LabeledIcon(spr, _, _) => -1-*spr,
									ContextMenuItem::IconList(vec, _, _) => -1-vec.get(data.focus_subitem).cloned().unwrap_or(-1)
								}
							};

							if ret < 0 {
								self.close(command_sender);
								return ret;
							}
						}
						_ => ()
					}

					Event::ButtonDown |
					Event::KeyDown => move_dir = match control_settings::MENU_CONTROLS.get_intent(global_state.last_mod_input) {
						Some(MenuIntent::SelectLeft) => Direction::Left,
						Some(MenuIntent::SelectRight) => Direction::Right,
						Some(MenuIntent::SelectUp) => Direction::Up,
						Some(MenuIntent::SelectDown) => Direction::Down,
						_ => Direction::None
					},

					_ => ()
				}

				match move_dir {
					Direction::None => return 1,
					Direction::Mouse => (),
					Direction::Up => if data.focus_item == usize::MAX {
						data.focus_item = data.items.len() - 1;
						if let ContextMenuItem::IconList(_, highlight, _) = &data.items[data.focus_item] {
							data.focus_subitem = *highlight;
						}
					} else if let ContextMenuItem::IconList(_, _, _) = &data.items[data.focus_item] {
						if data.focus_subitem > 3 {
							data.focus_subitem -= 4;
						} else {
							if data.focus_item == 0 {
								data.focus_item = data.items.len() - 1;
							} else {
								data.focus_item -= 1;
							}
	
							if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
								data.focus_subitem = ((vec.len() - 1) & !3) | (data.focus_subitem & 3);
								if data.focus_subitem >= vec.len() {
									data.focus_subitem = vec.len() - 1;
								}
							}
						}
					} else {
						if data.focus_item == 0 {
							data.focus_item = data.items.len() - 1;
						} else {
							data.focus_item -= 1;
						}

						if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
							data.focus_subitem = ((vec.len() - 1) & !3) | (data.focus_subitem & 3);
							if data.focus_subitem >= vec.len() {
								data.focus_subitem = vec.len() - 1;
							}
						}
					}
					Direction::Down => if data.focus_item == usize::MAX {
						data.focus_item = 0;
						if let ContextMenuItem::IconList(_, highlight, _) = &data.items[0] {
							data.focus_subitem = *highlight;
						}
					} else if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
						if data.focus_subitem < vec.len() - 4 {
							data.focus_subitem += 4;
						} else {
							data.focus_item = (data.focus_item + 1) % data.items.len();
	
							if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
								if data.focus_subitem >= vec.len() {
									data.focus_subitem = 0;
								}
							}
						} 
					} else {
						data.focus_item = (data.focus_item + 1) % data.items.len();

						if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
							data.focus_subitem %= 4;
							if data.focus_subitem >= vec.len() {
								data.focus_subitem = vec.len() - 1;
							}
						}
					}
					Direction::Left => if data.focus_item == usize::MAX {
						data.focus_item = 0;
						if let ContextMenuItem::IconList(_, highlight, _) = &data.items[0] {
							data.focus_subitem = *highlight;
						}
					} else if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
						if data.focus_subitem == 0 {
							data.focus_subitem = vec.len() - 1;
						} else {
							data.focus_subitem -= 1;
						}
					}
					Direction::Right => if data.focus_item == usize::MAX {
						data.focus_item = 0;
						if let ContextMenuItem::IconList(_, highlight, _) = &data.items[0] {
							data.focus_subitem = *highlight;
						}
					} else if let ContextMenuItem::IconList(vec, _, _) = &data.items[data.focus_item] {
						data.focus_subitem = (data.focus_subitem + 1) % vec.len();
					}
				}

				for (i, (item, objects)) in data.items.iter().zip(data.item_objects.iter_mut()).enumerate() {
					match item {
						ContextMenuItem::LabeledIcon(_, _, _) => objects[0].set_alpha(command_sender, if i == data.focus_item { 0.75 } else { 0.0 }),
						ContextMenuItem::IconList(vec, highlight, _) => {
							for j in 0..vec.len() {
								objects[j * 2].set_alpha(command_sender, if i == data.focus_item && j == data.focus_subitem { 0.75 }
									else if j == *highlight { 0.25 } else { 0.0 });
							}
						}
					}
				}

				1
			}
		}
	}

	fn close(&mut self, command_sender: &mut CommandSender) {
		match self.state {
			ContextMenuState::Closing(_) |
			ContextMenuState::Closed => (),
			ContextMenuState::Opening(x) => self.state = ContextMenuState::Closing(x),
			ContextMenuState::Open => {
				let data = self.data.as_mut().unwrap();

				for outer in &mut data.item_objects {
					for inner in outer {
						inner.destroy(command_sender);
					}
				}

				self.state = ContextMenuState::Closing(OPEN_CLOSE_FRAMES);
			}
		}
	}
}