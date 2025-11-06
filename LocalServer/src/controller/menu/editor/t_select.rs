use crate::controller::{command_handler::CommandSender, game_object::{self, GameObject}, level::{tile_manager::{Tile, TileManager}, ObjectID, AABB}, sound, undo::UndoAction};

use super::tool::Tool;

#[derive(Default)]
pub struct ToolSelect {}

impl Tool for ToolSelect {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) -> Vec<UndoAction> {
		editor.selection_start = (x, y);

		let mut sbox = GameObject::new(game_object::OBJ_YELLOW_BOX, -14990);
		sbox.create(command_sender);
		sbox.set_real(command_sender, 0, x * 32.0);
		sbox.set_real(command_sender, 1, y * 32.0);
		editor.selection_box = Some(sbox);

		self.use_frame(command_sender, editor, x, y);
		vec![]
	}
	
	fn use_frame(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) {
		if let Some(sbox) = editor.selection_box.as_mut() {
			sbox.set_real(command_sender, 2, x * 32.0);
			sbox.set_real(command_sender, 3, y * 32.0);
		}

		let nx = editor.selection_start.0.min(x);
		let ny = editor.selection_start.1.min(y);

		let bounds = AABB {
			x: nx, y: ny,
			width: editor.selection_start.0.max(x) - nx,
			height: editor.selection_start.1.max(y) - ny,
		};

		let level = editor.level.lock().unwrap();
		let mut selection = vec![];

		for i in 1..level.objects.len() {
			if level.objects[i].bounding_box().intersects(bounds) {
				if level.objects[i].sub_object_count() == 0 {
					selection.push(ObjectID::Object(i));
				} else {
					for j in 0..level.objects[i].sub_object_count() {
						if level.objects[i].sub_object_bounding_box(j).intersects(bounds) {
							selection.push(ObjectID::SubObject(i, j));
						}
					}
				}
			}
		}

		let tm = level.tile_manager();

		for ix in editor.selection_start.0.min(x).floor() as i32..editor.selection_start.0.max(x).ceil() as i32 {
			for iy in editor.selection_start.1.min(y).floor() as i32..editor.selection_start.1.max(y).ceil() as i32 {
				if tm.get(ix, iy) != Tile::None {
					selection.push(ObjectID::SubObject(0, TileManager::to_sub_id(ix, iy)));
				}
			}
		}

		drop(level);

		if selection.len() != editor.selection.len() {
			sound::play(command_sender, sound::SE_MESSAGE);
		}

		editor.set_selected(command_sender, selection);
	}

	fn use_end(&self, command_sender: &mut CommandSender, editor: &mut super::Editor) {
		if let Some(sbox) = editor.selection_box.as_mut() {
			sbox.destroy(command_sender);
			editor.selection_box = None;
		}
	}

	fn can_be_used_zoomed_out(&self) -> bool {true}
}