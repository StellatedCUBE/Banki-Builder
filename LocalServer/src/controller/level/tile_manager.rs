use std::{collections::{HashMap, HashSet}, i32, mem, sync::{atomic::{AtomicU8, Ordering}, Mutex}, u32};

use anyhow::Result;
use bincode::{Decode, Encode};
use rand::random;
use smallvec::SmallVec;

use crate::controller::{bg, command_handler::{Command, CommandOutput, CommandSender}, game_object::{self, GameObject}, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;
use super::{ground_decoration, tile_split::TileSplit, Level, LevelObject, LevelTheme, SubObjectDeleteUndoAction, AABB};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Tile {
	None,
	Water,
	GroundBase,
	GroundV1,
	GroundV2,
	GroundV3,
	GroundV4,
	GroundV5,
	GroundV6,
	GroundV7,
	GroundV8,
	BGCommon1,
	BGCommon2,
	BGCommon3,
	BGRare1,
	BGRare2,
	BGRare3,
	BGRare4,
	BGRare5,
	BGRare6,
	BGRare7,
	BGRare8,
	BGRare9,
	BGRare10,
	BGRare11,
	BGRare12,
	BGRare13,
	BGRare14,
	BGRare15,
	BGRare16,
	BGRare17,
	BGRare18,
	AltGroundBase,
	AltGroundV1,
	AltGroundV2,
	AltGroundV3,
	BlockRed,
	BlockBlue,
	BlockPurple,
	BlockGreen,
	Blocked
}

#[derive(Clone, Copy, PartialEq)]
pub enum OverwriteDirection {
	None,
	SelfOverwritesOther,
	OtherOverwritesSelf,
}

impl Tile {
	pub const fn types(self) -> u32 {
		match self.base() {
			Self::None => 0,
			Self::BlockRed |
			Self::BlockBlue |
			Self::BlockPurple |
			Self::BlockGreen |
			Self::AltGroundBase |
			Self::GroundBase => super::L_TILE | super::PL_IMMUTABLE_BLOCK | super::L_DECORATION,
			Self::Water |
			Self::BGCommon1 => super::L_TILE | super::L_DECORATION,
			_ => unreachable!()
		}
	}

	pub const fn sprite(self, theme: LevelTheme) -> i32 {
		match (self.base(), theme) {
			(Self::None, _) => -1,
			
			(Self::Water, _) => sprite::WATER,

			(Self::GroundBase, LevelTheme::Rasobi) |
			(Self::GroundBase, LevelTheme::FarawayLabyrinth) => sprite::WALL,
			(Self::GroundBase, LevelTheme::ReachOutToThatMoon) |
			(Self::GroundBase, LevelTheme::Seija) => sprite::GROUND_BLOCK_1 + LevelTheme::DancingStars as i32,
			(Self::GroundBase, LevelTheme::Entrance) => sprite::GROUND_BLOCK_1 + LevelTheme::DreamScraps as i32,
			(Self::GroundBase, theme) => sprite::GROUND_BLOCK_1 + theme as i32,

			(Self::BGCommon1, LevelTheme::JerryAttack) => sprite::JERRY,
			(Self::BGCommon1, LevelTheme::FarawayLabyrinth) => sprite::BG_TILE_1 + LevelTheme::Koumakan as i32,
			(Self::BGCommon1, LevelTheme::Entrance) => sprite::BG_TILE_1 + LevelTheme::DreamScraps as i32,
			(Self::BGCommon1, LevelTheme::Rasobi) => sprite::BG_TILE_1 + LevelTheme::AzureWinter as i32,
			(Self::BGCommon1, LevelTheme::ReachOutToThatMoon) |
			(Self::BGCommon1, LevelTheme::Seija) |
			(Self::BGCommon1, LevelTheme::DancingStars) => sprite::BG_TILE_STARS,
			(Self::BGCommon1, LevelTheme::MindBreak) => sprite::BG_TILE_MINDBREAK,
			(Self::BGCommon1, LevelTheme::Fireflies) => sprite::BG_TILE_FIREFLIES,
			(Self::BGCommon1, LevelTheme::Cirno) => sprite::BG_TILE_CIRNO,
			(Self::BGCommon1, LevelTheme::Rumia) => sprite::BG_TILE_RUMIA,
			(Self::BGCommon1, LevelTheme::Purple) => sprite::BG_TILE_PURPLE,
			(Self::BGCommon1, theme) => sprite::BG_TILE_1 + theme as i32,

			(Self::AltGroundBase, LevelTheme::Fireflies) => sprite::ALT_GROUND_BROWN,
			(Self::AltGroundBase, LevelTheme::Rasobi) |
			(Self::AltGroundBase, LevelTheme::FarawayLabyrinth) => sprite::WALL + 1,
			(Self::AltGroundBase, LevelTheme::Seija) |
			(Self::AltGroundBase, LevelTheme::ReachOutToThatMoon) |
			(Self::AltGroundBase, LevelTheme::DreamScraps) |
			(Self::AltGroundBase, LevelTheme::Entrance) |
			(Self::AltGroundBase, LevelTheme::DancingStars) => sprite::ALT_GROUND_WHITE,
			(Self::AltGroundBase, LevelTheme::Purple) => sprite::ALT_GROUND_PURPLE,
			(Self::AltGroundBase, LevelTheme::MindBreak) |
			(Self::AltGroundBase, LevelTheme::Rumia) => sprite::ALT_GROUND_RED,
			(Self::AltGroundBase, LevelTheme::Cirno) => sprite::ALT_GROUND_1 + LevelTheme::UltramarineRain as i32,
			(Self::AltGroundBase, theme) => sprite::ALT_GROUND_1 + theme as i32,

			(Self::BlockRed, _) => sprite::BLOCK_RED,
			(Self::BlockBlue, _) => sprite::BLOCK_BLUE,
			(Self::BlockPurple, _) => sprite::BLOCK_PURPLE,
			(Self::BlockGreen, _) => sprite::BLOCK_GREEN,

			_ => panic!("Bad sprite access")
		}
	}

	pub const fn overwrites(self, other: Tile) -> OverwriteDirection {
		if self as u8 == other as u8 || self as u8 == Tile::None as u8 {
			OverwriteDirection::OtherOverwritesSelf
		}

		else if other as u8 == Tile::None as u8 {
			OverwriteDirection::SelfOverwritesOther
		}

		else if self.bg() {
			OverwriteDirection::OtherOverwritesSelf
		}

		else if self.ground_any() && other.ground_any() {
			OverwriteDirection::SelfOverwritesOther
		}

		else {
			OverwriteDirection::None
		}
	}

	pub const fn ground_regular(self) -> bool {
		self as u8 >= Self::GroundBase as u8 && self as u8 <= Self::GroundV8 as u8
	}

	pub const fn ground_alt(self) -> bool {
		self as u8 >= Self::AltGroundBase as u8 && self as u8 <= Self::AltGroundV3 as u8
	}

	pub const fn ground_any(self) -> bool {
		self.ground_regular() || self.ground_alt()
	}

	pub const fn block(self) -> bool {
		self as u8 >= Self::BlockRed as u8 && self as u8 <= Self::BlockGreen as u8
	}

	pub const fn solid(self) -> bool {
		self.ground_any() || self.block()
	}

	pub const fn ground_themed(self, theme: LevelTheme) -> bool {
		match theme {
			LevelTheme::AzureWinter |
			LevelTheme::Purple |
			LevelTheme::ForestOfMagic |
			LevelTheme::DreamScraps |
			LevelTheme::Entrance |
			LevelTheme::Fireflies |
			LevelTheme::JerryAttack |
			LevelTheme::ShiningNeedleCastle |
			LevelTheme::Koumakan => self.ground_regular(),
			_ => self.ground_any()
		}
	}

	pub const fn ground_rare(self) -> bool {
		self as u8 > Self::GroundBase as u8 && self as u8 <= Self::GroundV8 as u8
	}

	pub const fn bg(self) -> bool {
		self as u8 >= Self::BGCommon1 as u8 && self as u8 <= Self::BGRare18 as u8
	}

	pub const fn bg_or_ground(self) -> bool {
		self as u8 >= Self::GroundBase as u8 && self as u8 <= Self::Blocked as u8
	}

	pub const fn bg_or_alt_ground(self) -> bool {
		self as u8 >= Self::BGCommon1 as u8 && self as u8 <= Self::Blocked as u8
	}

	pub const fn base(self) -> Self {
		if self.ground_regular() { Self::GroundBase }
		else if self.bg() { Self::BGCommon1 }
		else if self.ground_alt() { Self::AltGroundBase }
		else { self }
	}

	pub fn roll(self) -> Self {
		if self == Self::GroundBase || self == Self::BGCommon1 || self == Self::AltGroundBase {
			let p: f32 = random();

			if self == Self::GroundBase {
				if p < 0.011 {
					Self::GroundV1
				} else if p < 0.022 {
					Self::GroundV2
				} else if p < 0.033 {
					Self::GroundV3
				} else if p < 0.044 {
					Self::GroundV4
				} else if p < 0.055 {
					Self::GroundV5
				} else if p < 0.066 {
					Self::GroundV6
				} else if p < 0.077 {
					Self::GroundV7
				} else if p < 0.088 {
					Self::GroundV8
				} else {
					Self::GroundBase
				}
			}

			else if self == Self::AltGroundBase {
				if p < 0.25 {
					Self::AltGroundBase
				} else if p < 0.5 {
					Self::AltGroundV1
				} else if p < 0.75 {
					Self::AltGroundV2
				} else {
					Self::AltGroundV3
				}
			}

			else if p < 0.0025 {
				Self::BGRare1
			} else if p < 0.005 {
				Self::BGRare2
			} else if p < 0.0075 {
				Self::BGRare3
			} else if p < 0.01 {
				Self::BGRare4
			} else if p < 0.0125 {
				Self::BGRare5
			} else if p < 0.015 {
				Self::BGRare6
			} else if p < 0.0175 {
				Self::BGRare7
			} else if p < 0.02 {
				Self::BGRare8
			} else if p < 0.0225 {
				Self::BGRare9
			} else if p < 0.025 {
				Self::BGRare10
			} else if p < 0.0275 {
				Self::BGRare11
			} else if p < 0.03 {
				Self::BGRare12
			} else if p < 0.0325 {
				Self::BGRare13
			} else if p < 0.035 {
				Self::BGRare14
			} else if p < 0.0375 {
				Self::BGRare15
			} else if p < 0.04 {
				Self::BGRare16
			} else if p < 0.0425 {
				Self::BGRare17
			} else if p < 0.045 {
				Self::BGRare18
			} else if p < 0.3633 {
				Self::BGCommon1
			} else if p < 0.6817 {
				Self::BGCommon2
			} else {
				Self::BGCommon3
			}
		}

		else { self }
	}

	fn reroll(self) -> Self {
		self.base().roll()
	}

	pub const fn variant(self) -> u32 {
		self as u32 - self.base() as u32
	}

	pub const fn parse(id: u8) -> Self {
		if id < Self::Blocked as u8 {
			unsafe { mem::transmute(id) }
		} else {
			Tile::None
		}
	}
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct GMTileData {
	pub x: i32,
	pub y: i32,
	pub tx: u32,
	pub ty: u32,
	pub width: u32,
	pub height: u32
}

impl GMTileData {
	fn create(self, command_sender: &mut dyn CommandOutput, ts: f32) {
		command_sender.send(Command::F32(vec![
			self.x as f32,
			(self.y + 65536) as f32,
			ts,
			self.tx as f32,
			self.ty as f32,
			(self.width * 32) as f32,
			(self.height * 32) as f32,
			(15090 + (self.x & 1)) as f32
		]));

		command_sender.send(Command::SetTile);
	}

	fn remove(self, command_sender: &mut dyn CommandOutput) {
		command_sender.send(Command::F32(vec![
			self.x as f32,
			(self.y + 65536) as f32,
			-1.0
		]));

		command_sender.send(Command::SetTile);
	}
}

#[derive(Default)]
struct Deco {
	dirty: bool,
	tiles: HashSet<GMTileData>
}

#[derive(Clone, Copy, Encode, Decode)]
struct SolidData {
	x: i16,
	y: i16,
	width: i16,
	height: i16
}

pub struct TileManager {
	pub tiles: HashMap<(i32, i32), [Tile; 256]>,
	marker_objects: Mutex<HashMap<(i32, i32), GameObject>>,
	theme: AtomicU8,
	pub splits: Mutex<HashSet<TileSplit>>,
	deco: Mutex<Deco>,
	snc_thick_ground: Mutex<HashMap<(i32, i32), bool>>,
	solids: Option<Vec<SolidData>>,
}

impl LevelObject for TileManager {
	fn sub_object_count(&self) -> usize { u32::MAX as usize }

	fn create(&self, command_sender: &mut dyn CommandOutput, level: &Level, _return_object: bool) -> GameObject {
		*self.marker_objects.lock().unwrap() = HashMap::new();
		*self.snc_thick_ground.lock().unwrap() = HashMap::new();
		self.theme.store(level.theme as u8, Ordering::Relaxed);
		self.load_splits(level);

		let bounds = level.bounding_box();
		self.create_any(command_sender, bounds);

		{
			let left = bounds.x as i32 - 1;
			let top = bounds.y as i32 - 1;
			let right = (bounds.x + bounds.width) as i32;
			let bottom = (bounds.y + bounds.height) as i32;
			let splits = self.splits.lock().unwrap().clone();

			if self.get(left + 1, top + 1) != Tile::None && !splits.contains(&TileSplit { x: left + 1, y: top + 1, vertical: false }) &&
				!splits.contains(&TileSplit { x: left + 1, y: top + 1, vertical: true }) {
				self.update_tile(command_sender, left, top, &bounds, Some(self.get(left + 1, top + 1).reroll()));
			}

			if self.get(right - 1, top + 1) != Tile::None && !splits.contains(&TileSplit { x: right - 1, y: top + 1, vertical: false }) &&
				!splits.contains(&TileSplit { x: right, y: top + 1, vertical: true }) {
				self.update_tile(command_sender, right, top, &bounds, Some(self.get(right - 1, top + 1).reroll()));
			}

			if self.get(left + 1, bottom - 1) != Tile::None && !splits.contains(&TileSplit { x: left + 1, y: bottom, vertical: false }) &&
				!splits.contains(&TileSplit { x: left + 1, y: bottom - 1, vertical: true }) {
				self.update_tile(command_sender, left, bottom, &bounds, Some(self.get(left + 1, bottom - 1).reroll()));
			}

			if self.get(right - 1, bottom - 1) != Tile::None && !splits.contains(&TileSplit { x: right - 1, y: bottom, vertical: false }) &&
				!splits.contains(&TileSplit { x: right, y: bottom - 1, vertical: true }) {
				self.update_tile(command_sender, right, bottom, &bounds, Some(self.get(right - 1, bottom - 1).reroll()));
			}

			for x in left + 1..right {
				let tile = self.get(x, top + 1);
				if tile != Tile::None && !splits.contains(&TileSplit { x, y: top + 1, vertical: false }) {
					self.update_tile(command_sender, x, top, &bounds, Some(tile.reroll()));
				}

				let tile = self.get(x, bottom - 1);
				if tile != Tile::None && !splits.contains(&TileSplit { x, y: bottom, vertical: false }) {
					self.update_tile(command_sender, x, bottom, &bounds, Some(tile.reroll()));
				}
			}

			for y in top + 1..bottom {
				let tile = self.get(left + 1, y);
				if tile != Tile::None && !splits.contains(&TileSplit { x: left + 1, y, vertical: true }) {
					self.update_tile(command_sender, left, y, &bounds, Some(tile.reroll()));
				}

				let tile = self.get(right - 1, y);
				if tile != Tile::None && !splits.contains(&TileSplit { x: right, y, vertical: true }) {
					self.update_tile(command_sender, right, y, &bounds, Some(tile.reroll()));
				}
			}
		}

		self.deco.lock().unwrap().tiles = HashSet::new();
		self.update_deco(command_sender);

		let splits = self.splits.lock().unwrap();

		if level.theme == LevelTheme::FarawayLabyrinth || level.theme == LevelTheme::Rasobi {
			for (x, y) in self.tiles.keys() {
				for i in 0..16 {
					for j in 0..16 {
						let tile = self.get(x + i, y + j);
						if tile.solid() {
							let mut go = GameObject::new(match tile.base() {
								Tile::GroundBase => 0,
								Tile::AltGroundBase => 1,
								_ => game_object::OBJ_WALL
							}, 15000);

							go.x = ((x + i) * 32) as f32;
							go.y = ((y + j) * 32) as f32;
							go.create(command_sender);
							go.destroy_server_only();

							if y + j == bounds.y as i32 && !splits.contains(&TileSplit { x: x + i, y: y + j, vertical: false }) {
								go = GameObject::new(game_object::OBJ_WALL, 0);
								go.x = ((x + i) * 32) as f32;
								go.y = (bounds.y - 256.0) * 32.0;
								go.create(command_sender);
								go.set_scale(command_sender, 1.0, 256.0);
								go.destroy_server_only();
							}
						}
					}
				}
			}
		}

		else {
			for solid in self.solids(&splits, bounds, true) {
				let mut go = GameObject::new(game_object::OBJ_WALL, 0);
				go.x = solid.x as f32 * 32.0;
				go.y = solid.y as f32 * 32.0;
				go.create(command_sender);
				if solid.width != 1 || solid.height != 1 {
					go.set_scale(command_sender, solid.width as f32, solid.height as f32);
				}
				go.destroy_server_only();
			}
		}

		GameObject::null()
	}

	#[cfg(not(feature = "verify"))]
    fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject> {
		self.theme.store(level.theme as u8, Ordering::Relaxed);
		self.load_splits(level);

		self.create_any(command_sender, AABB {
			x: -100000.0,
			y: -100000.0,
			width: 200000.0,
			height: 200000.0
		});

		self.deco.lock().unwrap().tiles = HashSet::new();
		self.update_deco(command_sender);

        vec![]
    }

	#[cfg(not(feature = "verify"))]
	fn destroy_editor_view(&self, command_sender: &mut dyn CommandOutput, _objects: &mut Vec<GameObject>, _level: &Level) {
		self.clear_deco(command_sender);

		let mut mo = self.marker_objects.lock().unwrap();

		for obj in mo.values_mut() {
			obj.destroy(command_sender);
		}

		*mo = HashMap::new();
		self.splits.lock().unwrap().clear();
	}

	fn can_delete(&self) -> bool {
		false
	}

	fn to_tile_manager(&self) -> &TileManager {
		self
	}

	fn to_tile_manager_mut(&mut self) -> &mut TileManager {
		self
	}

	fn bounding_box(&self) -> AABB {
		if self.tiles.is_empty() {
			return AABB::null();
		}

		let mut x_min = i32::MAX;
		let mut y_min = i32::MAX;
		let mut x_max = i32::MIN;
		let mut y_max = i32::MIN;

		for ((cx, cy), data) in &self.tiles {
			let cx = *cx;
			let cy = *cy;
			
			if cx >= x_min && cy >= y_min && cx <= x_max - 16 && cy <= y_max - 16 {
				continue;
			}

			for i in 0..256 {
				if data[i as usize] != Tile::None {
					let x = cx + (i & 15);
					let y = cy + (i >> 4);
					x_min = x_min.min(x);
					y_min = y_min.min(y);
					x_max = x_max.max(x + 1);
					y_max = y_max.max(y + 1);
				}
			}
		}

		if x_max < x_min {
			AABB::null()
		} else {
			AABB {
				x: x_min as f32, y: y_min as f32,
				width: (x_max - x_min) as f32, height: (y_max - y_min) as f32
			}
		}
	}

	fn move_by(&mut self, _x: i32, _y: i32) {
		unreachable!();
	}

	fn delete_sub_object(&mut self, command_sender: &mut CommandSender, _objects: &mut Vec<GameObject>, _object: usize, sub_object: usize)
	-> SubObjectDeleteUndoAction {
		let x = (sub_object & 65535) as i32 - 32768;
		let y = (sub_object >> 16) as i32 - 32768;

		//println!("{} {} {}", sub_object, x, y);

		match self.get(x, y) {
			Tile::None => SubObjectDeleteUndoAction::None,
			tile => {
				self.set_and_update(command_sender, x, y, Tile::None);
				SubObjectDeleteUndoAction::Some(UndoAction::SetTile(x, y, tile))
			}
		}
	}

	fn sub_object_bounding_box(&self, sub_object: usize) -> AABB {
		let x = ((sub_object & 65535) as i32 - 32768) as f32;
		let y = ((sub_object >> 16) as i32 - 32768) as f32;

		AABB { x, y, width: 1.0, height: 1.0 }
	}

	fn types(&self) -> u32 {
		unreachable!()
	}

	fn sub_object_types(&self, sub_object: usize) -> u32 {
		let x = (sub_object & 65535) as i32 - 32768;
		let y = (sub_object >> 16) as i32 - 32768;

		self.get(x, y).types()
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&bincode::encode_to_vec(&self.solids, bincode::config::standard())?)?;

		for ((cx, cy), data) in &self.tiles {
			if data.iter().any(|tile| *tile != Tile::None) {
				to.write(&(*cx as i16).to_le_bytes())?;
				to.write(&(*cy as i16).to_le_bytes())?;
				to.write(&data.map(|t| t as u8))?;
			}
		}

		Ok(())
	}

	fn serialized_type(&self) -> u8 {0}

	#[cfg(not(feature = "verify"))]
	fn sub_object_context_menu_items(&self, sub_object: usize, theme: LevelTheme) -> Vec<ContextMenuItem> {
		let x = (sub_object & 65535) as i32 - 32768;
		let y = (sub_object >> 16) as i32 - 32768;
		let tile = self.get(x, y);
		if tile.block() {
			vec![ContextMenuItem::IconList(vec![
				Tile::BlockRed.sprite(theme),
				Tile::BlockBlue.sprite(theme),
				Tile::BlockPurple.sprite(theme),
				Tile::BlockGreen.sprite(theme)
			], tile as usize - Tile::BlockRed as usize, 0.5)]
		}

		else { vec![] }
	}

	fn handle_sub_object_context_menu_action(
		&mut self,
		command_sender: &mut CommandSender,
		_object: usize,
		sub_object: usize,
		action: i32,
		_theme: LevelTheme
	) -> (bool, Vec<UndoAction>) {
		sound::play(command_sender, sound::SE_HOLD);

		let x = (sub_object & 65535) as i32 - 32768;
		let y = (sub_object >> 16) as i32 - 32768;
		let old = self.get(x, y);
		let new = unsafe { mem::transmute((action - sprite::BLOCK_RED) as u8 + Tile::BlockRed as u8) };
		self.set(x, y, new);
		self.update_tile(command_sender, x, y, &AABB {
			x: -100000.0,
			y: -100000.0,
			width: 200000.0,
			height: 200000.0
		}, None);

		(false, vec![UndoAction::SetTile(x, y, old)])
	}
}

impl TileManager {
	pub fn new() -> Self {
		let mut tm = Self {
			tiles: HashMap::new(),
			marker_objects: Mutex::default(),
			theme: AtomicU8::new(0),
			splits: Mutex::default(),
			deco: Mutex::default(),
			snc_thick_ground: Mutex::default(),
			solids: None,
		};

		tm.set(0, 9, Tile::GroundBase.roll());
		tm.set(1, 9, Tile::GroundBase.roll());
		tm.set(2, 9, Tile::GroundBase.roll());
		tm.set(3, 9, Tile::GroundBase.roll());
		tm.set(4, 9, Tile::GroundBase.roll());

		tm
	}

	pub fn deserialize(from: &[u8]) -> Result<Self> {
		let (solids, len) = bincode::decode_from_slice(from, bincode::config::standard())?;
		let from = &from[len..];
		let mut tiles = HashMap::new();

		for i in 0..from.len() / 260 {
			let x = i16::from_le_bytes(from[i * 260..i * 260 + 2].try_into()?) as i32;
			let y = i16::from_le_bytes(from[i * 260 + 2..i * 260 + 4].try_into()?) as i32;
			let mut data = [Tile::None; 256];
			for j in 0..256 {
				data[j] = Tile::parse(from[i * 260 + 4 + j]);
			}
			tiles.insert((x, y), data);
		}

		Ok(Self {
			tiles,
			marker_objects: Mutex::default(),
			theme: AtomicU8::new(0),
			splits: Mutex::default(),
			deco: Mutex::default(),
			snc_thick_ground: Mutex::default(),
			solids,
		})
	}

	pub fn to_sub_id(x: i32, y: i32) -> usize {
		let x = (x + 32768) as usize;
		let y = (y + 32768) as usize;
		x | (y << 16)
	}

	pub fn from_sub_id(sub_object: usize) -> (i32, i32) {
		let x = (sub_object & 65535) as i32 - 32768;
		let y = (sub_object >> 16) as i32 - 32768;
		(x, y)
	}

	pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
		let key = (x & !15, y & !15);
		let i = ((x & 15) | ((y & 15) << 4)) as usize;
		if let Some(chunk) = self.tiles.get_mut(&key) {
			chunk[i] = tile;
		} else {
			let mut val = [Tile::None; 256];
			val[i] = tile;
			self.tiles.insert(key, val);
		}
	}

	pub fn get(&self, x: i32, y: i32) -> Tile {
		let key = (x & !15, y & !15);
		if let Some(chunk) = self.tiles.get(&key) {
			let i = ((x & 15) | ((y & 15) << 4)) as usize;
			chunk[i]
		} else {
			Tile::None
		}
	}

	fn left(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		if self.splits.lock().unwrap().contains(&TileSplit { x, y, vertical: true }) { Tile::Blocked }
		else { self.get((bounds.x as i32).max(x - 1), bounds.clamp_y(y)) }
	}

	fn up(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		if self.splits.lock().unwrap().contains(&TileSplit { x, y, vertical: false }) { Tile::Blocked }
		else { self.get(bounds.clamp_x(x), (bounds.y as i32).max(y - 1)) }
	}

	fn right(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		if self.splits.lock().unwrap().contains(&TileSplit { x: x + 1, y, vertical: true }) { Tile::Blocked }
		else { self.get(((bounds.x + bounds.width) as i32 - 1).min(x + 1), bounds.clamp_y(y)) }
	}

	fn down(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		if self.splits.lock().unwrap().contains(&TileSplit { x, y: y + 1, vertical: false }) { Tile::Blocked }
		else { self.get(bounds.clamp_x(x), ((bounds.y + bounds.height) as i32 - 1).min(y + 1)) }
	}

	fn upleft(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		let splits = self.splits.lock().unwrap();
		if splits.contains(&TileSplit { x: x - 1, y, vertical: false }) || splits.contains(&TileSplit { x, y: y - 1, vertical: true })
		{ Tile::Blocked }
		else { self.get((bounds.x as i32).max(x - 1), (bounds.y as i32).max(y - 1)) }
	}

	fn upright(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		let splits = self.splits.lock().unwrap();
		if splits.contains(&TileSplit { x: x + 1, y, vertical: false }) || splits.contains(&TileSplit { x: x + 1, y: y - 1, vertical: true })
		{ Tile::Blocked }
		else { self.get(((bounds.x + bounds.width) as i32 - 1).min(x + 1), (bounds.y as i32).max(y - 1)) }
	}

	fn downleft(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		let splits = self.splits.lock().unwrap();
		if splits.contains(&TileSplit { x: x - 1, y: y + 1, vertical: false }) || splits.contains(&TileSplit { x, y: y + 1, vertical: true })
		{ Tile::Blocked }
		else { self.get((bounds.x as i32).max(x - 1), ((bounds.y + bounds.height) as i32 - 1).min(y + 1)) }
	}

	fn downright(&self, x: i32, y: i32, bounds: &AABB) -> Tile {
		let splits = self.splits.lock().unwrap();
		if splits.contains(&TileSplit { x: x + 1, y: y + 1, vertical: false }) || splits.contains(&TileSplit { x: x + 1, y: y + 1, vertical: true })
		{ Tile::Blocked }
		else { self.get(((bounds.x + bounds.width) as i32 - 1).min(x + 1), ((bounds.y + bounds.height) as i32 - 1).min(y + 1)) }
	}

	fn create_any(&self, command_sender: &mut dyn CommandOutput, bounds: AABB) {
		for (x, y) in self.tiles.keys() {
			for i in 0..16 {
				for j in 0..16 {
					if self.get(x + i, y + j) != Tile::None {
						self.update_tile(command_sender, x + i, y + j, &bounds, None);
					}
				}
			}
		}
	}

	pub fn update_tile(&self, command_sender: &mut dyn CommandOutput, x: i32, y: i32, bounds: &AABB, tile: Option<Tile>) {
		let tile = tile.unwrap_or_else(|| self.get(x, y));
		let theme = LevelTheme::parse(self.theme.load(Ordering::Relaxed)).unwrap();
		let mut ts = match theme {
			LevelTheme::Entrance |
			LevelTheme::DreamScraps => 12.0,
			LevelTheme::TheDepths => 25.0,
			LevelTheme::JerryAttack => 136.0,
			LevelTheme::FarawayLabyrinth => 23.0,
			LevelTheme::DancingStars |
			LevelTheme::ReachOutToThatMoon |
			LevelTheme::Seija => 14.0,
			LevelTheme::MindBreak => 16.0,
			LevelTheme::Fireflies => 13.0,
			LevelTheme::Cirno => 124.0,
			LevelTheme::Rumia => 122.0,
			LevelTheme::Rasobi => 19.0,
			LevelTheme::Purple => 130.0,
			_ => (17 + theme as i32) as f32
		};

		match tile.base() {
			Tile::GroundBase => {
				let left = self.left(x, y, bounds).ground_themed(theme);
				let right = self.right(x, y, bounds).ground_themed(theme);
				let up = self.up(x, y, bounds).ground_themed(theme);
				let mut down = self.down(x, y, bounds).ground_themed(theme);
				let transparent_corners = match theme {
					LevelTheme::DancingStars |
					LevelTheme::ReachOutToThatMoon |
					LevelTheme::Fireflies |
					//LevelTheme::Cirno |
					LevelTheme::Seija |
					LevelTheme::Entrance |
					LevelTheme::Purple |
					LevelTheme::DreamScraps => true,
					_ => (theme as i32) < 5
				};

				if !down && theme == LevelTheme::Rumia && self.down(x, y, bounds) != Tile::Blocked {
					down = true;
					if bounds.width == 200000.0 {
						let mut markers = self.marker_objects.lock().unwrap();
	
						markers.entry((x + 100000, y)).or_insert_with(|| {
							let mut go = GameObject::new(game_object::OBJ_BLANK, 15000);
							go.x = (x * 32) as f32;
							go.y = ((y + 1) * 32) as f32;
							go.create(command_sender);
							go.set_sprite(command_sender, sprite::RUMIA_DANGLE);
							go
						});
					} else {
						command_sender.send(Command::F32(vec![x as f32, (y + 1) as f32, ts, 0.0, 192.0, 32.0, 32.0, 15000.0]));
						command_sender.send(Command::AddTile);
					}
				}

				if !up && tile == Tile::GroundV8 && theme == LevelTheme::AzureWinter {
					if bounds.width == 200000.0 {
						self.marker_objects.lock().unwrap().entry((x + 100000, y)).or_insert_with(|| {
							let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 15090);
							go.x = (x * 32) as f32;
							go.y = (y * 32 - 64) as f32;
							go.create(command_sender);
							go.set_sprite(command_sender, sprite::LANTERN);
							go
						});
					}

					else {
						let mut go = GameObject::new(game_object::OBJ_LANTERN, 15090);
						go.x = (x * 32) as f32;
						go.y = (y * 32 - 64) as f32;
						go.create(command_sender);
					}
				}

				else if !down && tile != Tile::GroundBase && theme == LevelTheme::ShiningNeedleCastle {
					if bounds.width == 200000.0 {
						self.marker_objects.lock().unwrap().entry((x + 100000, y)).or_insert_with(|| {
							let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 15090);
							go.x = (x * 32) as f32;
							go.y = (y * 32 + 32) as f32;
							go.create(command_sender);
							go.set_sprite(command_sender, sprite::LANTERNK);
							go
						});
					}

					else {
						let mut go = GameObject::new(game_object::OBJ_LANTERNK, 15090);
						go.x = (x * 32) as f32;
						go.y = (y * 32 + 32) as f32;
						go.create(command_sender);
					}
				}

				self.ground(
					command_sender,
					left, right, up, down,
					theme,
					transparent_corners,
					tile,
					x, y,
					ts,
					&bounds,
					(0.0, 0.0)
				);
			}

			Tile::BGCommon1 => {
				let (tleft, ttop) = match theme {
					LevelTheme::FarawayLabyrinth |
					LevelTheme::Rasobi |
					LevelTheme::Purple |
					LevelTheme::AzureWinter => {
						let left = self.left(x, y, bounds).bg_or_ground();
						let right = self.right(x, y, bounds).bg_or_ground();

						if left && !right {
							(384.0, (288 + tile.variant() % 3 * 32) as f32)
						}

						else if !left && right {
							(320.0, (288 + tile.variant() % 3 * 32) as f32)
						}

						else if tile.variant() > 2 {
							(352.0, (288 + tile.variant() % 3 * 32) as f32)
						}

						else {
							bg_tile_pos(LevelTheme::AzureWinter)
						}
					}
					LevelTheme::UltramarineRain => {
						match self.ultramarine_rain_tile(x, y, bounds) {
							Some(tpos) => tpos,
							None => {
								let tul = self.ultramarine_rain_tile(x - 1, y - 1, bounds);
								let tur = self.ultramarine_rain_tile(x + 1, y - 1, bounds);
								let tdl = self.ultramarine_rain_tile(x - 1, y + 1, bounds);
								let tdr = self.ultramarine_rain_tile(x + 1, y + 1, bounds);

								let ul = tur == Some((224.0, 192.0)) || tdl == Some((224.0, 192.0));
								let ur = tul == Some((192.0, 192.0)) || tdr == Some((192.0, 192.0));
								let dl = tdr == Some((224.0, 160.0)) || tul == Some((224.0, 160.0));
								let dr = tdl == Some((192.0, 160.0)) || tur == Some((192.0, 160.0));

								match (ul, ur, dl, dr) {
									(true, false, false, false) => (224.0, 192.0),
									(false, true, false, false) => (192.0, 192.0),
									(false, false, true, false) => (224.0, 160.0),
									(false, false, false, true) => (192.0, 160.0),
									_ => match tile {
										Tile::BGCommon1 |
										Tile::BGCommon2 |
										Tile::BGCommon3 => bg_tile_pos(theme),
										Tile::BGRare13 => (288.0, 64.0),
										Tile::BGRare14 => (288.0, 96.0),
										Tile::BGRare15 => (192.0, 160.0),
										Tile::BGRare16 => (192.0, 192.0),
										Tile::BGRare17 => (224.0, 160.0),
										Tile::BGRare18 => (224.0, 192.0),
										_ => (
											(192 + tile.variant() % 3 * 32) as f32,
											(tile.variant() / 3 * 32) as f32
										)
									}
								}
							}
						}
					}
					LevelTheme::OutsideWorld => {
						let up = self.get(x, y - 1).bg();
						let down = self.down(x, y, bounds).ground_any();
						if !up && down {
							let left = self.left(x, y, bounds).bg_or_ground();
							let right = self.right(x, y, bounds).bg_or_ground();
							(
								match (left, right) {
									(false, true) => 256.0,
									(true, false) => 320.0,
									_ => 288.0
								},
								192.0
							)
						} else {
							bg_tile_pos(theme)
						}
					}
					LevelTheme::Koumakan => {
						if tile >= Tile::BGRare1 && tile <= Tile::BGRare7 {
							if bounds.width == 200000.0 {
								self.marker_objects.lock().unwrap().entry((x, y)).or_insert_with(|| {
									let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 15090);
									go.x = (x * 32) as f32;
									go.y = (y * 32) as f32;
									go.create(command_sender);
									go.set_sprite(command_sender, sprite::CANDLE);
									go
								});
							}

							else {
								let mut go = GameObject::new(game_object::OBJ_CANDLE, 15090);
								go.x = (x * 32) as f32;
								go.y = (y * 32) as f32;
								go.create(command_sender);
							}
						}

						bg_tile_pos(theme)
					}
					LevelTheme::JerryAttack => {
						ts = -1.0;
						if bounds.width == 200000.0 {
							self.marker_objects.lock().unwrap().entry((x, y)).or_insert_with(|| {
								let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 15099);
								go.x = (x * 32) as f32;
								go.y = (y * 32) as f32;
								go.create(command_sender);
								go.set_sprite(command_sender, sprite::JERRY);
								go
							});
						}

						else {
							let mut go = GameObject::new(game_object::OBJ_JERRY, 15099);
							go.x = (x * 32) as f32;
							go.y = (y * 32) as f32;
							go.create(command_sender);
						}

						(0.0, 0.0)
					}
					LevelTheme::Cirno => {
						let left = self.left(x, y, bounds).bg_or_alt_ground();
						let right = self.right(x, y, bounds).bg_or_alt_ground();
						let up = self.up(x, y, bounds).bg_or_ground();
						let down = self.down(x, y, bounds).bg_or_ground();

						if bounds.width != 200000.0 && !up {
							let mut go = GameObject::new(game_object::OBJ_FLOORDUMMY, 0);
							go.x = (x * 32) as f32;
							go.y = (y * 32) as f32;
							go.create(command_sender);
						}

						match (left, right, up, down) {
							(false, false, false, false) => (0.0, 192.0),
							(false, false, false, true ) => (0.0, 256.0),
							(false, false, true , false) => (0.0, 320.0),
							(false, false, true , true ) => (0.0, 288.0),
							(false, true , false, false) => (64.0, 192.0),
							(false, true , false, true ) => (64.0, 256.0),
							(false, true , true , false) => (64.0, 320.0),
							(false, true , true , true ) => (64.0, 288.0),
							(true , false, false, false) => (128.0, 192.0),
							(true , false, false, true ) => (128.0, 256.0),
							(true , false, true , false) => (128.0, 320.0),
							(true , false, true , true ) => (128.0, 288.0),
							(true , true , false, false) => (96.0, 192.0),
							(true , true , false, true ) => (96.0, 256.0),
							(true , true , true , false) => (96.0, 320.0),
							(true , true , true , true ) => (96.0, 288.0)
						}
					}
					_ => bg_tile_pos(theme)
				};
				command_sender.send(Command::F32(vec![x as f32, y as f32, ts, tleft, ttop, 32.0, 32.0, 15100.0]));
			}

			Tile::AltGroundBase => match theme {
				LevelTheme::DreamFields |
				LevelTheme::BambooForest |
				LevelTheme::UltramarineRain |
				LevelTheme::Rumia |
				LevelTheme::Seija |
				LevelTheme::MindBreak |
				LevelTheme::DancingStars |
				LevelTheme::ReachOutToThatMoon |
				LevelTheme::Cirno |
				LevelTheme::DreamScraps |
				LevelTheme::Fireflies |
				LevelTheme::Entrance |
				LevelTheme::TheDepths => {
					if bounds.width != 200000.0 {
						let left = self.left(x, y, bounds).ground_any();
						let right = self.right(x, y, bounds).ground_any();
						let up = self.up(x, y, bounds).ground_any();
						let down = self.down(x, y, bounds).ground_any();

						let (tleft, ttop) = match (left, right, up, down) {
							(false, false, false, false) => (160.0, 192.0),
							(false, false, false, true ) => (160.0, 224.0),
							(false, false, true , false) => (160.0, 288.0),
							(false, false, true , true ) => (160.0, 256.0),
							(false, true , false, false) => (192.0, 192.0),
							(false, true , false, true ) => ( 64.0, 192.0),
							(false, true , true , false) => ( 64.0, 256.0),
							(false, true , true , true ) => ( 64.0, 224.0),
							(true , false, false, false) => (256.0, 192.0),
							(true , false, false, true ) => (128.0, 192.0),
							(true , false, true , false) => (128.0, 256.0),
							(true , false, true , true ) => (128.0, 224.0),
							(true , true , false, false) => (224.0, 192.0),
							(true , true , false, true ) => ( 96.0, 192.0),
							(true , true , true , false) => ( 96.0, 256.0),
							(true , true , true , true ) => (1.0, 0.0)
						};

						if tleft != 1.0 {
							command_sender.send(Command::F32(vec![x as f32, y as f32, 18.0, tleft, ttop, 32.0, 32.0, 14999.0]));
							command_sender.send(Command::AddTile);
						}
					}

					let v = tile.variant();
					command_sender.send(Command::F32(vec![x as f32, y as f32, if theme == LevelTheme::BambooForest
						{ ts } else { bg::TILES }, (32 * (v & 1)) as f32, match theme {
							LevelTheme::DreamFields => 0.0,
							LevelTheme::Cirno |
							LevelTheme::UltramarineRain => 64.0,
							LevelTheme::TheDepths => 128.0,
							LevelTheme::MindBreak |
							LevelTheme::Rumia => 256.0,
							LevelTheme::Fireflies => 320.0,
							_ => 192.0
						} + (16 * (v & 2)) as f32, 32.0, 32.0, 15000.0]));
				}

				LevelTheme::AzureWinter |
				LevelTheme::Purple |
				LevelTheme::OutsideWorld |
				LevelTheme::ForestOfMagic |
				LevelTheme::JerryAttack => {
					let left = self.left(x, y, bounds).ground_alt();//_themed(theme);
					let right = self.right(x, y, bounds).ground_alt();//_themed(theme);
					let up = self.up(x, y, bounds).ground_alt();//_themed(theme);
					let down = self.down(x, y, bounds).ground_alt();//_themed(theme);

					self.ground(
						command_sender,
						left, right, up, down,
						LevelTheme::DreamFields,
						true,
						Tile::GroundBase,
						x, y,
						match theme {
							LevelTheme::JerryAttack => 131.0,
							LevelTheme::ForestOfMagic => 79.0,
							_ => ts
						},
						bounds,
						match theme {
							LevelTheme::JerryAttack |
							LevelTheme::AzureWinter |
							LevelTheme::ForestOfMagic |
							LevelTheme::Purple => (0.0, 0.0),
							LevelTheme::OutsideWorld => if tile.variant() == 0 { (192.0, 224.0) } else { (0.0, 224.0) },
							_ => unreachable!()
						}
					);
				}

				LevelTheme::Koumakan => {
					if self.sdm_pillar(x, y, bounds) {
						let up = self.sdm_pillar(x, y - 1, bounds);
						let down = self.sdm_pillar(x, y + 1, bounds);
						command_sender.send(Command::F32(vec![x as f32, y as f32, ts, 0.0, match (up, down) {
							(true, false) => 288.0,
							(false, true) => 224.0,
							_ => 256.0
						}, 32.0, 32.0, 15000.0]));
					}

					else {
						let left = self.left(x, y, bounds).ground_alt();
						let right = self.right(x, y, bounds).ground_alt();

						if !left && !right {
							command_sender.send(Command::F32(vec![x as f32, y as f32, bg::TILES, 64.0, 0.0, 32.0, 32.0, 15000.0]));
						}

						else {
							command_sender.send(Command::F32(vec![x as f32, y as f32, ts, match (left, right) {
								(true, true) => 32.0,
								(false, true) => 0.0,
								(true, false) => 64.0,
								(false, false) => unreachable!()
							}, 160.0, 32.0, 32.0, 15000.0]));
						}
					}
				}

				LevelTheme::ShiningNeedleCastle => {
					let left = self.left(x, y, bounds).ground_alt();
					let right = self.right(x, y, bounds).ground_alt();
					let up = self.up(x, y, bounds).ground_alt();
					let down = self.down(x, y, bounds).ground_alt();

					self.ground(
						command_sender,
						left, right, up, down,
						LevelTheme::DreamFields,
						false,
						Tile::GroundBase,
						x, y,
						ts,
						bounds,
						(0.0, if up || down { 128.0 } else { 0.0 })
					);
				}

				_ => {
					if bounds.width == 200000.0 {
						let mut markers = self.marker_objects.lock().unwrap();
	
						markers.entry((x, y)).or_insert_with(|| {
							let mut go = GameObject::new(game_object::OBJ_BLANK, 15000);
							go.x = (x * 32) as f32;
							go.y = (y * 32) as f32;
							go.create(command_sender);
							go.set_sprite(command_sender, tile.sprite(theme));
							go
						});
					}

					command_sender.send(Command::F32(vec![x as f32, y as f32, -1.0]));
				}
			}

			Tile::Water => {
				if bounds.width == 200000.0 {
					let mut markers = self.marker_objects.lock().unwrap();

					markers.entry((x, y)).or_insert_with(|| {
						let mut go = GameObject::new(game_object::OBJ_NO_ANIM, 15095);
						go.x = (x * 32) as f32;
						go.y = (y * 32) as f32;
						go.create(command_sender);
						go.set_sprite(command_sender, sprite::WATER);
						go
					});
				} else {
					let mut go = GameObject::new(game_object::OBJ_WATER, 15095);
					go.x = (x * 32) as f32;
					go.y = (y * 32) as f32;
					go.create(command_sender);
					go.destroy_server_only();
				}

				command_sender.send(Command::F32(vec![x as f32, y as f32, -1.0]));
			}

			Tile::BlockRed |
			Tile::BlockBlue |
			Tile::BlockPurple |
			Tile::BlockGreen => {
				command_sender.send(Command::F32(vec![x as f32, y as f32, bg::F_TILE,
					(32 * (tile as u8 - Tile::BlockRed as u8)) as f32, 416.0, 32.0, 32.0, 15000.0]));
			}

			Tile::None => command_sender.send(Command::F32(vec![x as f32, y as f32, -1.0])),

			_ => unreachable!()
		}

		command_sender.send(Command::SetTile);
	}

	fn ground(
		&self,
		command_sender: &mut dyn CommandOutput,
		left: bool, right: bool, up: bool, down: bool,
		theme: LevelTheme,
		transparent_corners: bool,
		tile: Tile,
		x: i32, y: i32,
		mut ts: f32,
		bounds: &AABB,
		offset: (f32, f32)
	) {
		let (mut tleft, mut ttop, transparency) = match (left, right, up, down) {
			(false, false, false, false) => (0.0, 0.0, transparent_corners || theme == LevelTheme::JerryAttack),
			(false, false, false, true ) => (0.0, 64.0, transparent_corners || theme == LevelTheme::Rumia),
			(false, false, true , false) => (0.0, 128.0, transparent_corners || theme == LevelTheme::JerryAttack),
			(false, false, true , true ) => (0.0, 96.0, false),
			(false, true , false, false) => (64.0, 0.0, transparent_corners || theme == LevelTheme::JerryAttack),
			(false, true , false, true ) => (64.0, 64.0, transparent_corners || theme == LevelTheme::Rumia),
			(false, true , true , false) => (64.0, 128.0, transparent_corners || theme == LevelTheme::JerryAttack),
			(false, true , true , true ) => (64.0, 96.0, false),
			(true , false, false, false) => (128.0, 0.0, transparent_corners || theme == LevelTheme::JerryAttack),
			(true , false, false, true ) => (128.0, 64.0, transparent_corners || theme == LevelTheme::Rumia),
			(true , false, true , false) => (128.0, 128.0, transparent_corners || theme == LevelTheme::JerryAttack),
			(true , false, true , true ) => (128.0, 96.0, false),
			(true , true , false, false) => (96.0, 0.0, false),
			(true , true , false, true ) => (96.0, 64.0, false),
			(true , true , true , false) => (96.0, 128.0, false),
			(true , true , true , true ) => (96.0, 96.0, false)
		};

		match theme {
			LevelTheme::DreamFields => {
				tleft += offset.0;
				ttop += offset.1;
			}

			LevelTheme::Purple |
			LevelTheme::AzureWinter => {
				if up && down && left && right {
					tleft = (192 + tile.variant() % 3 * 32) as f32;
					ttop = (192 + tile.variant() / 3 * 32) as f32;
				}

				else {
					if tleft == 0.0 {
						tleft = 128.0;
					} else {
						tleft -= 64.0;
					}

					if ttop == 0.0 {
						// NEW TILES REQUIRED
						ttop += 224.0;
					} else {
						ttop += 160.0;
					}
				}
			}
			LevelTheme::OutsideWorld => {
				if tleft == 0.0 && ttop > 0.0 {
					tleft = 32.0;
				}
			}
			LevelTheme::Koumakan => {
				// AAAAAA
				if tleft == 0.0 {
					tleft = 224.0;
				} else {
					tleft += 64.0;
				}

				if ttop == 0.0 {
					ttop = 288.0;
				} else {
					ttop += 256.0;
				}

				if tleft == 160.0 && ttop == 352.0 {
					match (
						self.upleft(x, y, bounds).ground_regular(),
						self.upright(x, y, bounds).ground_regular(),
						self.downleft(x, y, bounds).ground_regular(),
						self.downright(x, y, bounds).ground_regular()
					) {
						(false, true, true, true) => { tleft = 128.0; ttop = 416.0; }
						(true, false, true, true) => { tleft = 160.0; ttop = 416.0; }
						(true, true, false, true) => { tleft = 128.0; ttop = 448.0; }
						(true, true, true, false) => { tleft = 160.0; ttop = 448.0; }
						_ => ()
					}
				}
			}
			LevelTheme::ShiningNeedleCastle => {
				if ttop == 0.0 {
					if tleft == 0.0 {
						ts = bg::TILES;
						tleft = 64.0;
						ttop = 32.0;
					} else {
						ttop = 32.0;
					}
				}

				else if ttop == 64.0 && tleft > 32.0 && (bounds.width == 200000.0 || !self.snc_thick_ground(x, y, bounds)) {
					ttop = 32.0;
					tleft += 96.0;
				}
			}
			LevelTheme::Rasobi |
			LevelTheme::FarawayLabyrinth => {
				ts = -1.0;
				if bounds.width == 200000.0 {
					let mut markers = self.marker_objects.lock().unwrap();

					markers.entry((x, y)).or_insert_with(|| {
						let mut go = GameObject::new(game_object::OBJ_BLANK, 15000);
						go.x = (x * 32) as f32;
						go.y = (y * 32) as f32;
						go.create(command_sender);
						go.set_sprite(command_sender, sprite::WALL);
						go
					});
				}
			}
			LevelTheme::TheDepths |
			LevelTheme::MindBreak => {
				// AAAAAA
				if tleft == 96.0 && ttop == 96.0 {
					match (
						self.upleft(x, y, bounds).ground_regular(),
						self.upright(x, y, bounds).ground_regular(),
						self.downleft(x, y, bounds).ground_regular(),
						self.downright(x, y, bounds).ground_regular()
					) {
						(false, true, true, true) => { tleft = 128.0; ttop = 416.0; }
						(true, false, true, true) => { tleft = 64.0; ttop = 416.0; }
						(true, true, false, true) => { tleft = 128.0; ttop = 352.0; }
						(true, true, true, false) => { tleft = 64.0; ttop = 352.0; }
						_ => ()
					}
				}
			}
			_ => ()
		}

		if transparency {
			let mut has_bg = false;

			for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
				let xo = self.get(x + dx, y).bg();
				let yo = self.get(x, y + dy).bg();

				if (xo && yo) || ((xo || yo) && theme != LevelTheme::Cirno && !self.get(x + dx, y + dy).bg_or_ground()) {
					has_bg = true;
					break;
				}
			}

			if bounds.width == 200000.0 {
				let mut markers = self.marker_objects.lock().unwrap();

				if has_bg {
					markers.entry((x, y)).or_insert_with(|| {
						let mut go = GameObject::new(game_object::OBJ_BLANK, 15100);
						go.x = (x * 32) as f32;
						go.y = (y * 32) as f32;
						go.create(command_sender);
						go.set_sprite(command_sender, if theme == LevelTheme::Cirno {sprite::BG_TILE_CIRNO_FULL}
							else {Tile::BGCommon1.sprite(theme)});
						go
					});
				}

				else {
					if let Some(mut obj) = markers.remove(&(x, y)) {
						obj.destroy(command_sender);
					}
				}
			}

			else if has_bg {
				let (left, top) = bg_tile_pos(theme);
				command_sender.send(Command::F32(vec![x as f32, y as f32, ts, left, top, 32.0, 32.0, 15100.0]));
				command_sender.send(Command::AddTile);
			}
		}

		command_sender.send(Command::F32(vec![x as f32, y as f32, ts, tleft, ttop, 32.0, 32.0, 15000.0]));
	}

	fn ultramarine_rain_tile(&self, x: i32, y: i32, bounds: &AABB) -> Option<(f32, f32)>{
		if !self.get(x, y).bg() {
			return None;
		}

		let left = self.left(x, y, bounds).bg_or_ground();
		let right = self.right(x, y, bounds).bg_or_ground();
		let up = self.up(x, y, bounds).bg_or_ground();
		let down = self.down(x, y, bounds).bg_or_ground();

		match (left, right, up, down) {
			(false, true, false, true) => Some((192.0, 160.0)),
			(false, true, true, false) => Some((192.0, 192.0)),
			(true, false, false, true) => Some((224.0, 160.0)),
			(true, false, true, false) => Some((224.0, 192.0)),
			_ => None
		}
	}

	fn sdm_pillar(&self, x: i32, y: i32, bounds: &AABB) -> bool {
		!self.left(x, y, bounds).ground_alt() &&
		!self.right(x, y, bounds).ground_alt() &&
		self.up(x, y, bounds).ground_any() &&
		self.down(x, y, bounds).ground_any()
	}

	pub fn set_and_update(&mut self, command_sender: &mut dyn CommandOutput, x: i32, y: i32, tile: Tile) {
		let bounds = AABB {
			x: -100000.0,
			y: -100000.0,
			width: 200000.0,
			height: 200000.0
		};

		if let Some(mut marker) = self.marker_objects.lock().unwrap().remove(&(x, y)) {
			marker.destroy(command_sender);
		}

		if let Some(mut marker) = self.marker_objects.lock().unwrap().remove(&(x + 100000, y)) {
			marker.destroy(command_sender);
		}
		
		self.set(x, y, tile);
		self.update_tile(command_sender, x, y, &bounds, None);
		self.update_tile(command_sender, x + 1, y, &bounds, None);
		self.update_tile(command_sender, x - 1, y, &bounds, None);
		self.update_tile(command_sender, x, y + 1, &bounds, None);
		self.update_tile(command_sender, x, y - 1, &bounds, None);
		self.update_tile(command_sender, x + 1, y + 1, &bounds, None);
		self.update_tile(command_sender, x - 1, y - 1, &bounds, None);
		self.update_tile(command_sender, x - 1, y + 1, &bounds, None);
		self.update_tile(command_sender, x + 1, y - 1, &bounds, None);

		self.deco.lock().unwrap().dirty = true;
	}

	fn load_splits(&self, level: &Level) {
		self.splits.lock().unwrap().extend(level.objects.iter().filter_map(|o| o.to_tile_split()));
	}

	fn clear_deco(&self, command_sender: &mut dyn CommandOutput) {
		for tile in mem::take(&mut self.deco.lock().unwrap().tiles) {
			tile.remove(command_sender);
		}
	}

	fn update_deco(&self, command_sender: &mut dyn CommandOutput) {
		let mut deco = self.deco.lock().unwrap();
		deco.dirty = false;

		let theme = LevelTheme::parse(self.theme.load(Ordering::Relaxed)).unwrap();

		if !ground_decoration::has(theme) {
			return;
		}

		let ts = match theme {
			LevelTheme::Entrance |
			LevelTheme::DreamScraps => 12.0,
			LevelTheme::TheDepths => 25.0,
			LevelTheme::DancingStars => 14.0,
			LevelTheme::Cirno => 19.0,
			LevelTheme::Purple => 130.0,
			_ => (17 + theme as i32) as f32
		};

		let tiles = self.calculate_deco_tiles(theme);

		for tile in deco.tiles.difference(&tiles) {
			tile.remove(command_sender);
		}

		for tile in tiles.difference(&deco.tiles) {
			tile.create(command_sender, ts);
		}

		deco.tiles = tiles;
	}

	fn calculate_deco_tiles(&self, theme: LevelTheme) -> HashSet<GMTileData> {
		let mut set = HashSet::new();

		for ((cx, cy), tiles) in &self.tiles {
			let cx = *cx;
			let cy = *cy;

			for (i, tile) in tiles.iter().enumerate() {
				if tile.ground_rare() {
					let x = cx + i as i32 % 16;
					let y = cy + i as i32 / 16;
					if !self.get(x, y - 1).solid() {
						self.fit_deco(tile.variant(), theme, x, y, &mut set, ground_decoration::get);
					}

					else if ground_decoration::has_down(theme) && !self.get(x, y + 1).solid() {
						self.fit_deco(tile.variant(), theme, x, y, &mut set, ground_decoration::down);
					}
				}
			}
		}

		set
	}

	fn fit_deco(&self, mut variant: u32, theme: LevelTheme, x: i32, y: i32, set: &mut HashSet<GMTileData>, getter: fn(LevelTheme, u32) -> SmallVec<[GMTileData; 1]>) {
		while variant > 0 {
			let mut deco = (getter)(theme, variant);

			for dt in deco.iter_mut() {
				dt.x += x;
				dt.y += y;
			}

			if !deco.iter().any(|dt| {
				for x in dt.x..dt.x + dt.width as i32 {
					for y in dt.y..dt.y + dt.height as i32 {
						if self.get(x, y).ground_any() {
							return true;
						}
					}
				}

				false
			}) {
				set.extend(deco);
				break;
			}

			variant -= 1;
		}
	}

	pub fn maybe_update_deco(&mut self, command_sender: &mut CommandSender) {
		if self.deco.get_mut().unwrap().dirty {
			self.update_deco(command_sender);
		}
	}

	fn snc_thick_ground(&self, x: i32, y: i32, bounds: &AABB) -> bool {
		let mut map = self.snc_thick_ground.lock().unwrap();
		match map.get(&(x, y)).cloned() {
			Some(v) => v,
			None => {
				let mut min_x = x - 1;
				let mut thick = true;

				loop {
					if !self.get(min_x, y).ground_regular() { break; }
					if self.up(min_x, y, bounds).ground_regular() { break; }
					if self.splits.lock().unwrap().contains(&TileSplit { x: min_x + 1, y, vertical: true }) { break; }
					if !self.down(min_x, y, bounds).ground_regular() {
						thick = false;
						break;
					}

					min_x -= 1;
				}

				let mut max_x = x + 1;

				loop {
					if !self.get(max_x, y).ground_regular() { break; }
					if self.up(max_x, y, bounds).ground_regular() { break; }
					if self.splits.lock().unwrap().contains(&TileSplit { x: max_x, y, vertical: true }) { break; }
					if !self.down(max_x, y, bounds).ground_regular() {
						thick = false;
						break;
					}

					max_x += 1;
				}

				if max_x - min_x > 2 {
					for x in min_x + 1..max_x {
						map.insert((x, y), thick);
					}
				}

				thick
			}
		}
	}

	fn solids(&self, splits: &HashSet<TileSplit>, bounds: AABB, local: bool) -> Vec<SolidData> {
		self.solids.clone().unwrap_or_else(|| {
			let mut solids = vec![];
			
			for (x, y) in self.tiles.keys() {
				let mut local_solids = vec![];

				let s = if local {
					&mut local_solids
				} else {
					&mut solids
				};

				for j in 0..16 {
					for i in 0..16 {
						if self.get(x + i, y + j).solid() {
							s.push(SolidData {
								x: (x + i) as i16,
								y: (y + j) as i16,
								width: 1,
								height: 1,
							});

							if y + j == bounds.y as i32 && !splits.contains(&TileSplit { x: x + i, y: y + j, vertical: false }) {
								s.push(SolidData {
									x: (x + i) as i16,
									y: (y + j - 256) as i16,
									width: 1,
									height: 256,
								});
							}
						}
					}
				}

				if local {
					compress_solids(s);
					solids.extend(local_solids);
				}
			}

			if !local {
				compress_solids(&mut solids);
			}			

			solids
		})
	}
}

fn bg_tile_pos(theme: LevelTheme) -> (f32, f32) {
	match theme {
		LevelTheme::AzureWinter |
		LevelTheme::Koumakan => (320.0, 256.0),
		LevelTheme::MindBreak |
		LevelTheme::TheDepths => (160.0, 224.0),
		_ => (192.0, 0.0)
	}
}

fn compress_solids(solids: &mut Vec<SolidData>) {
	//eprintln!("Solids at start: {}", solids.len());

	'l: loop {
		for i in 0..solids.len() {
			for j in 0..solids.len() {
				if i == j {
					continue;
				}

				let b = solids[j];
				let a = &mut solids[i];

				if a.y == b.y && a.height == b.height && a.x + a.width == b.x {
					a.width += b.width;
					solids.remove(j);
					continue 'l;
				}

				/*if a.x == b.x && a.width == b.width && a.y + a.height == b.y {
					a.height += b.height;
					solids.remove(j);
					continue 'l;
				}*/
			}
		}

		break;
	}

	//eprintln!("Solids at end: {}", solids.len());
}

#[cfg(feature = "verify")]
pub fn bake_solids(level: &mut Level) {
	level.tile_manager().load_splits(level);
	let bounds = level.bounding_box();
	let tm = level.tile_manager_mut();
	tm.solids = Some(tm.solids(&tm.splits.lock().unwrap(), bounds, false))
}