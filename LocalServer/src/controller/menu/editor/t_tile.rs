use std::{mem, sync::atomic::{AtomicU8, Ordering}};

use crate::controller::{command_handler::CommandSender, level::{tile_manager::{OverwriteDirection, Tile}, Character, LevelTheme, ObjectID}, menu::editor::context_menu::ContextMenuItem, sound, sprite, undo::UndoAction};

use super::tool::Tool;

pub struct ToolTile {
	tile_id: AtomicU8
}

impl Tool for ToolTile {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let x = x.floor() as i32;
		let y = y.floor() as i32;

		let mut level = editor.level.lock().unwrap();
		let prev = level.tile_manager().get(x, y);
		let tile = self.tile();

		for object in level.at(x, y) {
			if let ObjectID::SubObject(0, _) = object {
				if (tile.ground_any() && prev.ground_any()) || (tile.bg() && prev.bg()) || tile.overwrites(prev) != OverwriteDirection::SelfOverwritesOther {
					return vec![];
				}
			}
			
			else if level.object_types(object) & tile.types() > 0 {
				return vec![];
			}
		}
		
		level.tile_manager_mut().set_and_update(command_sender, x, y, tile.roll());
		sound::play(command_sender, sound::SE_HOLD);

		vec![UndoAction::SetTile(x, y, prev)]
	}

	fn use_new_tile(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) -> Vec<UndoAction> {
		self.use_start(command_sender, editor, x, y)
	}

	fn sprite(&self, theme: LevelTheme, _character: Character) -> i32 {
		self.tile().sprite(theme)
	}

	fn context_menu_items(&self, theme: LevelTheme) -> Vec<ContextMenuItem> {
		if self.tile().block() {
			vec![ContextMenuItem::IconList(vec![
				Tile::BlockRed.sprite(theme),
				Tile::BlockBlue.sprite(theme),
				Tile::BlockPurple.sprite(theme),
				Tile::BlockGreen.sprite(theme)
			], self.tile() as usize - Tile::BlockRed as usize, 0.5)]
		} else {
			vec![]
		}
	}

	fn handle_context_menu_action(&self, action: i32, _theme: LevelTheme) {
		self.tile_id.store((action - (sprite::BLOCK_RED - Tile::BlockRed as i32)) as u8, Ordering::Relaxed);
	}
}

impl ToolTile {
	pub fn new(tile: Tile) -> Self {
		Self {
			tile_id: AtomicU8::new(tile as u8)
		}
	}

	fn tile(&self) -> Tile {
		unsafe {
			mem::transmute(self.tile_id.load(Ordering::Relaxed))
		}
	}
}