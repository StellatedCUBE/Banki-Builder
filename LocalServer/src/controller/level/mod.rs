use std::{collections::HashMap, io::{Cursor, Write}, mem, ops::{BitOr, BitOrAssign}, path::{Path, PathBuf}, ptr, sync::{Arc, Mutex}, u32, usize};
use self::{tile_manager::{TileManager, Tile}, simple_object::ObjectType};

use super::{bg, command_handler::{Command, CommandOutput, CommandSender}, font, fs, game_object::{self, GameObject}, internal_command::InternalCommand, loc::MaybeLocalized, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use super::{menu::{editor::{context_menu::ContextMenuItem, Editor}, error::ErrorMenu, play::{Play, PlayingFrom}}, net::{logged_in, query, user}, loc::data::LOC_NETWORK_ERR};
use banki_common::{download_level::DownloadLevelRQ, MOD_VERSION};
use bincode::{Decode, Encode};
use chandelier::Chandelier;
use extra_head::ExtraHead;
use flate2::{Compress, FlushCompress, Compression, Decompress, FlushDecompress};
use flipper::Flipper;
use mochi::Mochi;
use moving_platform::MovingPlatform;
use onmyoudama_crawl::OnmyoudamaCrawl;
use onmyoudama_shoot::OnmyoudamaShoot;
use simple_object::SimpleObject;
use symbol::SymbolObject;
use tag::Tag;
use teleporter::Teleporter;
use tile_split::TileSplit;
use tokio::io::{AsyncRead, AsyncReadExt};
use anyhow::{anyhow, Result};
use warp::Warp;

pub mod chandelier;
pub mod flipper;
pub mod extra_head;
pub mod mochi;
pub mod moving_platform;
pub mod onmyoudama_crawl;
pub mod onmyoudama_shoot;
pub mod simple_object;
pub mod symbol;
pub mod teleporter;
pub mod tile_manager;
pub mod tile_split;
pub mod warp;
pub mod metadata;
pub mod database;
pub mod ground_decoration;
pub mod tag;

pub const LEVEL_FILE_MAGIC_NUMBERS: [u8; 8] = [255, 26, 232, 155, 174, 229, 165, 135];

pub const L_TILE: u32 = 1 << 0;
pub const L_OBJECT: u32 = 1 << 1;
pub const L_BLOCK: u32 = 1 << 2;
pub const L_DECORATION: u32 = 1 << 3;
pub const L_IMMUTABLE_BLOCK_UPPER: u32 = 1 << 4;
pub const L_IMMUTABLE_BLOCK_LOWER: u32 = 1 << 5;
pub const L_PHYSICS_OBJECT: u32 = 1 << 6;
pub const L_RAIL_MOVER: u32 = 1 << 7;
pub const L_SYMBOL: u32 = 1 << 8;
pub const L_FLIPPER: u32 = 1 << 9;
pub const L_PHYSICS_OBJECT_FULL: u32 = 1 << 10;

pub const PL_IMMUTABLE_BLOCK: u32 = 
	L_BLOCK |
	L_OBJECT |
	L_IMMUTABLE_BLOCK_UPPER |
	L_IMMUTABLE_BLOCK_LOWER |
	L_PHYSICS_OBJECT;

#[cfg(feature = "verify")]
pub struct Editor;
	
#[derive(Clone, Copy, PartialEq)]
pub enum Connect {
	None, Up, Down
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum Character {
	Banki, Cirno, Rumia, Seija
}

impl Character {
	pub fn parse(num: u8) -> anyhow::Result<Self> {
		match num {
			0 => Ok(Self::Banki),
			1 => Ok(Self::Cirno),
			2 => Ok(Self::Rumia),
			3 => Ok(Self::Seija),
			x => Err(anyhow!(format!("Bad character byte: {}", x)))
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord)]
pub enum ObjectID {
	Object(usize),
	SubObject(usize, usize)
}

impl ObjectID {
	pub const fn id(&self) -> usize {
		match self {
			Self::Object(id) => *id,
			Self::SubObject(id, _) => *id
		}
	}
}

impl PartialOrd for ObjectID {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			(Self::SubObject(a, b), Self::SubObject(c, d)) => if *a == *c {
				d.partial_cmp(b)
			} else {
				other.id().partial_cmp(&self.id())
			}
			_ => other.id().partial_cmp(&self.id())
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ObjectKey(pub usize);

pub struct ObjectButton {
	pub game_object: GameObject,
	pub bounds: AABB,
	pub object: ObjectKey,
	pub callback: fn(&mut CommandSender, &mut Editor, usize) -> Vec<UndoAction>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct AABB {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

impl AABB {
	pub fn contains(&self, x: f32, y: f32) -> bool {
		x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
	}

	pub fn null() -> Self {
		AABB {
			x: 0.0, y: 0.0,
			width: 0.0, height: 0.0
		}
	}

	pub fn zero(&self) -> bool {
		self.width == 0.0 || self.height == 0.0
	}

	pub fn times(&self, by: f32) -> Self {
		Self {
			x: self.x * by,
			y: self.y * by,
			width: self.width * by,
			height: self.height * by
		}
	}

	pub fn intersects(&self, other: AABB) -> bool {
		!(
			self.x + self.width <= other.x ||
			self.y + self.height <= other.y ||
			other.x + other.width <= self.x ||
			other.y + other.height <= self.y
		)
	}

	pub fn shrink_if_tile(&mut self) {
		if self.width.floor() == self.width && self.width > 0.0625 {
			self.x += 0.0625;
			self.width -= 0.0625;
		}

		let r = self.width + self.x;

		if r == r.floor() && self.width > 0.09375 {
			self.width -= 0.09375;
		}

		if self.height.floor() == self.height && self.height > 0.0625 {
			self.y += 0.0625;
			self.height -= 0.0625;
		}

		let r = self.height + self.y;

		if r == r.floor() && self.height > 0.09375 {
			self.height -= 0.09375;
		}
	}

	pub fn expand_to_tile(&self) -> Self {
		let x = self.x.floor();
		let y = self.y.floor();
		let width = (self.width + (self.x - x)).ceil();
		let height = (self.height + (self.y - y)).ceil();

		AABB { x, y, width, height }
	}

	pub fn clamp_x(&self, x: i32) -> i32 {
		x.clamp(self.x as i32, (self.x + self.width) as i32 - 1)
	}

	pub fn clamp_y(&self, y: i32) -> i32 {
		y.clamp(self.y as i32, (self.y + self.height) as i32 - 1)
	}
}

impl BitOr for AABB {
	type Output = Self;
	fn bitor(self, other: Self) -> Self::Output {
		if self.zero() {
			return other;
		}

		if other.zero() {
			return self;
		}

		let x_max = (self.x + self.width).max(other.x + other.width);
		let y_max = (self.y + self.height).max(other.y + other.height);
		let x_min = self.x.min(other.x);
		let y_min = self.y.min(other.y);

		Self {
			x: x_min, y: y_min,
			width: x_max - x_min, height: y_max - y_min
		}
	}
}

impl BitOrAssign for AABB {
	fn bitor_assign(&mut self, other: Self) {
		if other.zero() {
			return;
		}

		if self.zero() {
			*self = other;
			return;
		}

		let x_max = (self.x + self.width).max(other.x + other.width);
		let y_max = (self.y + self.height).max(other.y + other.height);
		self.x = self.x.min(other.x);
		self.y = self.y.min(other.y);
		self.width = x_max - self.x;
		self.height = y_max - self.y;
	}
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Encode, Decode)]
pub enum LevelTheme {
	DreamFields,
	BambooForest,
	AzureWinter,
	ForestOfMagic,
	UltramarineRain,
	OutsideWorld,
	Koumakan,
	ShiningNeedleCastle,
	DreamScraps,
	TheDepths,
	JerryAttack,
	FarawayLabyrinth,
	DancingStars,
	ReachOutToThatMoon,
	MindBreak,
	Fireflies,
	Cirno,
	Rumia,
	Seija,
	Rasobi,
	Purple,
	Entrance,
}

impl LevelTheme {
	const MAX: Self = Self::Entrance;

	pub fn backgrounds(self) -> Vec<f32> {
		match self {
			Self::DreamFields => vec![50.1, 54.1, 53.1, 52.1, 51.1, 55.0],
			Self::BambooForest => vec![bg::BG_STAR_COMMON, 61.1, 60.1, 59.0, 58.0, 57.0],
			Self::AzureWinter => vec![bg::BG_STAR_WHITE, 62.1, 70.1, 69.1, 68.1, 65.1, 64.1, 63.1],
			Self::ForestOfMagic => vec![90.1, 89.1, 88.1, 87.1, 86.0, 85.1, 84.1, bg::FOM_TOP + 0.1],
			Self::UltramarineRain => vec![bg::BG_STAR_COMMON, 91.1, 97.1, 96.1],
			Self::OutsideWorld => vec![bg::BG_STAR_COMMON, 102.1, 103.1, 104.1, 105.1],
			Self::Koumakan => vec![108.0, 107.0, 106.0],
			Self::ShiningNeedleCastle => vec![bg::BG_STAR_COMMON, 113.1, 114.1, 115.1, 116.0],
			Self::DreamScraps => vec![117.0, 120.0, 119.0, 118.0],
			Self::TheDepths => vec![30.0, 0.0, 31.0, 32.0, 33.0],
			Self::JerryAttack => vec![bg::BG_STAR_WHITE, 132.1, 135.1, 134.1, 133.1],
			Self::FarawayLabyrinth => vec![bg::BG_STAR_RED, 43.1, 44.0],
			Self::DancingStars => vec![bg::BG_STAR_COMMON, 45.1],
			Self::ReachOutToThatMoon => vec![bg::BG_STAR_COMMON, 49.1, 0.0, 48.0, 47.0, 46.0],
			Self::MindBreak => vec![bg::BG_STAR_RED, 43.1],
			Self::Fireflies => vec![bg::BG_STAR_COMMON, 49.1],
			Self::Rasobi |
			Self::Cirno => vec![bg::BG_STAR_WHITE, 62.1],
			Self::Rumia => vec![bg::BG_STAR_WHITE, 121.1],
			Self::Seija => vec![129.0, 128.0, 127.0],
			Self::Purple => vec![bg::BG_STAR_PURPLE, 126.1],
			Self::Entrance => vec![26.0, 27.0, 34.0, 35.0, 36.0],
		}
	}

	pub fn object(self) -> u32 {
		match self {
			Self::Seija => game_object::OBJ_BGSEIJA,
			Self::TheDepths |
			Self::Entrance => game_object::OBJ_BGENTRANCE,
			Self::DreamScraps |
			Self::Koumakan => game_object::OBJ_BG1 + self as u32,
			_ => game_object::OBJ_XBG
		}
	}

	pub const fn bgm(self, second: bool) -> u32 {
		match self {
			Self::JerryAttack => 121,
			Self::FarawayLabyrinth => 115,
			Self::Rasobi |
			Self::DancingStars => 116,
			Self::ReachOutToThatMoon => 117,
			Self::MindBreak => 118,
			Self::Fireflies => 122,
			Self::Entrance => if second {82} else {91},
			Self::Cirno => 90,
			Self::Rumia |
			Self::Seija => 89,
			Self::Purple => if second {87} else {72},
			Self::OutsideWorld => if second {106} else {104},
			_ => 2 * self as u32 + if second {1} else {0} + if (self as u32) < Self::OutsideWorld as u32 {94} else {95}
		}
	}

	pub fn precipitation(self) -> PrecipitationOverride {
		match self {
			Self::Cirno |
			Self::AzureWinter => PrecipitationOverride::Snow,
			Self::ForestOfMagic => PrecipitationOverride::Happa,
			Self::UltramarineRain => PrecipitationOverride::Rain,
			Self::DancingStars => PrecipitationOverride::Stars,
			_ => PrecipitationOverride::Nothing
		}
	}

	pub fn next(self) -> Self {
		match self {
			Self::MAX => Self::DreamFields,
			_ => Self::parse(self as u8 + 1).unwrap()
		}
	}

	pub fn prev(self) -> Self {
		match self {
			Self::DreamFields => Self::MAX,
			_ => Self::parse(self as u8 - 1).unwrap()
		}
	}

	pub const fn bg_colour(self) -> f32 {
		match self {
			Self::DreamFields => 0xffd19c as f32,
			Self::BambooForest => 0x424332 as f32,
			Self::Cirno |
			Self::Rasobi |
			Self::AzureWinter => 0xffd5a6 as f32,
			Self::ForestOfMagic => 0x8b957c as f32,
			Self::UltramarineRain => 0x74563b as f32,
			Self::OutsideWorld => 0x3e1a0d as f32,
			Self::Koumakan => 0x31338a as f32,
			Self::ShiningNeedleCastle => 0x743e3c as f32,
			Self::DreamScraps => 0xfc6c97 as f32,
			Self::TheDepths => 0xcb4881 as f32,
			Self::JerryAttack => 0xaf8dff as f32,
			Self::FarawayLabyrinth => 0x090a15 as f32,
			Self::DancingStars => 0xf0d0d as f32,
			Self::ReachOutToThatMoon => 0x4e271e as f32,
			Self::MindBreak => 0x974 as f32,
			Self::Fireflies => 0x4e271e as f32,
			Self::Rumia => 0x606060 as f32,
			Self::Seija => 0xf3cdac as f32,
			Self::Purple => 0x7b1d4b as f32,
			Self::Entrance => 0x571b30 as f32,
		}
	}

	pub fn parse(id: u8) -> Result<Self> {
		if id <= Self::MAX as u8 {
			Ok(unsafe { mem::transmute(id) })
		} else {
			Err(anyhow!("Bad theme data {id}"))
		}
	}
}

pub enum SubObjectDeleteUndoAction {
	None,
	Some(UndoAction),
	DeleteMain
}

pub trait LevelObject {
	fn create(&self, command_sender: &mut dyn CommandOutput, level: &Level, return_object: bool) -> GameObject;

	#[cfg(not(feature = "verify"))]
	fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject>;
	#[cfg(not(feature = "verify"))]
	fn destroy_editor_view(&self, command_sender: &mut dyn CommandOutput, objects: &mut Vec<GameObject>, #[allow(unused_variables)] level: &Level) {
		for object in objects {
			object.destroy(command_sender);
		}
	}

	fn sub_object_count(&self) -> usize {0}

	fn bounding_box(&self) -> AABB;
	fn sub_object_bounding_box(&self, #[allow(unused_variables)] sub_object: usize) -> AABB {
		unreachable!()
	}

	fn move_by(&mut self, x: i32, y: i32);
	fn move_sub_object_by(&mut self, #[allow(unused_variables)] sub_object: usize, #[allow(unused_variables)] x: i32, #[allow(unused_variables)] y: i32) {
		unreachable!()
	}
	fn ghost_sub_object(
		&self,
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] sub_object: usize,
		#[allow(unused_variables)] objects: &mut Vec<GameObject>
	) -> ((i32, i32), i32) {
		unreachable!()
	}

	fn can_delete(&self) -> bool {
		true
	}

	fn to_tile_manager(&self) -> &TileManager {
		unreachable!()
	}

	fn to_tile_manager_mut(&mut self) -> &mut TileManager {
		unreachable!()
	}

	fn to_simple_object(&self) -> &SimpleObject {
		unreachable!()
	}

	fn to_chain(&mut self) -> &mut MovingPlatform {
		unreachable!()
	}

	fn to_tile_split(&self) -> Option<&TileSplit> { None }

	fn to_simple_object_mut(&mut self) -> &mut SimpleObject {
		unreachable!()
	}

	fn simple_object_type(&self) -> Option<ObjectType> { None }

	fn delete_sub_object(
		&mut self,
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] objects: &mut Vec<GameObject>,
		#[allow(unused_variables)] object: usize,
		#[allow(unused_variables)] sub_object: usize
	) -> SubObjectDeleteUndoAction {SubObjectDeleteUndoAction::None}

	fn types(&self) -> u32;
	fn sub_object_types(&self, #[allow(unused_variables)] sub_object: usize) -> u32 {
		unreachable!()
	}

	fn serialized_type(&self) -> u8;
	fn serialize_inner(&self, to: &mut dyn Write) -> anyhow::Result<()>;

	#[cfg(not(feature = "verify"))]
	fn context_menu_items(&self, #[allow(unused_variables)] theme: LevelTheme) -> Vec<ContextMenuItem> {vec![]}
	#[cfg(not(feature = "verify"))]
	fn sub_object_context_menu_items(
		&self,
		#[allow(unused_variables)] sub_object: usize,
		#[allow(unused_variables)] theme: LevelTheme
	) -> Vec<ContextMenuItem> {vec![]}
	fn handle_context_menu_action(
		&mut self,
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] object: usize,
		#[allow(unused_variables)] action: i32,
		#[allow(unused_variables)] theme: LevelTheme
	) -> Vec<UndoAction> {vec![]}
	fn handle_sub_object_context_menu_action(
		&mut self,
		#[allow(unused_variables)] command_sender: &mut CommandSender,
		#[allow(unused_variables)] object: usize,
		#[allow(unused_variables)] sub_object: usize,
		#[allow(unused_variables)] action: i32,
		#[allow(unused_variables)] theme: LevelTheme
	) -> (bool, Vec<UndoAction>) {(false,vec![])}

	fn save_ids(&self, #[allow(unused_variables)] level: &Level) -> Vec<usize> {vec![]}
	fn post_create(
		&self,
		#[allow(unused_variables)] command_sender: &mut dyn CommandOutput,
		#[allow(unused_variables)] level: &Level,
		#[allow(unused_variables)] self_object: &GameObject,
		#[allow(unused_variables)] saved_objects: Vec<&GameObject>
	) {}

	fn connect_directions(&self) -> Connect {Connect::None}
	fn recreate_on_character_change(&self) -> bool {false}

	fn hax(&self) -> bool {false}
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum PrecipitationOverride {
	None,
	Nothing,
	Snow,
	Happa,
	Rain,
	Stars,
	Blizzard
}

impl PrecipitationOverride {
	pub fn parse(id: u8) -> Result<Self> {
		if id < 7 {
			Ok(unsafe { mem::transmute(id) })
		} else {
			Err(anyhow!("Bad precipitation data {id}"))
		}
	}
}

static CONNECTED: Mutex<Vec<bool>> = Mutex::new(vec![]);

pub struct Level {
	pub name: String,
	pub author: u64,
	pub theme: LevelTheme,
	pub music: u32,
	pub tags: u32,
	pub precipitation_override: PrecipitationOverride,
	pub objects: Vec<Box<dyn LevelObject + Send>>,
	pub heads: u32,
	pub seija_abilities: u8,
	pub filepath: Option<PathBuf>,
	pub online_id: u32,
	object_key_counter: usize,
	pub object_keys: Vec<ObjectKey>,
	pub speedrun_techniques: bool,
}

impl Level {
	pub fn is_name_valid(name: &str) -> bool {
		let len = name.chars().count();
		len > 0 && len < 128 && name.chars().all(font::contains) && name.len() == name.trim().len()
	}

	pub fn new(name: String) -> Arc<Mutex<Self>> {
		Arc::new(Mutex::new(Self {
			name,
			author: 0,
			theme: LevelTheme::DreamFields,
			music: LevelTheme::DreamFields.bgm(false),
			tags: 0,
			precipitation_override: PrecipitationOverride::None,
			objects: vec![
				Box::from(TileManager::new()),
				Box::from(SimpleObject {
					object_type: ObjectType::Player(Character::Banki),
					x: 2,
					y: 7
				}),
				Box::from(SimpleObject {
					object_type: ObjectType::Goal,
					x: 4,
					y: 7
				}),
			],
			heads: 0,
			seija_abilities: 15,
			filepath: None,
			online_id: u32::MAX,
			object_key_counter: 3,
			object_keys: vec![ObjectKey(0), ObjectKey(1), ObjectKey(2)],
			speedrun_techniques: false,
		}))
	}

	pub fn create_bg(&self, command_sender: &mut dyn CommandOutput) -> GameObject {
		let mut bgs = self.theme.backgrounds();
		bgs.push(-1.0);
		command_sender.send(Command::F32(bgs));
		command_sender.send(Command::SetBackgrounds);

		let mut go = GameObject::new(self.theme.object(), 16380);
		go.create(command_sender);

		if go.object_type == game_object::OBJ_XBG {
			go.set_real(command_sender, 0, self.theme as u32 as f32);
		}

		go
	}

	pub fn tile_manager(&self) -> &TileManager {
		self.objects[0].to_tile_manager()
	}

	pub fn tile_manager_mut(&mut self) -> &mut TileManager {
		self.objects[0].to_tile_manager_mut()
	}

	pub fn character(&self) -> Character {
		if let ObjectType::Player(character) = self.objects[1].to_simple_object().object_type {
			character
		} else {
			unreachable!()
		}
	}

	pub fn object_index(&self, object: &(impl LevelObject + ?Sized)) -> usize {
		self.objects.iter()
			.enumerate()
			.find(|p| ptr::addr_eq(p.1.as_ref(), object))
			.and_then(|m| Some(m.0))
			.unwrap_or(self.objects.len())
	}

	pub fn object_key(&self, object: &(impl LevelObject + ?Sized)) -> ObjectKey {
		self.object_keys.get(self.object_index(object)).cloned().unwrap_or(ObjectKey(self.object_key_counter))
	}

	pub fn index_by_key(&self, key: ObjectKey) -> usize {
		self.object_keys.iter().position(|k| *k == key).unwrap()
	}

	#[cfg(not(feature = "verify"))]
	pub fn load(&self, command_sender: &mut dyn CommandOutput) {
		*CONNECTED.lock().unwrap() = vec![false; self.objects.len()];

		if self.music == LevelTheme::OutsideWorld.bgm(false) ||
			self.music == LevelTheme::OutsideWorld.bgm(true)
		{
			sound::set_bgm(command_sender, 0);
		} else {
			sound::set_bgm(command_sender, self.music);
		}

		let mut go = GameObject::new(game_object::OBJ_GAMEMGR, 0);
		go.create(command_sender);
		go.destroy_server_only();
		command_sender.send(Command::F32(vec![self.heads as f32, self.seija_abilities as f32]));
		command_sender.send(Command::SetLevelData(self.character()));

		match self.character() {
			Character::Rumia => {
				let mut go = GameObject::new(game_object::OBJ_RUMIAPREVIEW, -2000);
				go.create(command_sender);
				go.destroy_server_only();
			}
			Character::Seija => {
				let mut go = GameObject::new(game_object::OBJ_SEIJACAMERA, -2000);
				go.create(command_sender);
				go.destroy_server_only();
			}
			_ => ()
		}
		
		self.create_bg(command_sender).destroy_server_only();

		/*let mut to_save = HashMap::<_, SmallVec<[usize; 2]>>::new();

		for (i, object) in self.objects.iter().enumerate() {
			for id in object.save_ids(self) {
				to_save.entry(id).or_default().push(i);
			}
		}

		for (i, object) in self.objects.iter().enumerate() {
			object.create(command_sender, self, to_save.contains_key(i));
		}*/

		let mut save = vec![false; self.objects.len()];
		let mut wants = HashMap::new();

		for (i, object) in self.objects.iter().enumerate() {
			let idl = object.save_ids(self);
			if idl.len() > 0 {
				save[i] = true;
				for id in &idl {
					save[*id] = true;
				}
				wants.insert(i, idl);
			}
		}

		let game_objects: Vec<_> = self.objects.iter()
			.enumerate()
			.map(|p| p.1.create(command_sender, self, save[p.0]))
			.collect();

		for (i, idl) in wants {
			self.objects[i].post_create(
				command_sender,
				self,
				&game_objects[i],
				idl.into_iter().map(|id| &game_objects[id]).collect()
			);
		}

		let bb = self.bounding_box();

		let mut go = GameObject::new(game_object::OBJ_VIEWLEFT, 0);
		go.x = bb.x * 32.0 - 33.0;
		go.y = bb.y * 32.0;
		go.create(command_sender);
		go.set_scale(command_sender, 1.0, bb.height);
		go.destroy_server_only();

		let mut go = GameObject::new(game_object::OBJ_VIEWRIGHT, 0);
		go.x = (bb.x + bb.width) * 32.0;
		go.y = bb.y * 32.0;
		go.create(command_sender);
		go.set_scale(command_sender, 1.0, bb.height);
		go.destroy_server_only();

		let mut go = GameObject::new(game_object::OBJ_VIEWTOP, 0);
		go.x = bb.x * 32.0;
		go.y = bb.y * 32.0 - 33.0;
		go.create(command_sender);
		go.set_scale(command_sender, bb.width, 1.0);
		go.destroy_server_only();

		let mut go = GameObject::new(game_object::OBJ_VIEWBOTTOM, 0);
		go.x = bb.x * 32.0;
		go.y = (bb.y + bb.height) * 32.0;
		go.create(command_sender);
		go.set_scale(command_sender, bb.width, 1.0);
		go.destroy_server_only();

		let mut go = GameObject::new(game_object::OBJ_STAGESTARTMGR, -5000);
		go.create(command_sender);
		go.set_string(command_sender, 0, &user::get(self.author).name);
		go.set_string(command_sender, 1, &self.name);
		go.destroy_server_only();
		let mut go = GameObject::new(game_object::OBJ_QUICKRETRYMGR, 0);
		go.create(command_sender);
		go.destroy_server_only();
		let mut go = GameObject::new(game_object::OBJ_VIEWMODEMGR, 0);
		go.create(command_sender);
		go.destroy_server_only();

		let precipitation = match self.precipitation_override {
			PrecipitationOverride::None => self.theme.precipitation(),
			x => x
		};

		let mut go = GameObject::new(game_object::OBJ_PRECIPITATOR, 0);
		go.x = (bb.y + bb.height) * 32.0;
		go.y = bb.y * 32.0;

		let (rate, sprite, dx, dy) = match precipitation {
			PrecipitationOverride::Snow => (0.1, sprite::SNOW, 0.0, 0.7),
			PrecipitationOverride::Happa => (0.03, sprite::HAPPA, 0.1, 1.0),
			PrecipitationOverride::Rain => (0.9, sprite::RAIN, 1.8, 9.0),
			PrecipitationOverride::Blizzard => (1.0, sprite::SNOW, -5.0, 5.0),
			PrecipitationOverride::Stars => (0.25, sprite::STAR, 0.0, -0.7),
			_ => (0.0, 0.0, 0.0, 100000.0)
		};

		if dy < 0.0 {
			mem::swap(&mut go.x, &mut go.y);
		}

		go.create(command_sender);

		go.set_real(command_sender, 0, rate);
		go.set_real(command_sender, 1, sprite);
		go.set_real(command_sender, 2, dx);
		go.set_real(command_sender, 3, dy);
		go.set_real(command_sender, 4, bb.x * 32.0 - 32.0);
		go.set_real(command_sender, 5, (bb.x + bb.width + 1.0) * 32.0);

		go.destroy_server_only();

		if self.theme == LevelTheme::ForestOfMagic && (precipitation == PrecipitationOverride::Nothing || precipitation == PrecipitationOverride::Happa) {
			let mut go = GameObject::new(game_object::OBJ_KOMOREBI, 0);
			go.create(command_sender);
			go.destroy_server_only();
		}

		let mut go = GameObject::new(game_object::OBJ_WALL, 0);
		go.x = (bb.x - 1.0) * 32.0;
		go.y = (bb.y - 32.0) * 32.0;
		go.create(command_sender);
		go.set_scale(command_sender, 1.0, bb.height + 64.0);
		go.destroy_server_only();
		let mut go = GameObject::new(game_object::OBJ_WALL, 0);
		go.x = (bb.x + bb.width) * 32.0;
		go.y = (bb.y - 32.0) * 32.0;
		go.create(command_sender);
		go.set_scale(command_sender, 1.0, bb.height + 64.0);
		go.destroy_server_only();

		for i in 0..4 {
			let mut go = GameObject::new(i * 4 + 39, 0);
			go.x = 200000.0;
			go.create(command_sender);
			go.destroy_server_only();

			let mut go = GameObject::new(i * 4 + 40, 0);
			go.x = 200000.0;
			go.create(command_sender);
			go.destroy_server_only();
		}

		let mut go = GameObject::new(game_object::OBJ_WALLICE, 0);
		go.x = 200000.0;
		go.create(command_sender);
		go.destroy_server_only();

		let mut go = GameObject::new(game_object::OBJ_WALLICE_OFF, 0);
		go.x = 200000.0;
		go.create(command_sender);
		go.destroy_server_only();

		let mut go = GameObject::new(game_object::OBJ_WHITESWITCH, 0);
		go.x = 200000.0;
		go.create(command_sender);
		go.destroy_server_only();

		if self.music == LevelTheme::OutsideWorld.bgm(false) ||
			self.music == LevelTheme::OutsideWorld.bgm(true) ||
			self.objects.iter().any(|o| match o.simple_object_type() {
				Some(ObjectType::BlinkyBlock(_)) => true,
				_ => false
		}) {
			let mut go = GameObject::new(game_object::OBJ_TRACKPOSITIONMGR, 0);
			if self.music == LevelTheme::OutsideWorld.bgm(true) {
				go.x = 3.0;
			} else if self.music == LevelTheme::OutsideWorld.bgm(false) {
				go.x = 2.0;
			} else {
				go.x = 1.0;
			}
			go.create(command_sender);
			go.destroy_server_only();
		}
	}

	pub fn at(&self, x: i32, y: i32) -> Vec<ObjectID> {
		let mut ids = vec![];

		if self.tile_manager().get(x, y) != Tile::None {
			ids.push(ObjectID::SubObject(0, TileManager::to_sub_id(x, y)))
		}

		for i in 1..self.objects.len() {
			if self.objects[i].bounding_box().expand_to_tile().contains(x as f32 + 0.5, y as f32 + 0.5) {
				if self.objects[i].sub_object_count() == 0 {
					ids.push(ObjectID::Object(i));
				} else {
					for j in 0..self.objects[i].sub_object_count() {
						if self.objects[i].sub_object_bounding_box(j).expand_to_tile().contains(x as f32 + 0.5, y as f32 + 0.5) {
							ids.push(ObjectID::SubObject(i, j));
						}
					}
				}
			}
		}

		ids
	}

	pub fn atf(&self, x: f32, y: f32) -> Vec<ObjectID> {
		let mut ids = vec![];

		if self.tile_manager().get(x.floor() as i32, y.floor() as i32) != Tile::None {
			ids.push(ObjectID::SubObject(0, TileManager::to_sub_id(x.floor() as i32, y.floor() as i32)))
		}

		for i in 1..self.objects.len() {
			if self.objects[i].bounding_box().contains(x, y) {
				if self.objects[i].sub_object_count() == 0 {
					ids.push(ObjectID::Object(i));
				} else {
					for j in 0..self.objects[i].sub_object_count() {
						if self.objects[i].sub_object_bounding_box(j).contains(x, y) {
							ids.push(ObjectID::SubObject(i, j));
						}
					}
				}
			}
		}

		ids
	}

	pub fn any_type_match(&self, x: i32, y: i32, types: u32) -> bool {
		self.at(x, y).into_iter().any(|o| self.object_types(o) & types != 0)
	}

	pub fn connect_up(&self, x: i32, y: i32) -> Vec<usize> {
		if self.objects.iter().filter_map(|o| o.to_tile_split()).any(|ts| ts.x == x && ts.y == y && !ts.vertical) {
			vec![]
		} else {
			let mut connected = CONNECTED.lock().unwrap();
			let v: Vec<_> = self.at(x, y - 1)
				.into_iter()
				.map(|o| o.id())
				.filter(|o| self.objects[*o].connect_directions() == Connect::Down && !connected[*o])
				.take(2)
				.collect();
			for i in &v {
				connected[*i] = true;
			}
			v
		}
	}

	pub fn connect_down(&self, x: i32, y: i32) -> Vec<usize> {
		if self.objects.iter().filter_map(|o| o.to_tile_split()).any(|ts| ts.x == x && ts.y == y + 1 && !ts.vertical) {
			vec![]
		} else {
			let mut connected = CONNECTED.lock().unwrap();
			let v: Vec<_> = self.at(x, y + 1)
				.into_iter()
				.map(|o| o.id())
				.filter(|o| self.objects[*o].connect_directions() == Connect::Up && !connected[*o])
				.take(2)
				.collect();
			for i in &v {
				connected[*i] = true;
			}
			v
		}
	}

	pub fn object_types(&self, object: ObjectID) -> u32 {
		match object {
			ObjectID::Object(id) => self.objects[id].types(),
			ObjectID::SubObject(id, sub_id) => self.objects[id].sub_object_types(sub_id)
		}
	}

	pub fn bounding_box(&self) -> AABB {
		let mut bb = self.objects[0].bounding_box();

		for object in self.objects.iter().skip(1).filter(|o| o.to_tile_split().is_none()) {
			bb |= object.bounding_box();
		}

		if bb.height < 9.0 {
			bb.y += bb.height - 9.0;
			bb.height = 9.0;
		}

		if bb.width < 15.0 {
			bb.x += (bb.width as i32 / 2 - 7) as f32;
			bb.width = 15.0;
		}

		bb.expand_to_tile()
	}

	pub fn has_puzzle_piece(&self) -> bool {
		self.objects.iter().any(|o| o.simple_object_type() == Some(ObjectType::PuzzlePiece))
	}

	pub fn serialize(&self, to: &mut dyn Write) -> anyhow::Result<()> {
		let mut inner = vec![];

		inner.write(&[self.music as u8, self.precipitation_override as u8])?;
		inner.write(&(self.objects.len() as u32).to_le_bytes())?;
		for object in &self.objects {
			serialize_object(object.as_ref(), &mut inner)?;
		}

		let mut compressed_vec = Vec::with_capacity(inner.len() + 8);
		Compress::new(Compression::best(), false).compress_vec(&inner, &mut compressed_vec, FlushCompress::Finish)?;

		self.write_header(to, inner.len() as u32, compressed_vec.len() as u32)?;
		to.write_all(&compressed_vec)?;
		Ok(())
	}

	fn write_header(&self, to: &mut dyn Write, body_len: u32, compressed_body_len: u32) -> anyhow::Result<()> {
		to.write(&LEVEL_FILE_MAGIC_NUMBERS)?;

		let mut header = vec![];

		header.write(&MOD_VERSION.to_le_bytes())?;
		header.write(&self.author.to_le_bytes())?;
		header.write(&self.online_id.to_le_bytes())?;
		header.write(&body_len.to_le_bytes())?;
		header.write(&compressed_body_len.to_le_bytes())?;

		let flags = self.seija_abilities;
		header.write(&[flags, self.character() as u8, self.heads as u8, self.theme as u8])?;

		let mut tags = self.tags;

		if self.has_puzzle_piece() {
			tags |= Tag::PuzzlePiece.bit();
		} else {
			tags &= !Tag::PuzzlePiece.bit();
		}
		header.write(&tags.to_le_bytes())?;

		MaybeLocalized::Unlocalized(self.name.clone()).write(&mut header)?;

		to.write_all(&(header.len() as u16).to_le_bytes())?;
		to.write_all(&header)?;

		Ok(())
	}

	pub fn get_filepath(&mut self) -> &Path {
		if self.filepath.is_none() {
			self.filepath = Some(fs::new_level_filename());
		}

		self.filepath.as_ref().unwrap()
	}

	pub async fn load_from_file(path: PathBuf) -> anyhow::Result<Self> {
		let mut file = tokio::fs::File::open(&path).await?;
		let mut level = Self::load_from(&mut file).await?;
		level.filepath = Some(path);
		Ok(level)
	}

	pub async fn load_from(file: &mut (impl AsyncRead + Unpin)) -> anyhow::Result<Self> {
		let mut buf = [0u8; 8];
		file.read(&mut buf).await?;

		if buf != LEVEL_FILE_MAGIC_NUMBERS {
			return Err(anyhow!("Not a valid level file"));
		}

		let header_length = file.read_u16_le().await? as usize;

		let mut buf = vec![0u8; header_length];
		file.read(&mut buf).await?;

		let mut header_reader = Cursor::new(buf);
		let _version = header_reader.read_u16_le().await?;
		
		let author = header_reader.read_u64_le().await?;
		let online_id = header_reader.read_u32_le().await?;

		let body_len = header_reader.read_u32_le().await? as usize;
		let compressed_body_len = header_reader.read_u32_le().await? as usize;

		let flags = header_reader.read_u8().await?;
		let seija_abilities = flags & 15;
		let _character = header_reader.read_u8().await?;
		let heads = header_reader.read_u8().await? as u32;
		
		let theme = header_reader.read_u8().await?;
		let theme = LevelTheme::parse(theme)?;

		let tags = header_reader.read_u32_le().await?;

		let name = MaybeLocalized::read(&mut header_reader).await?.for_current_locale().to_owned();

		if !Self::is_name_valid(&name) {
			return Err(anyhow!("Bad name"));
		}

		let mut buf = vec![0u8; compressed_body_len];
		file.read(&mut buf).await?;

		let mut body = Vec::with_capacity(body_len);
		Decompress::new(false).decompress_vec(&buf, &mut body, FlushDecompress::Finish)?;
		let mut body = Cursor::new(body);

		let music = body.read_u8().await? as u32;
		let precipitation_override = body.read_u8().await?;
		let precipitation_override = PrecipitationOverride::parse(precipitation_override)?;
		let object_count = body.read_u32_le().await? as usize;
		let mut objects: Vec<Box<dyn LevelObject + Send>> = Vec::with_capacity(object_count);

		if object_count < 3 {
			return Err(anyhow!("Level missing required objects"));
		}

		for i in 0..object_count {
			let object_type = body.read_u8().await?;

			if (i == 0) != (object_type == 0) {
				return Err(anyhow!("First and only first object must be TileManager"));
			}

			let object_len = body.read_u32_le().await? as usize;
			let mut object_data = vec![0u8; object_len];
			body.read(&mut object_data).await?;
			objects.push(Self::deserialize_object(object_type, &object_data)?);
		}

		let mut lvl = Self {
			objects,
			author,
			tags,
			filepath: None,
			online_id,
			heads,
			seija_abilities,
			name,
			theme,
			music,
			precipitation_override,
			object_key_counter: 0,
			object_keys: vec![],
			speedrun_techniques: false,
		};

		lvl.fill_object_keys();
		Ok(lvl)
	}

	pub fn fill_object_keys(&mut self) {
		while self.object_keys.len() < self.objects.len() {
			self.object_keys.push(ObjectKey(self.object_key_counter));
			self.object_key_counter += 1;
		}
	}

	pub fn deserialize_object(object_type: u8, object_data: &[u8]) -> anyhow::Result<Box<dyn LevelObject + Send>> {
		Ok(match object_type {
			0 => Box::from(TileManager::deserialize(object_data)?),
			1 => Box::from(SimpleObject::deserialize(object_data)?),
			2 => Box::from(TileSplit::deserialize(object_data)?),
			3 => Box::from(Mochi::deserialize(object_data)),
			4 => Box::from(MovingPlatform::deserialize(object_data)),
			5 => Box::from(Warp::deserialize(object_data)),
			6 => Box::from(Teleporter::deserialize(object_data)),
			7 => Box::from(Chandelier::deserialize(object_data)),
			8 => Box::from(ExtraHead::deserialize(object_data)),
			9 => Box::from(OnmyoudamaShoot::deserialize(object_data)),
			10 => Box::from(OnmyoudamaCrawl::deserialize(object_data)),
			11 => Box::from(SymbolObject::deserialize(object_data)),
			12 => Box::from(Flipper::deserialize(object_data)),
			unknown => return Err(anyhow!(format!("Unknown object type {}", unknown)))
		})
	}

	#[cfg(not(feature = "verify"))]
	pub async fn load_into(path: PathBuf, edit: bool, ninth_head_mode: bool) {
		InternalCommand::SwitchToMenu(match Self::load_from_file(path).await {
			Ok(level) => if edit {
				
				InternalCommand::SendExternalCommand(Command::GotoRoom(super::EDITOR_ROOM)).run();
				Box::from(Editor::new(Arc::new(Mutex::new(level)), true))
			} else {
				Box::from(Play::new(Arc::new(Mutex::new(level)), PlayingFrom::MyLevels, ninth_head_mode))
			},
			Err(error) => Box::from(ErrorMenu {
				message: format!("The level data is corrupt or invalid:\n{}", error),
			})
		}).run();
	}

	#[cfg(not(feature = "verify"))]
	pub async fn download_into(id: u32, ninth_head_mode: bool) {
		InternalCommand::SwitchToMenu(match query(&DownloadLevelRQ(id)).await {
			Ok(data) => match Self::load_from(&mut data.as_slice()).await {
				Ok(level) => Box::from(Play::new(Arc::new(Mutex::new(level)), PlayingFrom::BrowseLevels, ninth_head_mode)),
				Err(error) => Box::from(ErrorMenu {
					message: format!("The level data is corrupt or invalid:\n{}", error),
				})
			}
			Err(_) => Box::from(ErrorMenu {
				message: LOC_NETWORK_ERR.for_current_locale_static().to_owned(),
			})
		}).run();
	}

	pub fn force_tag(&self, tag: Tag) -> bool {
		match tag {
			Tag::SpeedrunTechniques => self.speedrun_techniques,
			Tag::Hax => self.precipitation_override != PrecipitationOverride::None || 
				!(self.music == 0 || self.music == self.theme.bgm(false) || self.music == self.theme.bgm(true)) ||
				self.objects.iter().any(|o| o.hax()),
			_ => false
		}
	}

	#[cfg(not(feature = "verify"))]
	pub fn can_publish(&self) -> bool {
		self.online_id == u32::MAX && user::is_me(self.author) && logged_in()
	}

	#[cfg(feature = "verify")]
	pub fn prepare(&mut self) {
		if self.force_tag(Tag::Hax) {
			self.tags |= Tag::Hax.bit();
		}

		tile_manager::bake_solids(self);
	}

	#[cfg(feature = "verify")]
	pub fn metadata(&self) -> metadata::LevelMetadata {
    	use std::time::SystemTime;

		metadata::LevelMetadata {
			name: self.name.clone(),
			author: self.author,
			character: self.character(),
			tags: self.tags,
			seija_flags: self.seija_abilities,
			modified_time: SystemTime::UNIX_EPOCH,
			filename: String::new(),
			theme: self.theme,
			online_id: self.online_id,
		}
	}
}

fn serialize_object(object: &dyn LevelObject, to: &mut dyn Write) -> anyhow::Result<()> {
	let mut inner: Cursor<Vec<u8>> = Cursor::new(vec![]);
	object.serialize_inner(&mut inner)?;
	let buf = [object.serialized_type()];
	to.write(&buf)?;
	let inner = inner.into_inner();
	to.write(&(inner.len() as u32).to_le_bytes())?;
	to.write(&inner)?;
	Ok(())
}