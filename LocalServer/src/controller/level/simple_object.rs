use bincode::{Decode, Encode};

use crate::controller::{command_handler::{Command, CommandOutput, CommandSender}, game_object::{self, GameObject}, internal_command::InternalCommand, sound, sprite, undo::UndoAction};
#[cfg(not(feature = "verify"))]
use crate::controller::menu::editor::context_menu::ContextMenuItem;
use super::{Character, Connect, Level, LevelObject, LevelTheme, AABB};
use super::ObjectButton;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum Holdable {
	_Head,
	Spring,
	Spring2,
	SukimaBall,
	//_SukimaBall2,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum SwitchType {
	Coloured(Colour),
	Conveyor,
	Cannon,
	Flipper,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum SeijaItem {
	Hirarinuno,
	Camera,
	Bomb,
	Hammer
}

#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum ObjectType {
	Player(Character),
	Goal,
	PuzzlePiece,
	Semisolid,
	Key(Colour),
	Lock(Colour),
	BrickBlock,
	Nothing,
	SpikeUp,
	SpikeDown,
	SpringBasic,
	SpringDown,
	Holdable(Holdable),
	Ice(bool),
	BounceBlock,
	Switch(SwitchType, bool),
	SwitchBlock(Colour, bool),
	Gem(Colour),
	GemHole(Colour),
	GemBlock(Colour),
	Conveyor(bool),
	Bomb,
	BombStart,
	Cannon(Direction, bool),
	Icicle,
	Bell,
	Piano(u8),
	BlinkyBlock(bool),
	Detector,
	DetectorBlock(bool),
	RumiaSwitch,
	DarkSwitchBlock(bool),
	CirnoSwitch,
	IceSwitchBlock(bool),
	SemisolidGreen,
	BrownBlock,
	Fairy(bool),
	PlayerLike(bool),
	RumiaSwitchR,
	SeijaItem(SeijaItem),
	None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Encode, Decode)]
pub enum Colour {
	Red,
	Blue,
	Green,
	Yellow
}

pub struct SimpleObject {
	pub object_type: ObjectType,
	pub x: i32,
	pub y: i32,
}

impl LevelObject for SimpleObject {
	fn save_ids(&self, level: &Level) -> Vec<usize> {
		if self.object_type.stackable() {
			level.objects.iter().enumerate().filter_map(|p| {
				match p.1.simple_object_type() {
					Some(o) => if o.stackable() {
						let so = p.1.to_simple_object();
						if so.x == self.x && (so.y - self.y).abs() == 1 {
							Some(p.0)
						} else { None }
					} else { None }
					None => None
				}
			}).collect()
		} else {
			vec![]
		}
	}

	fn create(&self, command_sender: &mut dyn CommandOutput, level: &Level, return_object: bool) -> GameObject {
		if let ObjectType::Holdable(holdable) = self.object_type {
			command_sender.send(Command::F32(vec![
				(self.x * 32) as f32,
				(self.y * 32) as f32,
				holdable as u8 as f32
			]));
			command_sender.send(Command::CreateHead);
			GameObject::null()
		}

		else {
			let mut go = GameObject::new(self.object(level.theme), self.depth());
			let mut x = self.x;
			let mut y = self.y;

			if self.object_type == ObjectType::RumiaSwitchR {
				x += 1;
				y += 1;
			}

			go.x = (x * 32) as f32;
			go.y = (y * 32) as f32;
			go.create(command_sender);

			if let ObjectType::Cannon(dir, true) = self.object_type {
				if dir == Direction::Down || dir == Direction::Right {
					go.set_cannon_direction(command_sender, dir);
				}
			}

			else if self.object_type == ObjectType::Ice(true) {
				go.set_sprite(command_sender, sprite::ICE2);
			}

			else if self.object_type == ObjectType::RumiaSwitchR {
				go.set_rotation(command_sender, 180.0);
			}

			else if let ObjectType::SeijaItem(item) = self.object_type {
				go.set_real(command_sender, 0, item as u8 as f32);
			}

			if !return_object {
				go.destroy_server_only();
			}

			go
		}
	}

	fn post_create(&self, command_sender: &mut dyn CommandOutput, _level: &Level, self_object: &GameObject, saved_objects: Vec<&GameObject>) {
		for go in saved_objects {
			self_object.set_object(command_sender, (go.y < self_object.y) as u8, go);
		}
	}

	#[cfg(not(feature = "verify"))]
    fn create_editor_view(&self, command_sender: &mut dyn CommandOutput, level: &Level) -> Vec<GameObject> {
        let mut go = GameObject::new(game_object::OBJ_NO_ANIM, self.depth());
		go.x = (self.x * 32) as f32;
		go.y = (self.y * 32) as f32;
		go.create(command_sender);
		go.set_sprite(command_sender, if self.object_type == ObjectType::Goal {
			let character = level.character();
			if character == Character::Seija { sprite::GOAL } else { sprite::GOAL + character as i32 }
		} else { self.object_type.editor_sprite(level.theme) });

		if self.object_type == ObjectType::Player(Character::Seija) {
			self.create_seija_buttons(command_sender, level);
		}
		
		vec![go]
    }

	#[cfg(not(feature = "verify"))]
	fn destroy_editor_view(&self, command_sender: &mut dyn CommandOutput, objects: &mut Vec<GameObject>, level: &Level) {
		objects[0].destroy(command_sender);
		InternalCommand::ClearObjectButtons(level.object_key(self)).run();
	}

	fn can_delete(&self) -> bool {
		match self.object_type {
			ObjectType::Player(_) => false,
			ObjectType::Goal => false,
			_ => true
		}
	}

	fn bounding_box(&self) -> AABB {
		let mut bb = match self.object_type {
			ObjectType::SeijaItem(SeijaItem::Hammer) |
			ObjectType::Goal => AABB { x: 0.125/*4px*/, y: 0.125/*4px*/, width: 0.71825/*23px*/, height: 0.71825/*23px*/ },
			ObjectType::Key(_) => AABB { x: 0.0625/*2px*/, y: 0.0625/*2px*/, width: 0.84375/*27px*/, height: 0.84375/*27px*/ },
			ObjectType::PlayerLike(_) |
			ObjectType::Player(_) => AABB { x: 0.15625/*5px*/, y: 0.09375/*3px*/, width: 0.6875/*22px*/, height: 0.90625/*29px*/ },
			ObjectType::PuzzlePiece => AABB { x: 0.28125/*9px*/, y: 0.15625/*5px*/, width: 0.5/*16px*/, height: 0.5625/*18px*/ },
			ObjectType::SemisolidGreen |
			ObjectType::Semisolid => AABB { x: 0.0, y: 0.0, width: 1.0, height: 0.21875/*7px*/ },
			ObjectType::SpikeDown => AABB { x: 0.0, y: 0.0, width: 1.0, height: 0.5/*16px*/ },
			ObjectType::SpikeUp => AABB { x: 0.0, y: 0.5/*16px*/, width: 1.0, height: 0.5/*16px*/ },
			ObjectType::SpringBasic => AABB { x: 0.125/*4px*/, y: 0.4375/*14px*/, width: 0.6875/*22px*/, height: 0.5625/*18px*/ },
			ObjectType::SpringDown => AABB { x: 0.125/*4px*/, y: 0.0, width: 0.6875/*22px*/, height: 0.5625/*18px*/ },
			ObjectType::Holdable(_) => AABB { x: 0.125/*4px*/, y: 0.15625/*5px*/, width: 0.6875/*22px*/, height: 0.5625/*18px*/ },
			ObjectType::Switch(_, false) => AABB { x: 0.1875/*6px*/, y: 0.4375/*14px*/, width: 0.5625/*18px*/, height: 0.5625/*18px*/ },
			ObjectType::Switch(_, true) => AABB { x: 0.1875/*6px*/, y: 0.0, width: 0.5625/*18px*/, height: 0.5625/*18px*/ },
			ObjectType::Gem(_) |
			ObjectType::GemHole(_) => AABB { x: 0.1875/*6px*/, y: 0.21875/*7px*/, width: 0.5625/*18px*/, height: 0.78125/*25px*/ },
			ObjectType::Icicle => AABB { x: 0.25/*8px*/, y: 0.0, width: 0.5/*16px*/, height: 0.84375/*27px*/ },
			ObjectType::Bell |
			ObjectType::Piano(_) => AABB { x: 0.125/*4px*/, y: 0.4375/*14px*/, width: 0.6875/*22px*/, height: 0.375/*12px*/ },
			ObjectType::RumiaSwitch => AABB { x: 0.25/*8px*/, y: 0.0, width: 0.4375/*14px*/, height: 1.0 },
			ObjectType::RumiaSwitchR => AABB { x: 0.25/*8px*/, y: 0.0, width: 0.4375/*14px*/, height: 1.0 },
			ObjectType::SeijaItem(SeijaItem::Hirarinuno) => AABB { x: 0.1875/*6px*/, y: 0.1875/*6px*/, width: 0.5625/*18px*/, height: 0.5625/*18px*/ },
			ObjectType::SeijaItem(SeijaItem::Camera) => AABB { x: 0.125/*4px*/, y: 0.25/*8px*/, width: 0.6875/*22px*/, height: 0.46875/*15px*/ },
			ObjectType::SeijaItem(SeijaItem::Bomb) => AABB { x: 0.1875/*6px*/, y: 0.125/*4px*/, width: 0.5625/*18px*/, height: 0.6875/*22px*/ },
			_ => AABB { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }
		};

		bb.x += self.x as f32;
		bb.y += self.y as f32;

		bb
	}

	fn move_by(&mut self, x: i32, y: i32) {
		self.x += x;
		self.y += y;
	}

	fn types(&self) -> u32 {
		self.object_type.types()
	}

	fn serialize_inner(&self, to: &mut dyn std::io::Write) -> anyhow::Result<()> {
		to.write(&(self.x as i16).to_le_bytes())?;
		to.write(&(self.y as i16).to_le_bytes())?;

		to.write(&bincode::encode_to_vec(&self.object_type, bincode::config::standard())?)?;

		Ok(())
	}

	fn serialized_type(&self) -> u8 {1}

	fn to_simple_object(&self) -> &SimpleObject {self}
	fn to_simple_object_mut(&mut self) -> &mut SimpleObject {self}

	fn simple_object_type(&self) -> Option<ObjectType> {
		Some(self.object_type)
	}

	#[cfg(not(feature = "verify"))]
	fn context_menu_items(&self, theme: LevelTheme) -> Vec<ContextMenuItem> {
		let variants = self.object_type.variants();
		if variants.len() > 1 {
			vec![ContextMenuItem::IconList(
				variants.iter().map(|o| o.editor_sprite(theme)).collect(),
				variants.iter().position(|o| *o == self.object_type).unwrap_or(usize::MAX),
				0.5
			)]
		} else { vec![] }
	}

	fn handle_context_menu_action(&mut self, command_sender: &mut CommandSender, object: usize, action: i32, theme: LevelTheme) -> Vec<UndoAction> {
		for variant in self.object_type.variants() {
			if variant.editor_sprite(theme) == action {
				let old = self.object_type;
				self.object_type = variant;
				return if variant == old {vec![]} else {
					sound::play(command_sender, variant.place_sound());
					vec![UndoAction::SetSimpleObjectType(object, old)]
				};
			}
		}

		vec![]
	}

	fn connect_directions(&self) -> Connect {
		match self.object_type {
			ObjectType::SpikeDown |
			ObjectType::SpringDown |
			ObjectType::RumiaSwitchR |
			ObjectType::Switch(_, true) => Connect::Up,
			ObjectType::SpikeUp |
			ObjectType::SpringBasic |
			ObjectType::GemHole(_) |
			ObjectType::RumiaSwitch |
			ObjectType::Switch(_, false) => Connect::Down,
			_ => Connect::None
		}
	}

	fn recreate_on_character_change(&self) -> bool {
		self.object_type == ObjectType::Goal
	}
}

pub const SEIJA_COLOUR_OFF: u32 = 0x707070;
const SEIJA_COLOUR_ON: u32 = 0xffffff;

impl SimpleObject {
	pub fn deserialize(from: &[u8]) -> anyhow::Result<Self> {
		let x = i16::from_le_bytes(from[0..2].try_into()?) as i32;
		let y = i16::from_le_bytes(from[2..4].try_into()?) as i32;
		let object_type = bincode::decode_from_slice(&from[4..], bincode::config::standard())?.0;

		Ok(Self {x, y, object_type})
	}
	
	fn depth(&self) -> i32 {
		match self.object_type {
			ObjectType::Key(_) => 1,
			ObjectType::PuzzlePiece |
			ObjectType::SeijaItem(_) |
			ObjectType::PlayerLike(_) |
			ObjectType::Player(_) => 10,
			ObjectType::Lock(_) |
			ObjectType::SwitchBlock(_, _) |
			ObjectType::DarkSwitchBlock(_) |
			ObjectType::IceSwitchBlock(_) |
			ObjectType::GemBlock(_) |
			ObjectType::BlinkyBlock(_) |
			ObjectType::DetectorBlock(_) |
			ObjectType::BrickBlock => 19,
			ObjectType::Bell |
			ObjectType::Piano(_) => 20,
			ObjectType::SpikeDown |
			ObjectType::SpikeUp |
			ObjectType::SemisolidGreen |
			ObjectType::Semisolid => 21,
			ObjectType::SpringBasic |
			ObjectType::SpringDown |
			ObjectType::Gem(_) |
			ObjectType::Holdable(_) => 5,
			ObjectType::RumiaSwitchR |
			ObjectType::RumiaSwitch => 12,
			ObjectType::GemHole(_) => 301,
			ObjectType::Detector => 400,
			ObjectType::BrownBlock => 1000,
			_ => 0
		}
	}

	fn object(&self, theme: LevelTheme) -> u32 {
		match self.object_type {
			ObjectType::Player(_) => game_object::OBJ_PLAYER,
			ObjectType::Goal => game_object::OBJ_GOAL,
			ObjectType::PuzzlePiece => game_object::OBJ_PUZZLEPIECE,
			ObjectType::BrickBlock => game_object::OBJ_BLOCK,
			ObjectType::Key(c) => game_object::OBJ_KEY + c as u32,
			ObjectType::Lock(c) => game_object::OBJ_KEYBLOCK + c as u32,
			ObjectType::Switch(SwitchType::Coloured(c), false) => game_object::OBJ_SWITCH + (c as u32) * 2,
			ObjectType::Switch(SwitchType::Coloured(c), true) => game_object::OBJ_SWITCH + 1 + (c as u32) * 2,
			ObjectType::SwitchBlock(c, false) => game_object::OBJ_SWITCHBLOCK + (c as u32) * 4,
			ObjectType::SwitchBlock(c, true) => game_object::OBJ_SWITCHBLOCK + 1 + (c as u32) * 4,
			ObjectType::Nothing => game_object::OBJ_BLANK,
			ObjectType::SpikeUp => game_object::OBJ_SPIKE,
			ObjectType::SpikeDown => game_object::OBJ_SPIKE2,
			ObjectType::SpringBasic => game_object::OBJ_SPRING,
			ObjectType::SpringDown => game_object::OBJ_SPRING2,
			ObjectType::Ice(_) => game_object::OBJ_ICEWALL1,
			ObjectType::BounceBlock => game_object::OBJ_BOUNCEBLOCK,
			ObjectType::Gem(Colour::Red) => game_object::OBJ_GEM_RED,
			ObjectType::Gem(Colour::Blue) => game_object::OBJ_GEM_BLUE,
			ObjectType::Gem(Colour::Yellow) => game_object::OBJ_GEM_YELLOW,
			ObjectType::Gem(Colour::Green) => game_object::OBJ_GEM_GREEN,
			ObjectType::GemHole(Colour::Red) => game_object::OBJ_GEMHOLE_RED,
			ObjectType::GemHole(Colour::Blue) => game_object::OBJ_GEMHOLE_BLUE,
			ObjectType::GemHole(Colour::Yellow) => game_object::OBJ_GEMHOLE_YELLOW,
			ObjectType::GemHole(Colour::Green) => game_object::OBJ_GEMHOLE_GREEN,
			ObjectType::GemBlock(Colour::Red) => game_object::OBJ_GEMBLOCK_RED,
			ObjectType::GemBlock(Colour::Blue) => game_object::OBJ_GEMBLOCK_BLUE,
			ObjectType::GemBlock(Colour::Yellow) => game_object::OBJ_GEMBLOCK_YELLOW,
			ObjectType::GemBlock(Colour::Green) => game_object::OBJ_GEMBLOCK_GREEN,
			ObjectType::Conveyor(false) => game_object::OBJ_LFLOOR,
			ObjectType::Conveyor(true) => game_object::OBJ_RFLOOR,
			ObjectType::Switch(SwitchType::Conveyor, false) => game_object::OBJ_GREYSWITCH,
			ObjectType::Switch(SwitchType::Conveyor, true) => game_object::OBJ_GREYSWITCH_R,
			ObjectType::Cannon(d, false) => game_object::OBJ_CANNON + d as u32,
			ObjectType::Cannon(Direction::Up, true) |
			ObjectType::Cannon(Direction::Down, true) => game_object::OBJ_CANNON_UD,
			ObjectType::Cannon(_, true) => game_object::OBJ_CANNON_LR,
			ObjectType::Switch(SwitchType::Cannon, false) => game_object::OBJ_WHITESWITCH,
			ObjectType::Switch(SwitchType::Cannon, true) => game_object::OBJ_WHITESWITCH_R,
			ObjectType::Bomb => game_object::OBJ_BOMBBLOCK,
			ObjectType::BombStart => game_object::OBJ_BOMBBLOCK2,
			ObjectType::None |
			ObjectType::Holdable(_) => u32::MAX,
			ObjectType::Icicle => game_object::OBJ_ICICLE,
			ObjectType::Bell => game_object::OBJ_BELL,
			ObjectType::Piano(k) => game_object::OBJ_PIANO + k as u32,
			ObjectType::BlinkyBlock(false) => game_object::OBJ_TIMEWALL,
			ObjectType::BlinkyBlock(true) => game_object::OBJ_TIMEWALL_OFF,
			ObjectType::Detector => game_object::OBJ_DETECTOR,
			ObjectType::DetectorBlock(false) => game_object::OBJ_DETECTORBLOCK1,
			ObjectType::DetectorBlock(true) => game_object::OBJ_DETECTORBLOCK2,
			ObjectType::RumiaSwitch => game_object::OBJ_DARKSWITCH,
			ObjectType::RumiaSwitchR => game_object::OBJ_DARKSWITCH,
			ObjectType::DarkSwitchBlock(false) => game_object::OBJ_WALLWHITE,
			ObjectType::DarkSwitchBlock(true) => game_object::OBJ_WALLWHITE_OFF,
			ObjectType::CirnoSwitch => game_object::OBJ_ICESWITCH,
			ObjectType::IceSwitchBlock(false) => game_object::OBJ_WALLICE,
			ObjectType::IceSwitchBlock(true) => game_object::OBJ_WALLICE_OFF,
			ObjectType::BrownBlock => game_object::OBJ_BROWNBLOCK,
			ObjectType::Fairy(false) => game_object::OBJ_FAIRY1,
			ObjectType::Fairy(true) => game_object::OBJ_FAIRY2,
			ObjectType::PlayerLike(false) => game_object::OBJ_DOREMY,
			ObjectType::PlayerLike(true) => game_object::OBJ_FLANDRE,
			ObjectType::Switch(SwitchType::Flipper, false) => game_object::OBJ_BLACK_SWITCH,
			ObjectType::Switch(SwitchType::Flipper, true) => game_object::OBJ_REVERSE_SWITCH,
			ObjectType::SeijaItem(_) => game_object::OBJ_SEIJA_GRANT_ITEM,
			ObjectType::SemisolidGreen => game_object::OBJ_FLOOR3,
			ObjectType::Semisolid => match theme {
				LevelTheme::OutsideWorld |
				LevelTheme::DreamScraps |
				LevelTheme::MindBreak |
				LevelTheme::Fireflies |
				LevelTheme::Entrance |
				LevelTheme::Rumia |
				LevelTheme::Rasobi |
				LevelTheme::JerryAttack => game_object::OBJ_FLOOR2,
				_ => game_object::OBJ_FLOOR
			}
		}
	}

	#[cfg(not(feature = "verify"))]
	fn create_seija_buttons(&self, command_sender: &mut dyn CommandOutput, level: &Level) {
		let object = level.object_key(self);

		{
			let mut go = GameObject::new(game_object::OBJ_BLANK, -9100);
			go.x = (self.x * 32 - 12) as f32;
			go.y = (self.y * 32 + 8) as f32;
			go.create(command_sender);
			go.set_sprite(command_sender, sprite::SEIJA_BUTTON_LEFT);
			if level.seija_abilities & 1 == 0 {
				go.set_colour(command_sender, SEIJA_COLOUR_OFF);
			}

			InternalCommand::CreateObjectButton(ObjectButton {
				game_object: go,
				object,
				bounds: AABB {
					x: (self.x * 32 - 12) as f32,
					y: (self.y * 32 + 8) as f32,
					width: 20.0,
					height: 16.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					editor.level.lock().unwrap().seija_abilities ^= 1;
					let colour = editor.object_buttons[button].game_object.colour;
					editor.object_buttons[button].game_object.set_colour(command_sender, colour ^ (SEIJA_COLOUR_OFF ^ SEIJA_COLOUR_ON));
					vec![UndoAction::SeijaPowerToggle(editor.object_buttons[button].game_object.sprite)]
				}
			}).run();
		}

		{
			let mut go = GameObject::new(game_object::OBJ_BLANK, -9100);
			go.x = (self.x * 32 + 20) as f32;
			go.y = (self.y * 32 + 8) as f32;
			go.create(command_sender);
			go.set_sprite(command_sender, sprite::SEIJA_BUTTON_RIGHT);
			if level.seija_abilities & 2 == 0 {
				go.set_colour(command_sender, SEIJA_COLOUR_OFF);
			}

			InternalCommand::CreateObjectButton(ObjectButton {
				game_object: go,
				object,
				bounds: AABB {
					x: (self.x * 32 + 24) as f32,
					y: (self.y * 32 + 8) as f32,
					width: 20.0,
					height: 16.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					editor.level.lock().unwrap().seija_abilities ^= 2;
					let colour = editor.object_buttons[button].game_object.colour;
					editor.object_buttons[button].game_object.set_colour(command_sender, colour ^ (SEIJA_COLOUR_OFF ^ SEIJA_COLOUR_ON));
					vec![UndoAction::SeijaPowerToggle(editor.object_buttons[button].game_object.sprite)]
				}
			}).run();
		}

		{
			let mut go = GameObject::new(game_object::OBJ_BLANK, -9100);
			go.x = (self.x * 32 + 8) as f32;
			go.y = (self.y * 32 - 12) as f32;
			go.create(command_sender);
			go.set_sprite(command_sender, sprite::SEIJA_BUTTON_UP);
			if level.seija_abilities & 4 == 0 {
				go.set_colour(command_sender, SEIJA_COLOUR_OFF);
			}

			InternalCommand::CreateObjectButton(ObjectButton {
				game_object: go,
				object,
				bounds: AABB {
					x: (self.x * 32 + 8) as f32,
					y: (self.y * 32 - 12) as f32,
					width: 16.0,
					height: 20.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					editor.level.lock().unwrap().seija_abilities ^= 4;
					let colour = editor.object_buttons[button].game_object.colour;
					editor.object_buttons[button].game_object.set_colour(command_sender, colour ^ (SEIJA_COLOUR_OFF ^ SEIJA_COLOUR_ON));
					vec![UndoAction::SeijaPowerToggle(editor.object_buttons[button].game_object.sprite)]
				}
			}).run();
		}

		{
			let mut go = GameObject::new(game_object::OBJ_BLANK, -9100);
			go.x = (self.x * 32 + 8) as f32;
			go.y = (self.y * 32 + 20) as f32;
			go.create(command_sender);
			go.set_sprite(command_sender, sprite::SEIJA_BUTTON_DOWN);
			if level.seija_abilities & 8 == 0 {
				go.set_colour(command_sender, SEIJA_COLOUR_OFF);
			}

			InternalCommand::CreateObjectButton(ObjectButton {
				game_object: go,
				object,
				bounds: AABB {
					x: (self.x * 32 + 8) as f32,
					y: (self.y * 32 + 24) as f32,
					width: 16.0,
					height: 20.0
				},
				callback: |command_sender, editor, button| {
					sound::play(command_sender, sound::SE_SELECT);
					editor.level.lock().unwrap().seija_abilities ^= 8;
					let colour = editor.object_buttons[button].game_object.colour;
					editor.object_buttons[button].game_object.set_colour(command_sender, colour ^ (SEIJA_COLOUR_OFF ^ SEIJA_COLOUR_ON));
					vec![UndoAction::SeijaPowerToggle(editor.object_buttons[button].game_object.sprite)]
				}
			}).run();
		}
	}
}

impl ObjectType {
	pub const fn types(&self) -> u32 {
		match self {
			Self::Goal |
			Self::PuzzlePiece |
			Self::SeijaItem(_) |
			Self::Key(_) => super::L_OBJECT,
			Self::Player(_) => !super::L_DECORATION & !super::L_IMMUTABLE_BLOCK_UPPER & !super::L_TILE & !super::L_RAIL_MOVER & !super::L_SYMBOL,
			Self::SpikeDown |
			Self::SemisolidGreen |
			Self::Semisolid => super::L_IMMUTABLE_BLOCK_UPPER,
			Self::SpikeUp => super::L_IMMUTABLE_BLOCK_LOWER,
			Self::Switch(_, true) => super::L_IMMUTABLE_BLOCK_UPPER | super::L_PHYSICS_OBJECT,
			Self::Switch(_, false) => super::L_IMMUTABLE_BLOCK_LOWER | super::L_PHYSICS_OBJECT,
			Self::BrickBlock |
			Self::Ice(_) |
			Self::BounceBlock |
			Self::Bomb |
			Self::BombStart |
			Self::GemBlock(_) |
			Self::SwitchBlock(_, _) |
			Self::DarkSwitchBlock(_) |
			Self::IceSwitchBlock(_) |
			Self::BlinkyBlock(_) |
			Self::Detector |
			Self::BrownBlock |
			Self::DetectorBlock(_) |
			Self::Lock(_) => super::L_BLOCK,
			Self::Holdable(_) |
			Self::GemHole(_) |
			Self::Gem(_) |
			Self::Bell |
			Self::Piano(_) |
			Self::RumiaSwitch |
			Self::RumiaSwitchR |
			Self::SpringBasic => super::L_PHYSICS_OBJECT,
			Self::Fairy(_) |
			Self::PlayerLike(_) => super::L_PHYSICS_OBJECT | super::L_PHYSICS_OBJECT_FULL,
			Self::SpringDown => super::L_PHYSICS_OBJECT | super::L_IMMUTABLE_BLOCK_UPPER,
			Self::Cannon(_, _) |
			Self::CirnoSwitch |
			Self::Conveyor(_) => super::PL_IMMUTABLE_BLOCK,
			Self::Icicle => super::L_PHYSICS_OBJECT | super::L_IMMUTABLE_BLOCK_UPPER | super::L_BLOCK,
			Self::None |
			Self::Nothing => !0,
		}
	}

	pub const fn editor_sprite(self, theme: LevelTheme) -> i32 {
		match self {
			Self::Player(Character::Banki) => sprite::BANKI,
			Self::Player(Character::Cirno) => sprite::CIRNO,
			Self::Player(Character::Rumia) => sprite::RUMIA,
			Self::Player(Character::Seija) => sprite::SEIJA,
			Self::Goal => sprite::GOAL,
			Self::PuzzlePiece => sprite::PUZZLE,
			Self::BrickBlock => sprite::BLOCK,
			Self::Key(c) => sprite::KEY + c as i32,
			Self::Lock(c) => sprite::KEYBLOCK + c as i32,
			Self::Switch(SwitchType::Coloured(c), false) => sprite::SWITCH + c as i32,
			Self::Switch(SwitchType::Coloured(c), true) => sprite::SWITCH_RED_R + c as i32,
			Self::SwitchBlock(c, false) => sprite::SWITCHBLOCK + c as i32 * 2,
			Self::SwitchBlock(c, true) => sprite::SWITCHBLOCK + 1 + c as i32 * 2,
			Self::Nothing => sprite::POOF,
			Self::SpikeUp => sprite::SPIKE,
			Self::SpikeDown => sprite::SPIKE2,
			Self::SpringBasic => sprite::SPRING,
			Self::SpringDown => sprite::SPRING2,
			Self::Holdable(Holdable::Spring) => sprite::SPRING3,
			Self::Holdable(Holdable::Spring2) => sprite::SPRING4,
			Self::Holdable(_) => -1,
			Self::Ice(false) => sprite::ICE,
			Self::Ice(true) => sprite::ICE2,
			Self::BounceBlock => sprite::BOUNCEBLOCK,
			Self::Gem(Colour::Red) => sprite::GEM_RED,
			Self::Gem(Colour::Blue) => sprite::GEM_BLUE,
			Self::Gem(Colour::Yellow) => sprite::GEM_YELLOW,
			Self::Gem(Colour::Green) => sprite::GEM_GREEN,
			Self::GemHole(Colour::Red) => sprite::GEMHOLE_RED,
			Self::GemHole(Colour::Blue) => sprite::GEMHOLE_BLUE,
			Self::GemHole(Colour::Yellow) => sprite::GEMHOLE_YELLOW,
			Self::GemHole(Colour::Green) => sprite::GEMHOLE_GREEN,
			Self::GemBlock(Colour::Red) => sprite::GEMBLOCK_RED,
			Self::GemBlock(Colour::Blue) => sprite::GEMBLOCK_BLUE,
			Self::GemBlock(Colour::Yellow) => sprite::GEMBLOCK_YELLOW,
			Self::GemBlock(Colour::Green) => sprite::GEMBLOCK_GREEN,
			Self::Switch(SwitchType::Conveyor, false) => sprite::SWITCH_GREY,
			Self::Switch(SwitchType::Conveyor, true) => sprite::SWITCH_GREY_R,
			Self::Conveyor(false) => sprite::LFLOOR,
			Self::Conveyor(true) => sprite::RFLOOR,
			Self::Switch(SwitchType::Cannon, false) => sprite::SWITCH_WHITE,
			Self::Switch(SwitchType::Cannon, true) => sprite::SWITCH_WHITE_R,
			Self::Cannon(d, false) => sprite::CANNON + d as i32,
			Self::Cannon(Direction::Up, true) => sprite::CANNON_RED,
			Self::Cannon(d, true) => sprite::CANNON_RED_X - 1 + d as i32,
			Self::Bomb => sprite::BOMBBLOCK,
			Self::BombStart => sprite::BOMBBLOCK2,
			Self::Icicle => sprite::ICICLE,
			Self::Bell => sprite::BELL,
			Self::Piano(k) => sprite::PIANO + k as i32,
			Self::BlinkyBlock(false) => sprite::TIMEWALL,
			Self::BlinkyBlock(true) => sprite::TIMEWALL_OFF,
			Self::Detector => sprite::BANKIBLOCK,
			Self::DetectorBlock(false) => sprite::BANKIBLOCK2,
			Self::DetectorBlock(true) => sprite::BANKIBLOCK_WHITE,
			Self::RumiaSwitch => sprite::DARKSWITCH,
			Self::RumiaSwitchR => sprite::DARKSWITCH_R,
			Self::DarkSwitchBlock(false) => sprite::WALLWHITE,
			Self::DarkSwitchBlock(true) => sprite::WALLWHITE_OFF,
			Self::CirnoSwitch => sprite::ICESWITCH,
			Self::IceSwitchBlock(false) => sprite::WALLICE,
			Self::IceSwitchBlock(true) => sprite::WALLICE_OFF,
			Self::BrownBlock => sprite::BROWNBLOCK,
			Self::SemisolidGreen => sprite::FLOOR3,
			Self::Fairy(false) => sprite::FAIRY_BLUE,
			Self::Fairy(true) => sprite::FAIRY_BLACK,
			Self::PlayerLike(false) => sprite::DOREMY,
			Self::PlayerLike(true) => sprite::FLANDRE,
			Self::Switch(SwitchType::Flipper, false) => sprite::BLACK_SWITCH,
			Self::Switch(SwitchType::Flipper, true) => sprite::REVERSE_SWITCH,
			Self::SeijaItem(item) => sprite::SEIJA_HIRARINUNO + item as i32,
			Self::Semisolid => match theme {
				LevelTheme::OutsideWorld |
				LevelTheme::DreamScraps |
				LevelTheme::MindBreak |
				LevelTheme::Fireflies |
				LevelTheme::Entrance |
				LevelTheme::Rumia |
				LevelTheme::Rasobi |
				LevelTheme::JerryAttack => sprite::FLOOR2,
				_ => sprite::FLOOR
			}
			Self::None => -1
		}
	}

	pub fn variants(self) -> Vec<Self> {
		match self {
			Self::Semisolid | Self::SemisolidGreen => vec![Self::Semisolid, Self::SemisolidGreen],
			Self::Key(_) => vec![Self::Key(Colour::Red), Self::Key(Colour::Blue), Self::Key(Colour::Green), Self::Key(Colour::Yellow)],
			Self::Lock(_) => vec![Self::Lock(Colour::Red), Self::Lock(Colour::Blue), Self::Lock(Colour::Green), Self::Lock(Colour::Yellow)],
			Self::SpringBasic |
			Self::SpringDown |
			Self::Holdable(Holdable::Spring) |
			Self::Holdable(Holdable::Spring2) => vec![Self::SpringBasic, Self::Holdable(Holdable::Spring), Self::Holdable(Holdable::Spring2), Self::SpringDown],
			Self::Switch(_, _) => vec![
				Self::Switch(SwitchType::Coloured(Colour::Red), false),
				Self::Switch(SwitchType::Coloured(Colour::Blue), false),
				Self::Switch(SwitchType::Coloured(Colour::Green), false),
				Self::Switch(SwitchType::Coloured(Colour::Yellow), false),
				Self::Switch(SwitchType::Conveyor, false),
				Self::Switch(SwitchType::Cannon, false),
				Self::Switch(SwitchType::Flipper, false), Self::None,
				Self::Switch(SwitchType::Coloured(Colour::Red), true),
				Self::Switch(SwitchType::Coloured(Colour::Blue), true),
				Self::Switch(SwitchType::Coloured(Colour::Green), true),
				Self::Switch(SwitchType::Coloured(Colour::Yellow), true),
				Self::Switch(SwitchType::Conveyor, true),
				Self::Switch(SwitchType::Cannon, true),
				Self::Switch(SwitchType::Flipper, true),
			],
			Self::DetectorBlock(_) |
			Self::DarkSwitchBlock(_) |
			Self::IceSwitchBlock(_) |
			Self::BrownBlock |
			Self::SwitchBlock(_, _) => vec![
				Self::SwitchBlock(Colour::Red, false),
				Self::SwitchBlock(Colour::Blue, false),
				Self::SwitchBlock(Colour::Green, false),
				Self::SwitchBlock(Colour::Yellow, false),
				Self::DarkSwitchBlock(false),
				Self::IceSwitchBlock(false),
				Self::DetectorBlock(false),
				Self::BrownBlock,
				Self::SwitchBlock(Colour::Red, true),
				Self::SwitchBlock(Colour::Blue, true),
				Self::SwitchBlock(Colour::Green, true),
				Self::SwitchBlock(Colour::Yellow, true),
				Self::DarkSwitchBlock(true),
				Self::IceSwitchBlock(true),
				Self::DetectorBlock(true),
			],
			Self::Gem(_) => vec![Self::Gem(Colour::Red), Self::Gem(Colour::Blue), Self::Gem(Colour::Green), Self::Gem(Colour::Yellow)],
			Self::GemHole(_) => vec![Self::GemHole(Colour::Red), Self::GemHole(Colour::Blue), Self::GemHole(Colour::Green), Self::GemHole(Colour::Yellow)],
			Self::GemBlock(_) => vec![Self::GemBlock(Colour::Red), Self::GemBlock(Colour::Blue), Self::GemBlock(Colour::Green), Self::GemBlock(Colour::Yellow)],
			Self::Conveyor(_) => vec![Self::Conveyor(false), Self::Conveyor(true)],
			Self::Bomb | Self::BombStart => vec![Self::BombStart, Self::Bomb],
			Self::BlinkyBlock(_) => vec![Self::BlinkyBlock(false), Self::BlinkyBlock(true)],
			Self::Cannon(_, _) => vec![
				Self::Cannon(Direction::Up, false),
				Self::Cannon(Direction::Down, false),
				Self::Cannon(Direction::Left, false),
				Self::Cannon(Direction::Right, false),
				Self::Cannon(Direction::Up, true),
				Self::Cannon(Direction::Down, true),
				Self::Cannon(Direction::Left, true),
				Self::Cannon(Direction::Right, true),
			],
			Self::Bell | Self::Piano(_) => vec![
				Self::Bell,
				Self::Piano(0),
				Self::Piano(1),
				Self::Piano(2),
				Self::Piano(3),
				Self::Piano(4),
				Self::Piano(5),
				Self::Piano(6),
			],
			Self::PlayerLike(_) |
			Self::Fairy(_) => vec![
				Self::Fairy(false),
				Self::Fairy(true),
				Self::PlayerLike(false),
				Self::PlayerLike(true),
			],
			Self::RumiaSwitch | Self::RumiaSwitchR => vec![Self::RumiaSwitch, Self::RumiaSwitchR],
			Self::Player(_) => vec![
				Self::Player(Character::Banki),
				Self::Player(Character::Cirno),
				Self::Player(Character::Rumia),
				Self::Player(Character::Seija),
			],
			_ => vec![]
		}
	}

	pub fn place_sound(self) -> f32 {
		match self {
			Self::PuzzlePiece => sound::SE_PEACE_GET,
			Self::Key(_) => sound::SE_ITEM_GET,
			Self::Lock(_) => sound::SE_UNLOCK,
			Self::BombStart |
			Self::BrickBlock => sound::SE_BLOCK,
			Self::Switch(_, _) |
			Self::Holdable(_) |
			Self::SpringDown |
			Self::SpringBasic => sound::SE_SWITCH,
			Self::BounceBlock => sound::SE_BUBBLE,
			Self::GemHole(_) |
			Self::Gem(_) => sound::SE_SWITCHBLOCKON,
			Self::Cannon(_, _) => sound::SE_HEAD,
			Self::Bell => sound::SE_BELL1 + (rand::random::<u8>() % 5) as f32,
			Self::Piano(k) => (sound::SE_PIANO1 + k) as f32,
			Self::RumiaSwitchR |
			Self::RumiaSwitch => sound::SE_DARK,
			Self::CirnoSwitch => sound::SE_ICESWITCH,
			_ => sound::SE_HOLD
		}
	}

	pub const fn unique(self) -> bool {
		match self {
			Self::PuzzlePiece => true,
			Self::Holdable(Holdable::SukimaBall) => true,
			Self::SeijaItem(_) => true,
			_ => false
		}
	}

	const fn stackable(self) -> bool {
		match self {
			Self::Fairy(_) => true,
			Self::PlayerLike(_) => true,
			_ => false
		}
	}
}