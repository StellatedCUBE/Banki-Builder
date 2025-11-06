use std::{sync::Mutex, usize};

use rand::random;

use crate::controller::{command_handler::CommandSender, level::{simple_object::{ObjectType, SimpleObject}, Character, LevelTheme}, sound, undo::UndoAction};

use super::{context_menu::ContextMenuItem, tool::Tool, Editor};

pub struct ToolSimple {
	object: Mutex<ObjectType>
}

impl Tool for ToolSimple {
    fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();
		let mut tool_object = *self.object.lock().unwrap();

		if tool_object.unique() && level.objects.iter().any(|o| o.simple_object_type() == Some(tool_object)) {
			sound::play(command_sender, sound::SE_NOT);
			return vec![];
		}

		if level.any_type_match(x, y, tool_object.types()) {
			return vec![];
		}

		drop(level);

		if tool_object == ObjectType::Ice(false) && random::<f32>() < 0.001 {
			tool_object = ObjectType::Ice(true)
		}

		let object = SimpleObject {
			object_type: tool_object,
			x, y
		};

		sound::play(command_sender, tool_object.place_sound());
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(object)))]
    }

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		if self.object.lock().unwrap().unique() {
			vec![]
		} else {
			self.use_start(command_sender, editor, x, y)
		}
	}

	fn sprite(&self, theme: LevelTheme, _character: Character) -> i32 {
		self.object.lock().unwrap().editor_sprite(theme)
	}

	fn context_menu_items(&self, theme: LevelTheme) -> Vec<ContextMenuItem> {
		let current = *self.object.lock().unwrap();
		let variants = current.variants();
		if variants.len() > 1 {
			vec![ContextMenuItem::IconList(
				variants.iter().map(|o| o.editor_sprite(theme)).collect(),
				variants.iter().position(|o| *o == current).unwrap_or(usize::MAX),
				0.5
			)]
		} else {vec![]}
	}

	fn handle_context_menu_action(&self, action: i32, theme: LevelTheme) {
		let mut object = self.object.lock().unwrap();
		for variant in object.variants() {
			if variant.editor_sprite(theme) == action {
				*object = variant;
				break;
			}
		}
	}
}

impl ToolSimple {
	pub fn new(object: ObjectType) -> Self {
		Self {
			object: Mutex::new(object)
		}
	}
}