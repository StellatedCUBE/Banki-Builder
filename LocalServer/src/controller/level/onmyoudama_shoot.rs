use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, internal_command::InternalCommand, level::LevelTheme, sound, sprite, undo::UndoAction};

use super::{Level, LevelObject, AABB, ObjectButton};

pub const LAYER: u32 = super::L_OBJECT;

const UP: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 4;
const RIGHT: u8 = 8;

pub struct OnmyoudamaShoot {
	x: i32, y: i32,
	dirs: u8
}

impl LevelObject for OnmyoudamaShoot {
	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject> {
		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 0);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, if self.dirs & (LEFT | RIGHT) == 0 { sprite::ONMYOUDAMA_BLUE } else { sprite::ONMYOUDAMA_RED });

		let object = level.object_key(self);

		{
			let x = (self.x * 32 + 13) as f32;
			let y = (self.y * 32 + 3) as f32;

			let mut game_object = GameObject::new(game_object::OBJ_BLANK, -1000);
			game_object.x = x;
			game_object.y = y;
			game_object.create(command_sender);
			game_object.set_sprite(command_sender, sprite::BUTTON_UP + (self.dirs & UP) as i32);

			InternalCommand::CreateObjectButton(ObjectButton {
				object,
				game_object,
				bounds: AABB {
					x, y,
					width: 6.0,
					height: 8.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					let go = &mut editor.object_buttons[button].game_object;
					go.set_sprite(command_sender, go.sprite ^ 1);
					let mut level = editor.level.lock().unwrap();
					let index = level.index_by_key(editor.object_buttons[button].object);
					level.objects[index].handle_context_menu_action(command_sender, index, UP as i32, LevelTheme::DreamFields)
				}
			}).run();
		}

		{
			let x = (self.x * 32 + 13) as f32;
			let y = (self.y * 32 + 21) as f32;

			let mut game_object = GameObject::new(game_object::OBJ_BLANK, -1000);
			game_object.x = x;
			game_object.y = y;
			game_object.create(command_sender);
			game_object.set_sprite(command_sender, sprite::BUTTON_DOWN + ((self.dirs & DOWN) / DOWN) as i32);

			InternalCommand::CreateObjectButton(ObjectButton {
				object,
				game_object,
				bounds: AABB {
					x, y,
					width: 6.0,
					height: 8.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					let go = &mut editor.object_buttons[button].game_object;
					go.set_sprite(command_sender, go.sprite ^ 1);
					let mut level = editor.level.lock().unwrap();
					let index = level.index_by_key(editor.object_buttons[button].object);
					level.objects[index].handle_context_menu_action(command_sender, index, DOWN as i32, LevelTheme::DreamFields)
				}
			}).run();
		}

		{
			let x = (self.x * 32 + 3) as f32;
			let y = (self.y * 32 + 13) as f32;

			let mut game_object = GameObject::new(game_object::OBJ_BLANK, -1000);
			game_object.x = x;
			game_object.y = y;
			game_object.create(command_sender);
			game_object.set_sprite(command_sender, sprite::BUTTON_LEFT + ((self.dirs & LEFT) / LEFT) as i32);

			InternalCommand::CreateObjectButton(ObjectButton {
				object,
				game_object,
				bounds: AABB {
					x, y,
					width: 8.0,
					height: 6.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					let button = &mut editor.object_buttons[button];
					button.game_object.set_sprite(command_sender, button.game_object.sprite ^ 1);
					let mut level = editor.level.lock().unwrap();
					let index = level.index_by_key(button.object);
					let ua = level.objects[index]
						.handle_context_menu_action(command_sender, index, LEFT as i32, LevelTheme::DreamFields);
					if let UndoAction::ContextMenuAction(_, old) = &ua[0] {
						editor.objects[index][0].set_sprite(
							command_sender,
							if ((!*old) as u8 ^ LEFT) & (LEFT | RIGHT) == 0 { sprite::ONMYOUDAMA_BLUE } else { sprite::ONMYOUDAMA_RED });
					}
					ua
				}
			}).run();
		}

		{
			let x = (self.x * 32 + 21) as f32;
			let y = (self.y * 32 + 13) as f32;

			let mut game_object = GameObject::new(game_object::OBJ_BLANK, -1000);
			game_object.x = x;
			game_object.y = y;
			game_object.create(command_sender);
			game_object.set_sprite(command_sender, sprite::BUTTON_RIGHT + ((self.dirs & RIGHT) / RIGHT) as i32);

			InternalCommand::CreateObjectButton(ObjectButton {
				object,
				game_object,
				bounds: AABB {
					x, y,
					width: 8.0,
					height: 6.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					let button = &mut editor.object_buttons[button];
					button.game_object.set_sprite(command_sender, button.game_object.sprite ^ 1);
					let mut level = editor.level.lock().unwrap();
					let index = level.index_by_key(button.object);
					let ua = level.objects[index]
						.handle_context_menu_action(command_sender, index, RIGHT as i32, LevelTheme::DreamFields);
					if let UndoAction::ContextMenuAction(_, old) = &ua[0] {
						editor.objects[index][0].set_sprite(
							command_sender,
							if ((!*old) as u8 ^ RIGHT) & (LEFT | RIGHT) == 0 { sprite::ONMYOUDAMA_BLUE } else { sprite::ONMYOUDAMA_RED });
					}
					ua
				}
			}).run();
		}

		vec![go]
	}

	#[cfg(not(feature = "verify"))]
	fn destroy_editor_view(&self, command_sender: &mut dyn CommandOutput, objects: &mut Vec<GameObject>, level: &Level) {
		objects[0].destroy(command_sender);
		InternalCommand::ClearObjectButtons(level.object_key(self)).run();
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, _return_object: bool) -> GameObject {
		let mut go = GameObject::new(
			if self.dirs == 0 { game_object::OBJ_ONMYOUDAMA+0 }
			else if self.dirs & (LEFT | RIGHT) == 0 { game_object::OBJ_ONMYOUDAMA+3 }
			else { game_object::OBJ_ONMYOUDAMA+4 },
			0
		);
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);

		if self.dirs != 0 && self.dirs != LEFT | RIGHT && self.dirs != UP | DOWN {
			go.set_real(command_sender, 0, (self.dirs & UP) as f32);
			go.set_real(command_sender, 1, (self.dirs & DOWN) as f32);
			go.set_real(command_sender, 2, (self.dirs & LEFT) as f32);
			go.set_real(command_sender, 3, (self.dirs & RIGHT) as f32);
		}

		go.destroy_server_only();

		go
	}

	fn bounding_box(&self) -> AABB {
		AABB {
			x: self.x as f32 + 0.25,
			y: self.y as f32 + 0.25,
			width: 0.5,
			height: 0.5
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
		9
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		to.write(&[self.dirs])?;
		Ok(())
	}

	fn handle_context_menu_action(&mut self, _command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		let old = self.dirs;
		if action < 0 {
			self.dirs = (!action) as u8;
		} else {
			self.dirs ^= action as u8;
		}
		vec![UndoAction::ContextMenuAction(object, !(old as i32))]
	}

}

impl OnmyoudamaShoot {
	pub const fn new(x: i32, y: i32) -> Self {
		Self { x, y, dirs: 0 }
	}

	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 4 {
			Self {
				x: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32,
				dirs: data[4],
			}
		} else {
			Self::new(0, 0)
		}
	}
}