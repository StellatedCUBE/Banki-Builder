use std::fmt::Write;

use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, internal_command::InternalCommand, sound, sprite, undo::UndoAction};

use super::{Level, LevelObject, LevelTheme, SubObjectDeleteUndoAction, AABB};
#[cfg(not(feature = "verify"))]
use crate::controller::loc::data::*;
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;

pub const LAYER: u32 = super::L_RAIL_MOVER;

pub struct MovingPlatform {
	points: Vec<(i32, i32)>,
	speed: u8,
	loops: bool,
}

impl LevelObject for MovingPlatform {
	fn sub_object_count(&self) -> usize {
		self.points.len()
	}

	fn save_ids(&self, level: &Level) -> Vec<usize> {
		let mut c = level.connect_up(self.points[0].0, self.points[0].1);
		c.append(&mut level.connect_down(self.points[0].0, self.points[0].1 - 1));
		c
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, _return_object: bool) -> GameObject {
		let mut path = String::with_capacity(if self.loops {
			self.points.len()
		} else {
			self.points.len() * 2 - 2
		} * 14);

		for (x, y) in &self.points {
			let _ = write!(&mut path, "{:07}{:07}", *x * 32 + 524288, *y * 32 + 524288);
		}

		if !self.loops {
			for (x, y) in self.points.iter().skip(1).take(self.points.len() - 2).rev() {
				let _ = write!(&mut path, "{:07}{:07}", *x * 32 + 524288, *y * 32 + 524288);
			}
		}

		let mut go = GameObject::new(game_object::OBJ_MOVINGFLOOR, -30);
		go.x = self.points[0].0 as f32 * 32.0;
		go.y = self.points[0].1 as f32 * 32.0;
		go.create(command_sender);
		go.set_real(command_sender, 2, (self.speed as f32 + 1.0) / 16.0);
		go.set_string(command_sender, 0, &path);
		go
	}

	fn post_create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, self_object: &GameObject, saved_objects: Vec<&GameObject>) {
		let mut used0 = false;
		let mut used2 = 2;

		for object in saved_objects {
			if object.y < self_object.y {
				self_object.set_object(command_sender, used0 as u8, object);
				used0 = true;
			} else {
				self_object.set_object(command_sender, used2, object);
				used2 = 3;
			}
		}
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, _level: &Level) -> Vec<GameObject> {
		self.cev(command_sender)
	}

	fn bounding_box(&self) -> AABB {
		let mut bb = self.sub_object_bounding_box(0);
		for i in 1..self.points.len() {
			bb |= self.sub_object_bounding_box(i);
		}
		bb
	}

	fn sub_object_bounding_box(&self, sub_object: usize) -> AABB {
		AABB {
			x: self.points[sub_object].0 as f32,
			y: self.points[sub_object].1 as f32,
			width: 1.0,
			height: 0.75
		}
	}

	fn delete_sub_object(&mut self, command_sender: &mut CommandSender, objects: &mut Vec<GameObject>, object: usize, sub_object: usize)
	-> SubObjectDeleteUndoAction {
		if self.points.len() < 3 {
			SubObjectDeleteUndoAction::DeleteMain
		} else {
			let (x, y) = self.points.remove(sub_object);
			let did_loop = self.loops;
			if self.points.len() == 2 {
				self.loops = false;
			}
			
			for go in objects.iter_mut() {
				go.destroy(command_sender);
			}
			*objects = self.cev(command_sender);

			SubObjectDeleteUndoAction::Some(UndoAction::MPInsert(object, sub_object, x, y, did_loop && self.points.len() == 2))
		}
	}

	fn move_by(&mut self, x: i32, y: i32) {
		for point in &mut self.points {
			*point = (
				point.0 + x,
				point.1 + y
			);
		}
	}

	fn move_sub_object_by(&mut self, sub_object: usize, x: i32, y: i32) {
		self.points[sub_object] = (
			self.points[sub_object].0 + x,
			self.points[sub_object].1 + y
		);
	}

	fn ghost_sub_object(&self, command_sender: &mut CommandSender, sub_object: usize, objects: &mut Vec<GameObject>) -> ((i32, i32), i32) {
		objects[sub_object].destroy(command_sender);
		if sub_object < objects.len() - 1 || self.loops {
			objects[(sub_object + 1) % objects.len()].set_real(command_sender, 3, 0.0);
		}

		(self.points[sub_object], sprite::MOVINGFLOOR)
	}

	fn types(&self) -> u32 {
		LAYER
	}

	fn sub_object_types(&self, _sub_object: usize) -> u32 {
		LAYER
	}

	fn serialized_type(&self) -> u8 {
		4
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		let mut init = self.speed;
		if self.loops {
			init |= 128;
		}
		to.write(&[init])?;
		for (x, y) in &self.points {
			to.write(&(*x as i16).to_le_bytes())?;
			to.write(&(*y as i16).to_le_bytes())?;
		}
		Ok(())
	}

	#[cfg(not(feature = "verify"))]
	fn sub_object_context_menu_items(&self, _sub_object: usize, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		//vec![ContextMenuItem::LabeledIcon(sprite::SWAP, LOC_SWAP_ENDS.for_current_locale_static(), 0xffffff)]
		let mut items = vec![
			ContextMenuItem::LabeledIcon(sprite::PLUS1, LOC_INSERT_BEFORE.for_current_locale_static(), 0xffffff),
			ContextMenuItem::LabeledIcon(sprite::PLUS2, LOC_INSERT_AFTER.for_current_locale_static(), 0xffffff),
		];

		if self.points.len() == 2 {
			items.push(ContextMenuItem::LabeledIcon(sprite::SWAP, LOC_SWAP_ENDS.for_current_locale_static(), 0xffffff));
		} else {
			items.push(ContextMenuItem::LabeledIcon(sprite::LOOP, if self.loops {
				LOC_UNLOOP_PATH
			} else {
				LOC_LOOP_PATH
			}.for_current_locale_static(), 0xffffff));
			items.push(ContextMenuItem::LabeledIcon(sprite::SWAP, LOC_REVERSE.for_current_locale_static(), 0xffffff));
		}

		items
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		/*sound::play(command_sender, sound::SE_HOLD);
		mem::swap(&mut self.x_start, &mut self.x_end);
		mem::swap(&mut self.y_start, &mut self.y_end);
		vec![UndoAction::ContextMenuAction(object, action)]*/
		
		sound::play(command_sender, sound::SE_HOLD);
		match action {
			sprite::SWAP => if self.loops {
				self.points[1..].reverse();
			} else {
				self.points.reverse();
			}
			sprite::LOOP => self.loops = !self.loops,
			_ => ()
		}
		vec![UndoAction::ContextMenuAction(object, action)]
	}

	fn handle_sub_object_context_menu_action(
		&mut self,
		command_sender: &mut CommandSender,
		object: usize,
		sub_object: usize,
		action: i32,
		theme: LevelTheme
	) -> (bool, Vec<UndoAction>) {
		if action == sprite::PLUS1 || action == sprite::PLUS2 {
			InternalCommand::CreateChainObjectInserter(
				object,
				if sub_object == 0 && action == sprite::PLUS1 && self.loops {
					self.points.len()
				} else {
					sub_object + (action - sprite::PLUS1) as usize
				},
				sprite::MOVINGFLOOR
			).run();
			(false, vec![])
		} else {
			(true, self.handle_context_menu_action(command_sender, object, action, theme))
		}
	}

	fn to_chain(&mut self) -> &mut MovingPlatform {
		self
	}

	fn hax(&self) -> bool {
		self.speed != 15
	}
}

impl MovingPlatform {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			points: vec![
				(x, y),
				(x + 1, y),
			],
			speed: 15,
			loops: false,
		}
	}

	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() < 9 {
			return Self::new(0, 0);
		}

		Self {
			speed: data[0] & 127,
			loops: data[0] > 127,
			points: data[1..].chunks_exact(4).map(|s| (
				i16::from_le_bytes(s[0..2].try_into().unwrap()) as i32,
				i16::from_le_bytes(s[2..4].try_into().unwrap()) as i32
			)).collect()
		}
	}

	pub fn insert(&mut self, index: usize, x: i32, y: i32) {
		self.points.insert(index, (x, y));
	}

	fn cev(&self, command_sender: &mut dyn CommandOutput) -> Vec<GameObject> {
		self.points.iter().cloned().enumerate().map(|(i, (x, y))| {
			let mut go = GameObject::new(game_object::OBJ_MOVINGFLOOR_EDITOR, -10);
			go.x = (x * 32 + 16) as f32;
			go.y = (y * 32 + 16) as f32;
			go.create(command_sender);
			if i == 0 {
				go.set_real(command_sender, 2, 1.0);
				if self.loops {
					go.set_real(command_sender, 0, (self.points.last().unwrap().0 * 32 + 16) as f32);
					go.set_real(command_sender, 1, (self.points.last().unwrap().1 * 32 + 16) as f32);
				} else {
					go.set_real(command_sender, 3, 0.0);
				}
			} else {
				go.set_real(command_sender, 0, (self.points[i - 1].0 * 32 + 16) as f32);
				go.set_real(command_sender, 1, (self.points[i - 1].1 * 32 + 16) as f32);
			}
			if self.loops {
				go.set_sprite(command_sender, sprite::SMALL_ARROW);
			}
			go
		}).collect()
	}
}
