use std::sync::Mutex;

use super::{command_handler::{Command, CommandOutput}, level::simple_object::Direction};

pub const OBJ_BLANK: u32 = 1504;
pub const OBJ_BLACK_TRANSITION_1: u32 = 932;
pub const OBJ_BLACK_TRANSITION_2: u32 = 933;
pub const OBJ_BG1: u32 = 711;
pub const OBJ_BGSEIJA: u32 = 722;
pub const OBJ_BGENTRANCE: u32 = 709;
pub const OBJ_BROWNBLOCK: u32 = 3;
pub const OBJ_GOAL: u32 = 1217;
pub const OBJ_PLAYER: u32 = 65;
pub const OBJ_GAMEMGR: u32 = 1375;
pub const OBJ_RUMIAPREVIEW: u32 = 102;
pub const OBJ_PAUSEMGR: u32 = 737;
pub const OBJ_QUICKRETRYMGR: u32 = 1383;
pub const OBJ_VIEWMODEMGR: u32 = 1391;
pub const OBJ_PUZZLEPIECE: u32 = 220;
pub const OBJ_FLOOR: u32 = 611;
pub const OBJ_FLOOR2: u32 = 612;
pub const OBJ_FLOOR3: u32 = 613;
pub const OBJ_FLOORDUMMY: u32 = 614;
pub const OBJ_QUICKRETRY: u32 = 942;
pub const OBJ_VIEWLEFT: u32 = 1499;
pub const OBJ_VIEWRIGHT: u32 = 1500;
pub const OBJ_VIEWTOP: u32 = 1497;
pub const OBJ_VIEWBOTTOM: u32 = 1498;
pub const OBJ_SPIKE: u32 = 107;
pub const OBJ_SPIKE2: u32 = 108;
pub const OBJ_BLOCK: u32 = 19;
pub const OBJ_JERRY: u32 = 211;
pub const OBJ_KEY: u32 = 215;
pub const OBJ_KEYBLOCK: u32 = 12;
pub const OBJ_CANDLE: u32 = 222;
pub const OBJ_LANTERN: u32 = 223;
pub const OBJ_LANTERNK: u32 = 224;
pub const OBJ_KOMOREBI: u32 = 705;
pub const OBJ_SPRING: u32 = 109;
pub const OBJ_SPRING2: u32 = 110;
pub const OBJ_ICEWALL1: u32 = 31;
pub const OBJ_BOUNCEBLOCK: u32 = 64;
pub const OBJ_SWITCH: u32 = 140;
pub const OBJ_SWITCHBLOCK: u32 = 39;
pub const OBJ_GEM_RED: u32 = 120;
pub const OBJ_GEM_BLUE: u32 = 124;
pub const OBJ_GEM_YELLOW: u32 = 132;
pub const OBJ_GEM_GREEN: u32 = 128;
pub const OBJ_GEMHOLE_RED: u32 = 121;
pub const OBJ_GEMHOLE_BLUE: u32 = 125;
pub const OBJ_GEMHOLE_YELLOW: u32 = 133;
pub const OBJ_GEMHOLE_GREEN: u32 = 129;
pub const OBJ_GEMBLOCK_RED: u32 = 60;
pub const OBJ_GEMBLOCK_BLUE: u32 = 61;
pub const OBJ_GEMBLOCK_YELLOW: u32 = 63;
pub const OBJ_GEMBLOCK_GREEN: u32 = 62;
pub const OBJ_GREYSWITCH: u32 = 148;
pub const OBJ_LFLOOR: u32 = 10;
pub const OBJ_RFLOOR: u32 = 11;
pub const OBJ_BOMBBLOCK: u32 = 24;
pub const OBJ_BOMBBLOCK2: u32 = 25;
pub const OBJ_CANNON: u32 = 169;
pub const OBJ_CANNON_UD: u32 = 173;
pub const OBJ_CANNON_LR: u32 = 174;
pub const OBJ_WHITESWITCH: u32 = 149;
pub const OBJ_WATER: u32 = 180;
pub const OBJ_ICICLE: u32 = 152;
pub const OBJ_MOCHI: u32 = 138;
pub const OBJ_WARP: u32 = 155;
pub const OBJ_BELL: u32 = 118;
pub const OBJ_PIANO: u32 = 111;
pub const OBJ_TIMEWALL: u32 = 16;
pub const OBJ_TIMEWALL_OFF: u32 = 17;
pub const OBJ_TRACKPOSITIONMGR: u32 = 1483;
pub const OBJ_DARKSWITCH: u32 = 136;
pub const OBJ_WALLWHITE: u32 = 58;
pub const OBJ_WALLWHITE_OFF: u32 = 59;
pub const OBJ_ICESWITCH: u32 = 55;
pub const OBJ_WALLICE: u32 = 56;
pub const OBJ_WALLICE_OFF: u32 = 57;
pub const OBJ_CHANDELIER: u32 = 176;
pub const OBJ_HEADPLUS: u32 = 219;
pub const OBJ_ONMYOUDAMA: u32 = 671;
pub const OBJ_REVERSE_SWITCH: u32 = 151;


pub const OBJ_CUSTOM_BASE: u32 = 1517;
pub const OBJ_CURSOR: u32 = OBJ_CUSTOM_BASE + 1;
pub const OBJ_TEXTBOX: u32 = OBJ_CUSTOM_BASE + 2;
pub const OBJ_EDITOR_ACTUAL: u32 = OBJ_CUSTOM_BASE + 3;
pub const OBJ_NO_ANIM: u32 = OBJ_CUSTOM_BASE + 4;
pub const OBJ_STAGESTARTMGR: u32 = OBJ_CUSTOM_BASE + 5;
pub const OBJ_UI: u32 = OBJ_CUSTOM_BASE + 6;
pub const OBJ_TEXT: u32 = OBJ_CUSTOM_BASE + 7;
pub const OBJ_YELLOW_BOX: u32 = OBJ_CUSTOM_BASE + 8;
pub const OBJ_FILLED_RECTANGLE: u32 = OBJ_CUSTOM_BASE + 9;
pub const OBJ_DETECTOR: u32 = OBJ_CUSTOM_BASE + 10;
pub const OBJ_DETECTORBLOCK1: u32 = OBJ_CUSTOM_BASE + 11;
pub const OBJ_DETECTORBLOCK2: u32 = OBJ_CUSTOM_BASE + 12;
pub const OBJ_PRECIPITATOR: u32 = OBJ_CUSTOM_BASE + 13;
pub const _OBJ_PRECIPITATION: u32 = OBJ_CUSTOM_BASE + 14;
pub const OBJ_XBG: u32 = OBJ_CUSTOM_BASE + 15;
pub const OBJ_SEIJACAMERA: u32 = OBJ_CUSTOM_BASE + 16;
pub const OBJ_MOVINGFLOOR: u32 = OBJ_CUSTOM_BASE + 17;
pub const OBJ_MOVINGFLOOR_EDITOR: u32 = OBJ_CUSTOM_BASE + 18;
pub const OBJ_WARP_EDITOR: u32 = OBJ_CUSTOM_BASE + 19;
pub const OBJ_ONMYOUDAMA_CRAWL: u32 = OBJ_CUSTOM_BASE + 20;
pub const _OBJ_FAIRYDEAD: u32 = OBJ_CUSTOM_BASE + 21;
pub const OBJ_FAIRY1: u32 = OBJ_CUSTOM_BASE + 22;
pub const OBJ_FAIRY2: u32 = OBJ_CUSTOM_BASE + 23;
pub const _OBJ_MOVING_CHANDELIER: u32 = OBJ_CUSTOM_BASE + 24;
pub const _OBJ_MOVING_CHANDELIER_LINE: u32 = OBJ_CUSTOM_BASE + 25;
pub const OBJ_CONVEYOR_CHANDELIER: u32 = OBJ_CUSTOM_BASE + 26;
pub const OBJ_DOREMY: u32 = OBJ_CUSTOM_BASE + 27;
pub const OBJ_FLANDRE: u32 = OBJ_CUSTOM_BASE + 28;
pub const OBJ_BLACK_SWITCH: u32 = OBJ_CUSTOM_BASE + 30;
pub const OBJ_GREYSWITCH_R: u32 = OBJ_CUSTOM_BASE + 31;
pub const OBJ_WHITESWITCH_R: u32 = OBJ_CUSTOM_BASE + 32;
pub const OBJ_FLIPPER: u32 = OBJ_CUSTOM_BASE + 33;
pub const _OBJ_ICE_BULLET_CAMERA_TRACKER: u32 = OBJ_CUSTOM_BASE + 34;
pub const OBJ_SEIJA_GRANT_ITEM: u32 = OBJ_CUSTOM_BASE + 35;
pub const _OBJ_SEIJA_PHOTO: u32 = OBJ_CUSTOM_BASE + 36;
pub const OBJ_NO_PHOTOGRAPHY: u32 = OBJ_CUSTOM_BASE + 37;
pub const OBJ_TAG_BUTTON: u32 = OBJ_CUSTOM_BASE + 38;
pub const OBJ_THROBBER: u32 = OBJ_CUSTOM_BASE + 39;
pub const OBJ_WATCH_SRT: u32 = OBJ_CUSTOM_BASE + 40;
pub const OBJ_WALL: u32 = OBJ_CUSTOM_BASE + 41;


static OBJECT_IDS: Mutex<Vec<bool>> = Mutex::new(vec![]);

pub struct GameObject {
	id: Option<usize>,
	pub object_type: u32,
	pub depth: i32,
	pub x: f32,
	pub y: f32,
	pub sprite: i32,
	pub colour: u32,
	pub rotation: f32,
}

impl GameObject {
	pub fn clear_all() {
		OBJECT_IDS.lock().unwrap().clear();
	}

	pub const fn new(object_type: u32, depth: i32) -> Self {
		Self {
			id: None,
			object_type,
			depth,
			x: 0.0,
			y: 0.0,
			sprite: -1,
			colour: 0xffffff,
			rotation: 0.0,
		}
	}

	pub const fn null() -> Self {
		Self::new(u32::MAX, 0)
	}

	pub fn exists(&self) -> bool {
		self.id.is_some()
	}

	pub fn create(&mut self, command_sender: &mut dyn CommandOutput) -> usize {
		if let Some(existing_id) = self.id {
			return existing_id;
		}

		if self.object_type == u32::MAX {
			panic!("Creating null GameObject");
		}

		let mut object_ids = OBJECT_IDS.lock().unwrap();
		let mut id = 0;

		while id < object_ids.len() {
			if !object_ids[id] {
				object_ids[id] = true;
				self.id = Some(id);
				break;
			}
			id += 1;
		}

		object_ids.push(true);
		self.id = Some(id);

		command_sender.send(Command::F32(vec![self.x, self.y, self.depth as f32, self.object_type as f32]));
		command_sender.send(Command::CreateInstance(id));

		id
	}

	pub fn destroy_server_only(&mut self) {
		if let Some(id) = self.id {
			let mut object_ids = OBJECT_IDS.lock().unwrap();
			if object_ids.len() > id {
				object_ids[id] = false;
			} else {
				panic!("Object free after room free");
			}
			self.id = None;
		}
	}

	pub fn destroy(&mut self, command_sender: &mut dyn CommandOutput) {
		if let Some(id) = self.id {
			command_sender.send(Command::DestroyInstance(id))
		}

		self.destroy_server_only();
	}

	pub fn update_position(&self, command_sender: &mut dyn CommandOutput) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![self.x, self.y]));
			command_sender.send(Command::SetPosition(id));
		}
	}

	pub fn set_sprite(&mut self, command_sender: &mut dyn CommandOutput, sprite: i32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, sprite as f32]));
			command_sender.send(Command::SetSprite);
			self.sprite = sprite;
		}
	}

	pub fn set_sprite_frame(&self, command_sender: &mut dyn CommandOutput, frame: u32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, frame as f32]));
			command_sender.send(Command::SetFrame);
		}
	}

	pub fn set_alpha(&self, command_sender: &mut dyn CommandOutput, alpha: f32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, alpha]));
			command_sender.send(Command::SetAlpha);
		}
	}

	pub fn set_colour(&mut self, command_sender: &mut dyn CommandOutput, colour: u32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, colour as f32]));
			command_sender.send(Command::SetColour);
			self.colour = colour;
		}
	}

	pub fn set_scale(&self, command_sender: &mut dyn CommandOutput, x: f32, y: f32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, x, y]));
			command_sender.send(Command::SetScale);
		}
	}

	pub fn set_rotation(&mut self, command_sender: &mut dyn CommandOutput, rotation: f32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, rotation]));
			command_sender.send(Command::SetRotation);
			self.rotation = rotation;
		}
	}

	pub fn set_cannon_direction(&self, command_sender: &mut dyn CommandOutput, direction: Direction) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32]));
			command_sender.send(Command::SetCannonDirection(direction));
		}
	}

	pub fn set_real(&self, command_sender: &mut dyn CommandOutput, variable: u8, value: f32) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, value as f32]));
			command_sender.send(Command::SetReal(variable));
		}
	}

	pub fn set_string(&self, command_sender: &mut dyn CommandOutput, variable: u8, value: &str) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32]));
			command_sender.send(Command::SetString(variable, value.to_owned()));
		}
	}

	pub fn set_object_id(&self, command_sender: &mut dyn CommandOutput, variable: u8, value: usize) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32, value as f32]));
			command_sender.send(Command::SetObject(variable));
		}
	}

	pub fn set_object(&self, command_sender: &mut dyn CommandOutput, variable: u8, value: &Self) {
		if let Some(id) = value.id {
			self.set_object_id(command_sender, variable, id);
		}
	}

	pub fn query_string(&self, command_sender: &mut dyn CommandOutput, variable: u8) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32]));
			command_sender.send(Command::GetString(variable));
		}
	}

	pub fn _query_real(&self, command_sender: &mut dyn CommandOutput, variable: u8) {
		if let Some(id) = self.id {
			command_sender.send(Command::F32(vec![id as f32]));
			command_sender.send(Command::GetReal(variable));
		}
	}
}