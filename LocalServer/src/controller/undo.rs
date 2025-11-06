use crate::controller::{command_handler::CommandSender, level::{simple_object::ObjectType, tile_manager::Tile, LevelObject, LevelTheme, ObjectID}, sound, sprite};

#[cfg(not(feature = "verify"))]
use super::menu::editor::Editor;

pub struct UndoFrame {
	pub actions: Vec<UndoAction>
}

pub enum UndoAction {
	Nop,
	Delete(ObjectID),
	SetTile(i32, i32, Tile),
	AddObject(usize, Box<dyn LevelObject + Send>),
	MoveObject(ObjectID, i32, i32),
	SetSimpleObjectType(usize, ObjectType),
	ContextMenuAction(usize, i32),
	SeijaPowerToggle(i32),
	MPInsert(usize, usize, i32, i32, bool),
}

#[cfg(not(feature = "verify"))]
impl UndoAction {
	pub fn perform(self, command_sender: &mut CommandSender, editor: &mut Editor) -> UndoAction {
		sound::disable();
		let revert = match self {
			Self::Nop => Self::Nop,
			Self::Delete(obj) => editor.delete(command_sender, obj).unwrap_or(Self::Nop),
			Self::SetTile(x, y, tile) => {
				let mut level = editor.level.lock().unwrap();
				let tm = level.tile_manager_mut();
				let old = tm.get(x, y);
				tm.set_and_update(command_sender, x, y, tile);
				Self::SetTile(x, y, old)
			}
			Self::AddObject(id, obj) => {
				editor.add(command_sender, obj);
				let obj = editor.objects.pop().unwrap();
				editor.objects.insert(id, obj);
				let mut level = editor.level.lock().unwrap();
				let obj = level.objects.pop().unwrap();
				level.objects.insert(id, obj);
				let key = level.object_keys.pop().unwrap();
				level.object_keys.insert(id, key);
				Self::Delete(ObjectID::Object(id))
			}
			Self::MoveObject(oid, x, y) => {
				let mut level = editor.level.lock().unwrap();
				level.objects[oid.id()].destroy_editor_view(command_sender, &mut editor.objects[oid.id()], &level);
				match oid {
					ObjectID::Object(id) => level.objects[id].move_by(x, y),
					ObjectID::SubObject(id, sid) => level.objects[id].move_sub_object_by(sid, x, y),
				}
				editor.objects[oid.id()] = level.objects[oid.id()].create_editor_view(command_sender, &level);
				Self::MoveObject(oid, -x, -y)
			}
			Self::SetSimpleObjectType(id, object_type) => {
				let mut level = editor.level.lock().unwrap();
				level.objects[id].destroy_editor_view(command_sender, &mut editor.objects[id], &level);
				let old = level.objects[id].to_simple_object().object_type;
				level.objects[id].to_simple_object_mut().object_type = object_type;
				editor.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
				Self::SetSimpleObjectType(id, old)
			}
			Self::ContextMenuAction(id, action) => {
				let mut level = editor.level.lock().unwrap();
				level.objects[id].destroy_editor_view(command_sender, &mut editor.objects[id], &level);
				let revert = level.objects[id].handle_context_menu_action(command_sender, id, action, LevelTheme::DreamFields)
					.into_iter().next().unwrap();
				editor.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
				revert
			}
			Self::SeijaPowerToggle(button) => {
				if let Some((i, bref)) = editor.object_buttons.iter()
					.enumerate()
					.filter(|p| p.1.game_object.sprite == button)
					.next() {
					(bref.callback)(command_sender, editor, i);
				}
				Self::SeijaPowerToggle(button)
			}
			Self::MPInsert(id, index, x, y, make_loop) => {
				let mut level = editor.level.lock().unwrap();
				level.objects[id].destroy_editor_view(command_sender, &mut editor.objects[id], &level);
				level.objects[id].to_chain().insert(index, x, y);
				if make_loop {
					level.objects[id].handle_context_menu_action(command_sender, id, sprite::LOOP, LevelTheme::DreamFields);
				}
				editor.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
				Self::Delete(ObjectID::SubObject(id, index))
			}
		};
		sound::enable();
		revert
	}
}