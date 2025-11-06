use std::mem;

use crate::controller::{command_handler::{CommandOutput, CommandSender}, game_object::{self, GameObject}, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;
use super::{tile_manager::Tile, Level, LevelObject, LevelTheme, AABB};

const DEPTH: i32 = 18;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Symbol {
	Circle,
	ArrowU,
	ArrowUR,
	ArrowR,
	ArrowDR,
	ArrowD,
	ArrowDL,
	ArrowL,
	ArrowUL,
	NoPhotography,
	VCord,
	HCord,
}

impl Symbol {
	const fn parse(id: u8) -> Option<Self> {
		if id < 12 {
			Some(unsafe { mem::transmute(id) })
		} else {
			None
		}
	}

	const fn depth(self) -> i32 {
		match self {
			Self::HCord |
			Self::VCord => 0,
			_ => DEPTH
		}
	}
}

pub struct SymbolObject {
	pub x: i32, pub y: i32,
	pub symbol: Symbol
}

impl LevelObject for SymbolObject {
	fn create(&self, command_sender: &mut dyn CommandOutput, level: &Level, _return_object: bool) -> GameObject {
		if self.symbol == Symbol::NoPhotography {
			let mut go = GameObject::new(game_object::OBJ_NO_PHOTOGRAPHY, -10);
			go.x = (self.x * 32 + 16) as f32;
			go.y = (self.y * 32 + 16) as f32;
			go.create(command_sender);
			go.destroy_server_only();
			go
		}

		else {
			let mut go = GameObject::new(game_object::OBJ_BLANK, self.symbol.depth());
			go.x = (self.x * 32 + 16) as f32;
			go.y = (self.y * 32 + 16) as f32;
			go.create(command_sender);
			self.build(command_sender, &mut go, level);
			go.destroy_server_only();
			go
		}
	}

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject> {
		let mut go = GameObject::new(game_object::OBJ_NO_ANIM, DEPTH);
		go.x = (self.x * 32 + 16) as f32;
		go.y = (self.y * 32 + 16) as f32;
		go.create(command_sender);
		self.build(command_sender, &mut go, level);

		vec![go]
	}

	fn bounding_box(&self) -> AABB {
		match self.symbol {
			Symbol::VCord => AABB {
				x: self.x as f32 + 0.4375,
				y: self.y as f32,
				width: 0.125,
				height: 1.0
			},
			Symbol::HCord => AABB {
				x: self.x as f32,
				y: self.y as f32 + 0.4375,
				width: 1.0,
				height: 0.125
			},
			_ => AABB {
				x: self.x as f32,
				y: self.y as f32,
				width: 1.0,
				height: 1.0
			}
		}
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn types(&self) -> u32 {
		super::L_SYMBOL
	}

	fn serialized_type(&self) -> u8 {
		11
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;
		to.write(&[self.symbol as u8])?;
		Ok(())
	}

	#[cfg(not(feature = "verify"))]
	fn context_menu_items(&self, _theme: LevelTheme) -> Vec<ContextMenuItem> {
		vec![ContextMenuItem::IconList(
			vec![
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowUL as i32,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowU as i32,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowUR as i32, sprite::SYM_ICON_NO_PHOTOGRAPHY,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowL as i32,
				sprite::SYM_ICON_CIRCLE + Symbol::Circle as i32,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowR as i32, sprite::SYM_ICON_VCORD,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowDL as i32,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowD as i32,
				sprite::SYM_ICON_CIRCLE + Symbol::ArrowDR as i32, sprite::SYM_ICON_HCORD,
			],
			match self.symbol {
				Symbol::ArrowUL => 0,
				Symbol::ArrowU => 1,
				Symbol::ArrowUR => 2,
				Symbol::ArrowL => 4,
				Symbol::Circle => 5,
				Symbol::ArrowR => 6,
				Symbol::ArrowDL => 8,
				Symbol::ArrowD => 9,
				Symbol::ArrowDR => 10,

				Symbol::NoPhotography => 3,
				Symbol::VCord => 7,
				Symbol::HCord => 11,
			},
			1.0
		)]
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, _theme: LevelTheme) -> Vec<UndoAction> {
		let new = match action {
			sprite::SYM_ICON_NO_PHOTOGRAPHY => Symbol::NoPhotography,
			sprite::SYM_ICON_VCORD => Symbol::VCord,
			sprite::SYM_ICON_HCORD => Symbol::HCord,
			action => unsafe { mem::transmute((action - sprite::SYM_ICON_CIRCLE) as u8) }
		};

		if new != self.symbol {
			sound::play(command_sender, sound::SE_HOLD);
			let old = self.symbol;
			self.symbol = new;
			vec![UndoAction::ContextMenuAction(object, old as i32 + sprite::SYM_ICON_CIRCLE)]
		}

		else { vec![] }
	}
}

impl SymbolObject {
	fn build(&self, command_sender: &mut dyn CommandOutput, go: &mut GameObject, level: &Level) {
		match self.symbol {
			Symbol::NoPhotography => go.set_sprite(command_sender, sprite::NO_PHOTOGRAPHY),
			Symbol::VCord => go.set_sprite(command_sender, sprite::CORD),
			Symbol::HCord => {
				go.set_sprite(command_sender, sprite::CORD);
				go.set_rotation(command_sender, 90.0);
			}
			_ => {
				let tile = level.tile_manager().get(self.x, self.y);
				let clr = colour(level.theme, tile);
				if clr != 0xffffff {
					go.set_colour(command_sender, clr);
				}

				transform(command_sender, go, self.symbol);
			}
		}
	}

	pub const fn new(x: i32, y: i32) -> Self {
		Self { x, y, symbol: Symbol::Circle }
	}

	pub fn deserialize(data: &[u8]) -> Self {
		if data.len() > 4 {
			Self {
				x: i16::from_le_bytes(data[0..2].try_into().unwrap()) as i32,
				y: i16::from_le_bytes(data[2..4].try_into().unwrap()) as i32,
				symbol: Symbol::parse(data[4]).unwrap_or(Symbol::Circle),
			}
		} else {
			Self::new(0, 0)
		}
	}
}

const fn colour(theme: LevelTheme, tile: Tile) -> u32 {
	match (theme, tile.base()) {
		(_, Tile::BlockRed) => 0xff0000,
		(_, Tile::BlockBlue) => 0xff,
		(_, Tile::BlockGreen) => 0xff4f97,
		(_, Tile::BlockPurple) => 0xff00,

		(LevelTheme::DreamFields, Tile::Water) |
		(LevelTheme::DreamFields, Tile::BGCommon1) |
		(LevelTheme::DreamFields, Tile::None) => 0xff,
		(LevelTheme::DreamFields, _) => 0,

		(LevelTheme::BambooForest, Tile::Water) |
		(LevelTheme::BambooForest, Tile::BGCommon1) |
		(LevelTheme::BambooForest, Tile::None) => 0xc0,
		(LevelTheme::BambooForest, _) => 0xffffff,

		(LevelTheme::AzureWinter, Tile::None) => 0xab5745,
		(LevelTheme::AzureWinter, Tile::BGCommon1) => 0xe9d4cc,
		(LevelTheme::AzureWinter, _) => 0,

		(LevelTheme::ForestOfMagic, Tile::Water) |
		(LevelTheme::ForestOfMagic, Tile::None) => 0xff,
		(LevelTheme::ForestOfMagic, Tile::BGCommon1) => 0xf1f1f1,
		(LevelTheme::ForestOfMagic, _) => 0,

		(LevelTheme::UltramarineRain, Tile::Water) |
		(LevelTheme::UltramarineRain, Tile::GroundBase) => 0x361e00,
		(LevelTheme::UltramarineRain, _) => 0xc1af95,

		(LevelTheme::OutsideWorld, Tile::Water) => 0,
		(LevelTheme::OutsideWorld, _) => 0xffffff,

		(LevelTheme::Koumakan, Tile::Water) => 0xff,
		(LevelTheme::Koumakan, _) => 0xff00,

		(LevelTheme::ShiningNeedleCastle, Tile::AltGroundBase) => 0x4c3b38,
		(LevelTheme::ShiningNeedleCastle, Tile::Water) => 0x7d7393,
		(LevelTheme::ShiningNeedleCastle, _) => 0xcec9da,

		(LevelTheme::DreamScraps, _) => 0,

		(LevelTheme::TheDepths, Tile::AltGroundBase) |
		(LevelTheme::TheDepths, Tile::BGCommon1) => 0xffffff,
		(LevelTheme::TheDepths, _) => 0,

		(LevelTheme::JerryAttack, Tile::GroundBase) |
		(LevelTheme::JerryAttack, Tile::AltGroundBase) => 0xffffff,
		(LevelTheme::JerryAttack, _) => 0,

		(LevelTheme::FarawayLabyrinth, Tile::GroundBase) => 0,
		(LevelTheme::FarawayLabyrinth, Tile::AltGroundBase) => 0xffffff,
		(LevelTheme::FarawayLabyrinth, Tile::Water) => 0xff,
		(LevelTheme::FarawayLabyrinth, _) => 0xff00,

		(LevelTheme::DancingStars, Tile::AltGroundBase) |
		(LevelTheme::DancingStars, Tile::Water) => 0,
		(LevelTheme::DancingStars, _) => 0xffffff,

		(LevelTheme::ReachOutToThatMoon, Tile::AltGroundBase) |
		(LevelTheme::ReachOutToThatMoon, Tile::Water) => 0,
		(LevelTheme::ReachOutToThatMoon, Tile::None) => 0xa6f9ff,
		(LevelTheme::ReachOutToThatMoon, _) => 0xffffff,

		(LevelTheme::MindBreak, Tile::Water) => 0x50,
		(LevelTheme::MindBreak, _) => 0xffffff,

		(LevelTheme::Fireflies, Tile::Water) => 0,
		(LevelTheme::Fireflies, _) => 0xa6f9ff,

		(LevelTheme::Cirno, Tile::AltGroundBase) |
		(LevelTheme::Cirno, Tile::BGCommon1) => 0xffffff,
		(LevelTheme::Cirno, _) => 0,

		(LevelTheme::Rumia, Tile::AltGroundBase) |
		(LevelTheme::Rumia, Tile::GroundBase) |
		(LevelTheme::Rumia, Tile::BGCommon1) => 0xffffff,
		(LevelTheme::Rumia, _) => 0x202020,

		(LevelTheme::Seija, _) => 0,

		(LevelTheme::Rasobi, Tile::GroundBase) => 0,
		(LevelTheme::Rasobi, Tile::AltGroundBase) => 0xffffff,
		(LevelTheme::Rasobi, _) => 0xff,

		(LevelTheme::Purple, Tile::Water) |
		(LevelTheme::Purple, Tile::AltGroundBase) |
		(LevelTheme::Purple, Tile::GroundBase) => 0xff,
		(LevelTheme::Purple, _) => 0xb45687,

		(LevelTheme::Entrance, _) => 0xff
	}
}

pub fn transform(command_sender: &mut dyn CommandOutput, go: &mut GameObject, symbol: Symbol) {
	go.set_sprite(command_sender, match symbol {
		Symbol::Circle => sprite::SYM_CIRCLE,
		Symbol::ArrowU |
		Symbol::ArrowD |
		Symbol::ArrowL |
		Symbol::ArrowR => sprite::SYM_ARROW_AA,
		_ => sprite::SYM_ARROW_DIAG
	});

	if symbol == Symbol::ArrowU || symbol == Symbol::ArrowD {
		go.set_rotation(command_sender, 90.0);
	}

	let (scale_x, scale_y) = match symbol {
		Symbol::ArrowDR => (1.0, -1.0),
		Symbol::ArrowL |
		Symbol::ArrowUL |
		Symbol::ArrowD => (-1.0, 1.0),
		Symbol::ArrowDL => (-1.0, -1.0),
		_ => return
	};

	go.set_scale(command_sender, scale_x, scale_y);
}