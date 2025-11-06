use core::f32;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::controller::{command_handler::CommandSender, level::{onmyoudama_crawl::{self, Face, OnmyoudamaCrawl}, Character, Level, LevelTheme, L_BLOCK}, sound, sprite::{self, CRAWL_CLOCKWISE}, undo::UndoAction};

use super::{context_menu::ContextMenuItem, tool::Tool, Editor};

#[derive(Default)]
pub struct ToolOnmyoudamaCrawl {
	direction: AtomicBool
}

impl Tool for ToolOnmyoudamaCrawl {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
		let fx = x;
		let fy = y;
        let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if level.any_type_match(x, y, onmyoudama_crawl::LAYER) {
			return vec![];
		}

		let faces = [
			(block_at(&level, x, y + 1), Face::Bottom, 0.5, 1.0),
			(block_at(&level, x - 1, y), Face::Left, 0.0, 0.5),
			(block_at(&level, x, y - 1), Face::Top, 0.5, 0.0),
			(block_at(&level, x + 1, y), Face::Right, 1.0, 0.5)
		];

		let mut face = Face::Bottom;
		let mut dist: f32 = f32::INFINITY;
		let fx = fx - x as f32;
		let fy = fy - y as f32;

		for (valid, f, dx, dy) in faces {
			if valid {
				let d = (dx - fx) * (dx - fx) + (dy - fy) * (dy - fy);
				if d < dist {
					dist = d;
					face = f;
				}
			}
		}

		drop(level);

		sound::play(command_sender, sound::SE_HOLD);
		vec![UndoAction::Delete(editor.add(command_sender, Box::from(OnmyoudamaCrawl {
			x, y,
			face,
			direction: self.direction.load(Ordering::Relaxed)
		})))]
    }

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		CRAWL_CLOCKWISE + self.direction.load(Ordering::Relaxed) as i32
	}

	fn context_menu_items(&self, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![ContextMenuItem::IconList(vec![
			sprite::SWAP,
			sprite::CLOCKWISE
		], !self.direction.load(Ordering::Relaxed) as usize, 1.0)]
	}

	fn handle_context_menu_action(&self, action: i32, _theme: LevelTheme) {
		self.direction.store(action == sprite::SWAP, Ordering::Relaxed);
	}
}

fn block_at(level: &Level, x: i32, y: i32) -> bool {
	level.at(x, y).into_iter().any(|o| level.object_types(o) & onmyoudama_crawl::LAYER == L_BLOCK)
}