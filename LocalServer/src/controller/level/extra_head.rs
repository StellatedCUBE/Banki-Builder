use std::sync::Mutex;

use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, internal_command::InternalCommand, level::{Character, LevelTheme, ObjectButton}, sound, sprite, undo::UndoAction};

use super::{simple_object::ObjectType, Level, LevelObject, AABB};

pub const LAYER: u32 = ObjectType::PuzzlePiece.types();

pub struct ExtraHead {
	pub x: i32,
	pub y: i32,
	amount: u8,
	text: Mutex<GameObject>,
}

impl LevelObject for ExtraHead {
	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject> {
		let self_key = level.object_key(self);

		let mut go = GameObject::new(game_object::OBJ_BLANK, -1000);
		go.x = (self.x * 32 + 4) as f32;
		go.y = (self.y * 32 + 4) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, sprite::BUTTON_MINUS);

		InternalCommand::CreateObjectButton(ObjectButton {
			game_object: go,
			object: self_key,
			bounds: AABB {
				x: (self.x * 32 + 4) as f32,
				y: (self.y * 32 + 4) as f32,
				width: 7.0,
				height: 7.0
			},
			callback: |command_sender, editor, button| {
				let key = editor.object_buttons[button].object;
				let mut level = editor.level.lock().unwrap();
				let index = level.index_by_key(key);
				level.objects[index].handle_context_menu_action(command_sender, index, 101, LevelTheme::DreamFields)
			}
		}).run();

		let mut go = GameObject::new(game_object::OBJ_BLANK, -1000);
		go.x = (self.x * 32 + 13) as f32;
		go.y = (self.y * 32 + 4) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, sprite::BUTTON_PLUS);

		InternalCommand::CreateObjectButton(ObjectButton {
			game_object: go,
			object: self_key,
			bounds: AABB {
				x: (self.x * 32 + 13) as f32,
				y: (self.y * 32 + 4) as f32,
				width: 7.0,
				height: 7.0
			},
			callback: |command_sender, editor, button| {
				let key = editor.object_buttons[button].object;
				let mut level = editor.level.lock().unwrap();
				let index = level.index_by_key(key);
				level.objects[index].handle_context_menu_action(command_sender, index, 102, LevelTheme::DreamFields)
			}
		}).run();

		let mut go = GameObject::new(game_object::OBJ_TEXT, -1000);
		go.x = (self.x * 32 + 28) as f32;
		go.y = (self.y * 32 + 16) as f32;
		go.create(command_sender);
		if self.amount > 1 {
			go.set_string(command_sender, 0, &self.amount.to_string());
		}
		go.set_real(command_sender, 1, 2.0);
		*self.text.lock().unwrap() = go;

		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 0);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, match level.character() {
			Character::Banki => sprite::HEAD,
			c => sprite::CIRNO_HEAD - 1 + c as i32
		});
		
		vec![go]
	}

	#[cfg(not(feature = "verify"))]
	fn destroy_editor_view(&self, command_sender: &mut dyn CommandOutput, objects: &mut Vec<GameObject>, level: &Level) {
		objects[0].destroy(command_sender);
		self.text.lock().unwrap().destroy(command_sender);
		InternalCommand::ClearObjectButtons(level.object_key(self)).run();
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, return_object: bool) -> GameObject {
		let mut go = GameObject::new(game_object::OBJ_HEADPLUS, 2);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);

		if self.amount > 1 {
			go.set_real(command_sender, 0, self.amount as f32);

			let mut go2 = GameObject::new(game_object::OBJ_TEXT, 1);
			go2.x = (self.x * 32 + 28) as f32;
			go2.y = (self.y * 32 + 16) as f32;
			go2.create(command_sender);
			go2.set_string(command_sender, 0, &self.amount.to_string());
			go2.set_real(command_sender, 1, 2.0);

			go.set_object(command_sender, 0, &go2);
			go2.destroy_server_only();
		}

		if !return_object {
			go.destroy_server_only();
		}

		go
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32 + 0.09375,
			y: self.y as f32 + 0.25,
			width: 0.75,
			height: 0.46875
		}
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn types(&self) -> u32 {
		LAYER
	}

	fn serialized_type(&self) -> u8 {
		8
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		to.write(&[self.amount])?;
		Ok(())
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		let old = self.amount;
		let new = match action {
			101 => old - 1,
			102 => old + 1,
			x => x as u8
		};

		if new < 1 || new > 99 {
			sound::play(command_sender, sound::SE_NOT);
			return vec![];
		}

		self.amount = new;

		if action > 100 {
			sound::play(command_sender, sound::SE_SELECT);
			let string;
			self.text.get_mut().unwrap().set_string(command_sender, 0, if new > 1 { string = new.to_string(); &string } else { "" });
		}

		vec![UndoAction::ContextMenuAction(object, old as i32)]
	}

	fn recreate_on_character_change(&self) -> bool {true}
}

impl ExtraHead {
	pub const fn new(x: i32, y: i32) -> Self {
		Self { x, y, amount: 1, text: Mutex::new(GameObject::null()) }
	}

	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 4 {
			Self {
				x: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32,
				amount: data[4],
				text: Mutex::new(GameObject::null()),
			}
		} else {
			Self::new(0, 0)
		}
	}
}