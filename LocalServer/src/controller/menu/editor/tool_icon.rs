use std::sync::Arc;

use crate::controller::{command_handler::CommandSender, game_object::{self, GameObject}, level::{Character, LevelTheme}, menu::shape::Shape, sprite};

use super::tool::Tool;

pub struct ToolIcon {
	pub tool: Arc<dyn Tool + Send + Sync>,
	backing: GameObject,
	icon: GameObject,
	backing_colour: u32,
	icon_sprite: i32,
	x: f32,
	y: f32
}

impl ToolIcon {
	pub fn new(tool: Arc<dyn Tool + Send + Sync>) -> Self {
		Self {
			tool,
			backing: GameObject::new(game_object::OBJ_UI, -12500),
			icon: GameObject::new(game_object::OBJ_UI, -12501),
			backing_colour: 0,
			icon_sprite: -1,
			x: 0.0,
			y: 0.0
		}
	}

	pub fn create(&mut self, command_sender: &mut CommandSender, parent: f32, x: f32, y: f32) {
		let back = self.backing.create(command_sender);
		self.icon.create(command_sender);

		self.icon.set_real(command_sender, 0, 2.0);
		self.icon.set_real(command_sender, 1, 2.0);
		self.icon.set_real(command_sender, 2, back as f32);

		self.backing.set_real(command_sender, 0, x);
		self.backing.set_real(command_sender, 1, y);
		self.backing.set_real(command_sender, 2, parent);
		self.backing.set_sprite(command_sender, sprite::TOOL);

		self.x = x;
		self.y = y;
	}

	pub fn destroy(&mut self, command_sender: &mut CommandSender) {
		self.icon.destroy(command_sender);
		self.backing.destroy(command_sender);
	}

	pub fn update(&mut self, command_sender: &mut CommandSender, theme: LevelTheme, character: Character, selected_tool: &Arc<dyn Tool + Send + Sync>, rmx: f32, rmy: f32, hover: Option<bool>) {
		let icon_spr = self.tool.sprite(theme, character);
		if icon_spr != self.icon_sprite {
			self.icon_sprite = icon_spr;
			self.icon.set_sprite(command_sender, icon_spr);
		}

		let backing_col = if Arc::ptr_eq(selected_tool, &self.tool) { 0xffff }
		else if hover.unwrap_or(Shape::rect_from_pos_size(self.x, self.y, 36.0, 36.0).contains(rmx, rmy)) { 0xf2d5e3 }
		else { 0xffffff };

		if backing_col != self.backing_colour {
			self.backing_colour = backing_col;
			self.backing.set_colour(command_sender, backing_col);
		}
	}

	pub fn contains(&self, rmx: f32, rmy: f32) -> bool {
		Shape::rect_from_pos_size(self.x, self.y, 36.0, 36.0).contains(rmx, rmy)
	}
}