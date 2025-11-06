use crate::controller::{command_handler::CommandSender, level::{moving_platform::{self, MovingPlatform}, simple_object::{Holdable, ObjectType}, teleporter::{self, Teleporter}, warp::{self, Warp}, Character, LevelTheme}, sound, sprite, undo::UndoAction};

use super::{tool::Tool, Editor};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PairedObject {
	MovingPlatform,
	Warp,
	Teleporter,
}

impl PairedObject {
	fn types(self) -> u32 {
		match self {
			Self::MovingPlatform => moving_platform::LAYER,
			Self::Warp => warp::LAYER,
			Self::Teleporter => teleporter::LAYER,
		}
	}

	fn sound(self) -> f32 {
		match self {
			Self::MovingPlatform => ObjectType::Semisolid.place_sound(),
			Self::Teleporter |
			Self::Warp => sound::SE_WARP,
		}
	}

	fn sprite(self) -> i32 {
		match self {
			Self::MovingPlatform => sprite::MOVINGFLOOR,
			Self::Warp => sprite::WARP + 1,
			Self::Teleporter => sprite::SUKIMA + 1,
		}
	}
}

pub struct ToolPairedObject {
	pub object: PairedObject
}

impl Tool for ToolPairedObject {
	fn use_start(&self, command_sender: &mut CommandSender, editor: &mut Editor, x: f32, y: f32) -> Vec<UndoAction> {
        let x = x.floor() as i32;
		let y = y.floor() as i32;

		let level = editor.level.lock().unwrap();

		if self.object == PairedObject::Teleporter && level.objects.iter().any(
			|o| o.simple_object_type() == Some(ObjectType::Holdable(Holdable::SukimaBall))
		) {
			sound::play(command_sender, sound::SE_NOT);
			return vec![];
		}

		if level.any_type_match(x, y, self.object.types()) || level.any_type_match(x + 1, y, self.object.types()) {
			return vec![];
		}

		drop(level);

		sound::play(command_sender, self.object.sound());
		vec![UndoAction::Delete(editor.add(command_sender, match self.object {
			PairedObject::MovingPlatform => Box::from(MovingPlatform::new(x, y)),
			PairedObject::Warp => Box::from(Warp::new(x, y)),
			PairedObject::Teleporter => Box::from(Teleporter::new(x, y)),
		}))]
    }

	fn sprite(&self, _theme: LevelTheme, _character: Character) -> i32 {
		self.object.sprite()
	}
}