use std::{mem, sync::Mutex};

use crate::controller::{command_handler::CommandSender, game_object::{self, GameObject}, level::{self, simple_object::{ObjectType, SimpleObject}, Level}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
struct Inner {
	start: (i32, i32),
	down: bool,
	down_lock: bool,
	px: i32,
	pcount: usize,
	objects: Vec<GameObject>,
}

impl Inner {
	fn go(&mut self, command_sender: &mut CommandSender, level: &mut Level, x: i32, y: f32) {
		let down = if self.down_lock { self.down } else { y < self.start.1 as f32 + 0.5 };

		if down != self.down || x != self.px {
			let types = if down { level::L_IMMUTABLE_BLOCK_UPPER } else { level::L_IMMUTABLE_BLOCK_LOWER };
			let mut lep = self.start.0;
			while lep > x {
				if level.any_type_match(lep - 1, self.start.1, types) {
					break;
				}
				lep -= 1;
			}

			let mut rep = self.start.0 + 1;
			while rep <= x {
				if level.any_type_match(rep, self.start.1, types) {
					break;
				}
				rep += 1;
			}

			let count = (rep - lep) as usize;

			if down != self.down || count != self.pcount {
				sound::play(command_sender, sound::SE_HOLD);
			}

			if count > self.objects.len() {
				for _ in self.objects.len()..count {
					self.objects.push(GameObject::new(game_object::OBJ_BLANK, -10000));
				}
			}

			else if count < self.objects.len() {
				for mut object in self.objects.split_off(count) {
					object.destroy(command_sender);
				}
			}

			let sprite = if down { sprite::SPIKE2 } else { sprite::SPIKE };

			for object in &mut self.objects {
				if object.exists() {
					let nx = (32 * lep) as f32;
					if object.x != nx {
						object.x = nx;
						object.update_position(command_sender);
					}
					if down != self.down {
						object.set_sprite(command_sender, sprite);
					}
				}

				else {
					object.x = (32 * lep) as f32;
					object.y = (self.start.1 * 32) as f32;
					object.create(command_sender);
					object.set_sprite(command_sender, sprite);
					object.set_alpha(command_sender, 0.5);
				}

				lep += 1;
			}

			self.down = down;
			self.px = x;
			self.pcount = count;
		}
	}
}

#[derive(Default)]
pub struct ToolSpike {
	inner: Mutex<Option<Inner>>
}

impl Tool for ToolSpike {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let x = x.floor() as i32;
		let iy = y.floor() as i32;
		let mut level = editor.level.lock().unwrap();
		let at = level.at(x, iy);
		let can_up = !at.iter().any(|o| level.object_types(*o) & level::L_IMMUTABLE_BLOCK_LOWER != 0);
		let can_down = !at.iter().any(|o| level.object_types(*o) & level::L_IMMUTABLE_BLOCK_UPPER != 0);

		if can_up || can_down {
			let mut inner = Inner::default();
			inner.start = (x, iy);
			inner.px = !x;

			if !can_up {
				inner.down = true;
				inner.down_lock = true;
			}

			else if !can_down {
				inner.down_lock = true;
			}

			inner.go(command_sender, &mut level, x, y);
			*self.inner.lock().unwrap() = Some(inner);
		}

		vec![]
	}

	fn use_frame(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) {
		if let Some(inner) = self.inner.lock().unwrap().as_mut() {
			inner.go(command_sender, &mut editor.level.lock().unwrap(), x.floor() as i32, y);
		}
	}

	fn use_end(&self, command_sender: &mut CommandSender, editor: &mut Editor) {
		if let Some(inner) = mem::take(&mut *self.inner.lock().unwrap()) {
			if inner.objects.len() > 1 {
				sound::play(command_sender, sound::SE_CHANDELIER1);
			}

			let object_type = if inner.down { ObjectType::SpikeDown } else { ObjectType::SpikeUp };
			let actions = inner.objects.into_iter().map(|mut object| {
				object.destroy(command_sender);
				UndoAction::Delete(editor.add(command_sender, Box::new(SimpleObject {
					object_type,
					x: object.x as i32 / 32,
					y: object.y as i32 / 32,
				})))
			}).collect();
			editor.add_undo_frame_complete(actions);
		}
	}

	fn sprite(&self, _theme: level::LevelTheme, _character: level::Character) -> i32 {
		sprite::SPIKE
	}

	fn block_context_menu(&self) -> bool {
		self.inner.lock().unwrap().is_some()
	}
}