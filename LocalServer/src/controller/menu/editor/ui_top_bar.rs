use std::sync::Arc;

use crate::controller::{command_handler::CommandSender, game_object::{self, GameObject}, level::{Character, LevelTheme}, menu::shape::Shape, sprite};

use super::{tool::{ArcTool, TOOLS}, tool_icon::ToolIcon, ZOOM_FACTOR};

pub struct UITopBar {
	enabled: bool,
	zoom_enabled: bool,
	bar_go: GameObject,
	small_tools: [GameObject; 4],
	zoom_tools: [GameObject; 4],
	pub dropdown_go: GameObject,
	pub tool_bar: [ToolIcon; 10],
	pub tool_bar_index: usize,
}

impl UITopBar {
	pub fn new() -> Self {
		Self {
			enabled: true,
			zoom_enabled: false,
			bar_go: GameObject::new(game_object::OBJ_UI, -12000),
			small_tools: [
				GameObject::new(game_object::OBJ_UI, -12010),
				GameObject::new(game_object::OBJ_UI, -12010),
				GameObject::new(game_object::OBJ_UI, -12010),
				GameObject::new(game_object::OBJ_UI, -12010)
			],
			zoom_tools: [
				GameObject::new(game_object::OBJ_UI, -12010),
				GameObject::new(game_object::OBJ_UI, -12010),
				GameObject::new(game_object::OBJ_UI, -12010),
				GameObject::new(game_object::OBJ_UI, -12010)
			],
			dropdown_go: GameObject::new(game_object::OBJ_UI, -12010),
			tool_bar: TOOLS.rows[0].clone().map(|tool| ToolIcon::new(tool)),
			tool_bar_index: 0,
		}
	}

	pub fn init(&mut self, command_sender: &mut CommandSender) {
		let bar = self.bar_go.create(command_sender) as f32;
		self.bar_go.set_sprite(command_sender, sprite::EDITOR_TOP_BAR);

		for i in 0..4 {
			self.small_tools[i].create(command_sender);
			self.small_tools[i].set_sprite(command_sender, sprite::TOOL_SMALL);
			self.small_tools[i].set_real(command_sender, 2, bar);
			self.small_tools[i].set_real(command_sender, 0, (2 + 20 * (i & 1)) as f32);
			self.small_tools[i].set_real(command_sender, 1, (2 + 19 * ((i & 2) >> 1)) as f32);

			let zt = self.zoom_tools[i].create(command_sender) as f32;
			self.zoom_tools[i].set_sprite(command_sender, sprite::TOOL_ZOOMED);
			self.zoom_tools[i].set_real(command_sender, 0, ((i as u32 * 19 + 404) * ZOOM_FACTOR) as f32);
			self.zoom_tools[i].set_real(command_sender, 1, -1024.0);
			self.zoom_tools[i].set_scale(command_sender, ZOOM_FACTOR as f32, ZOOM_FACTOR as f32);

			let mut go = GameObject::new(game_object::OBJ_UI, -12020);
			go.create(command_sender);
			go.set_real(command_sender, 0, (2 * ZOOM_FACTOR) as f32);
			go.set_real(command_sender, 1, (2 * ZOOM_FACTOR) as f32);
			go.set_real(command_sender, 2, zt);
			go.set_scale(command_sender, ZOOM_FACTOR as f32, ZOOM_FACTOR as f32);
			go.set_sprite(command_sender, match i {
				0 => sprite::TOOL_SELECT,
				1 => sprite::TOOL_MOVE,
				2 => sprite::ZOOM_IN,
				3 => sprite::TOOL_PAN,
				_ => unreachable!()
			});
		}

		let mut icons = GameObject::new(game_object::OBJ_UI, -12020);
		icons.create(command_sender);
		icons.set_sprite(command_sender, sprite::TOOL_SMALL_ICONS);
		icons.set_real(command_sender, 0, 2.0);
		icons.set_real(command_sender, 1, 2.0);
		icons.set_real(command_sender, 2, bar);
		icons.destroy_server_only();

		for i in 0..10 {
			self.tool_bar[i].create(command_sender, bar, 42.0 + (40 * i) as f32, 2.0);
		}

		self.dropdown_go.create(command_sender);
		self.dropdown_go.set_real(command_sender, 0, 447.0);
		self.dropdown_go.set_real(command_sender, 1, 22.0);
		self.dropdown_go.set_real(command_sender, 2, bar);
		self.dropdown_go.set_sprite(command_sender, sprite::TOOL_DROPDOWN_BUTTON);
	}

	pub fn set_row(&mut self, _command_sender: &mut CommandSender, row: usize) {
		for i in 0..10 {
			self.tool_bar[i].tool = TOOLS.rows[row][i].clone();
		}

		self.tool_bar_index = row;
	}

	pub fn highlight_appropriate(&mut self, command_sender: &mut CommandSender, theme: LevelTheme, character: Character, mx: f32, my: f32, selected_tool: ArcTool) {
		for i in 0..4 {
			if self.enabled {
				let colour = if Arc::ptr_eq(&selected_tool, match i {
					0 => &TOOLS.move_,
					1 => &TOOLS.pan,
					2 => &TOOLS.select,
					3 => &TOOLS.delete,
					_ => unreachable!()
				}) { 0xffff }
				else if Shape::rect_from_pos_size((2 + 20 * (i & 1)) as f32, (2 + 19 * ((i & 2) >> 1)) as f32, 16.0, 16.0).contains(mx, my)
				{ 0xf2d5e3 } else { 0xffffff };

				if colour != self.small_tools[i].colour {
					self.small_tools[i].set_colour(command_sender, colour);
				}
			}

			if self.zoom_enabled {
				let colour = if Arc::ptr_eq(&selected_tool, match i {
					0 => &TOOLS.select,
					1 => &TOOLS.move_,
					2 => &TOOLS.undo,
					3 => &TOOLS.pan,
					_ => unreachable!()
				}) { 0xffff }
				else if Shape::rect_from_pos_size((404 + i * 19) as f32, 1.0, 18.0, 18.0).contains(mx, my)
				{ 0xf2d5e3 } else { 0xffffff };

				if colour != self.zoom_tools[i].colour {
					self.zoom_tools[i].set_colour(command_sender, colour);
				}
			}
		}

		if self.enabled {
			for tool in &mut self.tool_bar {
				tool.update(command_sender, theme, character, &selected_tool, mx, my, None);
			}
		}
	}

	pub fn get_tool_at(&self, mx: f32, my: f32) -> Option<ArcTool> {
		if self.enabled {
			for i in 0..4 {
				if Shape::rect_from_pos_size((2 + 20 * (i & 1)) as f32, (2 + 19 * ((i & 2) >> 1)) as f32, 16.0, 16.0).contains(mx, my) {
					return TOOLS.from_id(i);
				}
			}

			for tool in &self.tool_bar {
				if tool.contains(mx, my) {
					return Some(tool.tool.clone());
				}
			}
		}

		if self.zoom_enabled {
			for i in 0..4 {
				if i == 2 {
					continue;
				}
				
				if Shape::rect_from_pos_size((404 + i * 19) as f32, 1.0, 18.0, 18.0).contains(mx, my) {
					return Some(match i {
						0 => &TOOLS.select,
						1 => &TOOLS.move_,
						3 => &TOOLS.pan,
						_ => unreachable!()
					}.clone());
				}
			}
		}

		None
	}

	pub fn enable(&mut self, command_sender: &mut CommandSender) {
		self.enabled = true;
		self.bar_go.set_real(command_sender, 1, 0.0);
	}

	pub fn disable(&mut self, command_sender: &mut CommandSender) {
		self.enabled = false;
		self.bar_go.set_real(command_sender, 1, -1024.0);
	}

	pub fn enabled(&self) -> bool {
		self.enabled
	}

	pub fn enable_zoom(&mut self, command_sender: &mut CommandSender) {
		self.zoom_enabled = true;
		for i in 0..4 {
			self.zoom_tools[i].set_real(command_sender, 1, ZOOM_FACTOR as f32);
		}
	}

	pub fn disable_zoom(&mut self, command_sender: &mut CommandSender) {
		self.zoom_enabled = false;
		for i in 0..4 {
			self.zoom_tools[i].set_real(command_sender, 1, -1024.0);
		}
	}
}