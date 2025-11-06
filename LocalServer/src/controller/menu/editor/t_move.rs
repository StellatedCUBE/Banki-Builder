use std::{collections::HashSet, mem, sync::{atomic::Ordering, Mutex}};

use crate::controller::{command_handler::CommandSender, game_object::{self, GameObject}, level::{flipper, tile_manager::{OverwriteDirection, Tile, TileManager}, ObjectID}, undo::UndoAction};

use super::tool::Tool;

#[derive(Default)]
pub struct ToolMove {
    data: Mutex<MoveData>
}

#[derive(Default)]
struct MoveData {
    active: bool,
    from_selection: bool,
    start_x: f32,
    start_y: f32,
    offset_x: i32,
    offset_y: i32,
    objects: Vec<usize>,
    sub_objects: Vec<(usize, usize)>,
    tiles: Vec<(i32, i32, Tile)>,
    ghosts: Vec<GameObject>,
    undo_actions: Vec<UndoAction>,
}

impl Tool for ToolMove {
    fn use_start(&self, command_sender: &mut CommandSender, editor: &mut super::Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let mut data = self.data.lock().unwrap();

        if data.active {
            return vec![];
        }

        data.from_selection = editor.selection.len() != 0;

        let oids: Vec<ObjectID> = if data.from_selection {
            let oids = editor.selection.keys().cloned().collect();
            editor.set_selected(command_sender, vec![]);
            oids
        }

        else {
            editor.top_at(x, y).into_iter().collect()
        };

        if oids.len() != 0 {
            data.active = true;
            data.start_x = x;
            data.start_y = y;

            let mut level = editor.level.lock().unwrap();

            for oid in oids {
                match oid {
                    ObjectID::Object(id) => {
                        data.objects.push(id);

                        for go in &editor.objects[id] {
                            if go.object_type == game_object::OBJ_NO_ANIM && go.sprite >= 0 {
                                let mut ghost = GameObject::new(game_object::OBJ_CURSOR, -8000);
                                ghost.create(command_sender);
                                ghost.set_alpha(command_sender, 0.8);
                                ghost.set_colour(command_sender, go.colour);
                                ghost.set_rotation(command_sender, go.rotation);
                                ghost.set_real(command_sender, 0, go.x - x * 32.0);
                                ghost.set_real(command_sender, 1, go.y - y * 32.0);
                                ghost.set_sprite(command_sender, go.sprite);
                                data.ghosts.push(ghost);
                            }
                        }

                        level.objects[id].destroy_editor_view(command_sender, &mut editor.objects[id], &level);
                    }

                    ObjectID::SubObject(0, sid) => {
                        let tm = level.tile_manager_mut();
                        let (tx, ty) = TileManager::from_sub_id(sid);
                        let tile = tm.get(tx, ty);
                        data.tiles.push((tx, ty, tile));
                        data.undo_actions.push(UndoAction::SetTile(tx, ty, tile));
                        tm.set_and_update(command_sender, tx, ty, Tile::None);

                        let mut ghost = GameObject::new(game_object::OBJ_CURSOR, -8000);
                        ghost.create(command_sender);
                        ghost.set_alpha(command_sender, 0.8);
                        ghost.set_real(command_sender, 0, (tx * 32) as f32 - x * 32.0);
                        ghost.set_real(command_sender, 1, (ty * 32) as f32 - y * 32.0);
                        ghost.set_sprite(command_sender, tile.sprite(level.theme));
                        data.ghosts.push(ghost);
                    }

                    ObjectID::SubObject(id, sid) => {
                        data.sub_objects.push((id, sid));

                        let ((gx, gy), sprite) = level.objects[id].ghost_sub_object(command_sender, sid, &mut editor.objects[id]);
                        let mut ghost = GameObject::new(game_object::OBJ_CURSOR, -8000);
                        ghost.create(command_sender);
                        ghost.set_alpha(command_sender, 0.8);
                        ghost.set_real(command_sender, 0, (gx * 32) as f32 - x * 32.0);
                        ghost.set_real(command_sender, 1, (gy * 32) as f32 - y * 32.0);
                        ghost.set_sprite(command_sender, sprite);
                        data.ghosts.push(ghost);
                    }
                }
            }
        }

        vec![]
    }

    fn use_frame(&self, _command_sender: &mut CommandSender, _editor: &mut super::Editor, x: f32, y: f32) {
        let mut data = self.data.lock().unwrap();

        if data.active {
            data.offset_x = (x - data.start_x).round() as i32;
            data.offset_y = (y - data.start_y).round() as i32;
        }
    }

    fn use_end(&self, command_sender: &mut CommandSender, editor: &mut super::Editor) {
        let mut data_mtx = self.data.lock().unwrap();

        if data_mtx.active {
            let mut data = MoveData::default();
            mem::swap(&mut data, &mut data_mtx);

            for mut ghost in data.ghosts {
                ghost.destroy(command_sender);
            }

            let mut level = editor.level.lock().unwrap();

            let mut valid = true;

            flipper::SHIFT_HITBOX.store(true, Ordering::Relaxed);

            for id in &data.objects {
                let id = *id;
                let bb = level.objects[id].bounding_box().expand_to_tile();
                for x in bb.x as i32..(bb.x + bb.width) as i32 {
                    for y in bb.y as i32..(bb.y + bb.height) as i32 {
                        for obj in level.at(x + data.offset_x, y + data.offset_y) {
                            match obj {
                                ObjectID::Object(oid) => {
                                    if level.objects[id].types() & level.objects[oid].types() > 0 && !data.objects.contains(&oid) {
                                        valid = false;
                                        break;
                                    }
                                }

                                ObjectID::SubObject(oid, sid) => {
                                    if level.objects[id].types() & level.objects[oid].sub_object_types(sid) > 0 &&
                                        !data.sub_objects.contains(&(oid, sid))
                                    {
                                        valid = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if !valid {break}
            }

            for (id, sid) in data.sub_objects.iter().cloned() {
                if !valid {break}

                let bb = level.objects[id].sub_object_bounding_box(sid);
                let x = bb.x.floor() as i32;
                let y = bb.y.floor() as i32;

                for obj in level.at(x + data.offset_x, y + data.offset_y) {
                    match obj {
                        ObjectID::Object(oid) => {
                            if level.objects[id].sub_object_types(sid) & level.objects[oid].types() > 0 && !data.objects.contains(&oid) {
                                valid = false;
                                break;
                            }
                        }

                        ObjectID::SubObject(oid, sid2) => {
                            if level.objects[id].sub_object_types(sid) & level.objects[oid].sub_object_types(sid2) > 0 &&
                                !data.sub_objects.contains(&(oid, sid2))
                            {
                                valid = false;
                                break;
                            }
                        }
                    }
                }
            }

            for (x, y, tile) in &data.tiles {
                if !valid {break}

                let nx = x + data.offset_x;
                let ny = y + data.offset_y;

                for obj in level.at(nx, ny) {
                    if let ObjectID::Object(id) = obj {
                        if level.objects[id].types() & tile.types() > 0 && !data.objects.contains(&id) {
                            valid = false;
                            break;
                        }
                    }

                    else if level.tile_manager().get(nx, ny).overwrites(*tile) == OverwriteDirection::None {
                        valid = false;
                        break;
                    }
                }
            }

            flipper::SHIFT_HITBOX.store(false, Ordering::Relaxed);

            if !valid {
                data.offset_x = 0;
                data.offset_y = 0;
            }

            for id in &data.objects {
                let id = *id;
                level.objects[id].move_by(data.offset_x, data.offset_y);
                editor.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
                data.undo_actions.push(UndoAction::MoveObject(ObjectID::Object(id), -data.offset_x, -data.offset_y));
            }

            let mut partially_moved_objects = HashSet::new();
            for (id, sid) in data.sub_objects.iter().cloned() {
                level.objects[id].destroy_editor_view(command_sender, &mut editor.objects[id], &level);
                level.objects[id].move_sub_object_by(sid, data.offset_x, data.offset_y);
                partially_moved_objects.insert(id);
                data.undo_actions.push(UndoAction::MoveObject(ObjectID::SubObject(id, sid), -data.offset_x, -data.offset_y));
            }

            for id in partially_moved_objects {
                editor.objects[id] = level.objects[id].create_editor_view(command_sender, &level);
            }

            let tm = level.tile_manager_mut();

            for (x, y, tile) in &data.tiles {
                let nx = x + data.offset_x;
                let ny = y + data.offset_y;
                let old = tm.get(nx, ny);
                if tile.overwrites(old) == OverwriteDirection::SelfOverwritesOther {
                    tm.set_and_update(command_sender, nx, ny, *tile);
                    data.undo_actions.push(UndoAction::SetTile(nx, ny, old));
                }
            }

            drop(level);

            if data.from_selection {
                let mut selection = Vec::with_capacity(data.objects.len() + data.tiles.len());

                for id in &data.objects {
                    selection.push(ObjectID::Object(*id));
                }

                for (id, sid) in &data.sub_objects {
                    selection.push(ObjectID::SubObject(*id, *sid));
                }

                for (x, y, _) in data.tiles {
                    selection.push(ObjectID::SubObject(0, TileManager::to_sub_id(x + data.offset_x, y + data.offset_y)));
                }

                editor.set_selected(command_sender, selection);
            }

            if data.offset_x != 0 || data.offset_y != 0 {
                editor.add_undo_frame_complete(data.undo_actions);
            }
        }
    }

    fn clear_selection(&self) -> bool {false}

    fn block_context_menu(&self) -> bool {
        self.data.lock().unwrap().active
    }

    fn can_be_used_zoomed_out(&self) -> bool {true}
}