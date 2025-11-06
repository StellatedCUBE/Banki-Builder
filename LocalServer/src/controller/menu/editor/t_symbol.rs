use std::{cmp::Ordering::*, sync::atomic::{AtomicBool, AtomicI16, AtomicU32, Ordering}};

use crate::controller::{command_handler::CommandSender, level::{symbol::{self, Symbol, SymbolObject}, Character, LevelTheme, ObjectID, L_SYMBOL}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Default)]
pub struct ToolSymbol {
	object: AtomicU32,
	x: AtomicI16,
	y: AtomicI16,
	undirected: AtomicBool,
}

impl Tool for ToolSymbol {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(x, y, L_SYMBOL) {
			return vec![];
		}

		drop(level);

		if let ObjectID::Object(id) = editor.add(command_sender, Box::new(SymbolObject::new(x, y))) {
			self.object.store(id as u32, Ordering::Relaxed);
			self.x.store(x as i16, Ordering::Relaxed);
			self.y.store(y as i16, Ordering::Relaxed);
			self.undirected.store(true, Ordering::Relaxed);
			vec![UndoAction::Delete(ObjectID::Object(id))]
		}

		else {
			unreachable!()
		}
	}

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let object = self.object.load(Ordering::Relaxed) as usize;

		if object != 0 {
			let x = x.floor() as i16;
			let y = y.floor() as i16;

			let new = match (x.cmp(&self.x.load(Ordering::Relaxed)), y.cmp(&self.y.load(Ordering::Relaxed))) {
				(Less, Less) => Symbol::ArrowUL,
				(Equal, Less) => Symbol::ArrowU,
				(Greater, Less) => Symbol::ArrowUR,
				(Less, Equal) => Symbol::ArrowL,
				(Equal, Equal) => Symbol::Circle,
				(Greater, Equal) => Symbol::ArrowR,
				(Less, Greater) => Symbol::ArrowDL,
				(Equal, Greater) => Symbol::ArrowD,
				(Greater, Greater) => Symbol::ArrowDR
			};

			let mut level = editor.level.lock().unwrap();
			if level.objects[object].handle_context_menu_action(
				command_sender, object, new as i32 + sprite::SYM_ICON_CIRCLE, LevelTheme::DreamFields).len() > 0 {
				let go = &mut editor.objects[object][0];
				go.set_rotation(command_sender, 0.0);
				go.set_scale(command_sender, 1.0, 1.0);
				symbol::transform(command_sender, go, new);
				self.undirected.store(false, Ordering::Relaxed);
			}
		}

		vec![]
	}

	fn use_end(&self, command_sender: &mut CommandSender, _editor: &mut Editor) {
		self.object.store(0, Ordering::Relaxed);
		if self.undirected.load(Ordering::Relaxed) {
			sound::play(command_sender, sound::SE_HOLD);
			self.undirected.store(false, Ordering::Relaxed);
		}
	}

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		sprite::TOOL_SYMBOL
	}
}