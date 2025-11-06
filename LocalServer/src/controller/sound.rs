use std::sync::atomic::{AtomicBool, Ordering};

use super::command_handler::{Command, CommandOutput, CommandSender};

#[allow(unused)] pub const SE_HEAD: f32 = 0.0;
#[allow(unused)] pub const SE_HEAD2: f32 = 1.0;
#[allow(unused)] pub const SE_HEADPLUS: f32 = 2.0;
#[allow(unused)] pub const SE_DARK: f32 = 3.0;
#[allow(unused)] pub const SE_DARKBREAK: f32 = 4.0;
#[allow(unused)] pub const SE_ICE: f32 = 5.0;
#[allow(unused)] pub const SE_ICESHOT: f32 = 6.0;
#[allow(unused)] pub const SE_ICESWITCH: f32 = 7.0;
#[allow(unused)] pub const SE_BREAK: f32 = 8.0;
#[allow(unused)] pub const SE_BLOCK: f32 = 9.0;
#[allow(unused)] pub const SE_JUMP: f32 = 10.0;
#[allow(unused)] pub const SE_JUMP2: f32 = 11.0;
#[allow(unused)] pub const SE_WALK: f32 = 12.0;
#[allow(unused)] pub const SE_WARP: f32 = 13.0;
#[allow(unused)] pub const SE_UNLOCK: f32 = 14.0;
#[allow(unused)] pub const SE_SWITCHBLOCKON: f32 = 15.0;
#[allow(unused)] pub const SE_SWITCH: f32 = 16.0;
#[allow(unused)] pub const SE_DEAD: f32 = 17.0;
#[allow(unused)] pub const SE_NOT: f32 = 18.0;
#[allow(unused)] pub const SE_HOLD: f32 = 19.0;
#[allow(unused)] pub const SE_PAUSE: f32 = 20.0;
#[allow(unused)] pub const SE_HARDSTYLE: f32 = 21.0;
#[allow(unused)] pub const SE_SELECT: f32 = 22.0;
#[allow(unused)] pub const SE_SELECTRUMIA: f32 = 23.0;
#[allow(unused)] pub const SE_SELECTSEIJA: f32 = 24.0;
#[allow(unused)] pub const SE_STAGEDECIDERUMIA: f32 = 25.0;
#[allow(unused)] pub const SE_DECIDERUMIA: f32 = 26.0;
#[allow(unused)] pub const SE_DECIDESEIJA: f32 = 27.0;
#[allow(unused)] pub const SE_DECIDE: f32 = 28.0;
#[allow(unused)] pub const SE_CANCEL: f32 = 29.0;
#[allow(unused)] pub const SE_STAGEDECIDE: f32 = 30.0;
#[allow(unused)] pub const SE_QUICK: f32 = 31.0;
#[allow(unused)] pub const SE_QUICK2: f32 = 32.0;
#[allow(unused)] pub const SE_ITEM_GET: f32 = 33.0;
#[allow(unused)] pub const SE_GOAL_GET: f32 = 34.0;
#[allow(unused)] pub const SE_PEACE_GET: f32 = 35.0;
#[allow(unused)] pub const SE_PIANO1: u8 = 36;
#[allow(unused)] pub const SE_PIANO2: f32 = 37.0;
#[allow(unused)] pub const SE_PIANO3: f32 = 38.0;
#[allow(unused)] pub const SE_PIANO4: f32 = 39.0;
#[allow(unused)] pub const SE_PIANO5: f32 = 40.0;
#[allow(unused)] pub const SE_PIANO6: f32 = 41.0;
#[allow(unused)] pub const SE_PIANO7: f32 = 42.0;
#[allow(unused)] pub const SE_BELL1: f32 = 43.0;
#[allow(unused)] pub const SE_BELL2: f32 = 44.0;
#[allow(unused)] pub const SE_BELL3: f32 = 45.0;
#[allow(unused)] pub const SE_BELL4: f32 = 46.0;
#[allow(unused)] pub const SE_BELL5: f32 = 47.0;
#[allow(unused)] pub const SE_CANNONBLOCK: f32 = 48.0;
#[allow(unused)] pub const SE_MESSAGE: f32 = 49.0;
#[allow(unused)] pub const SE_CONNECT: f32 = 50.0;
#[allow(unused)] pub const SE_ITEMF: f32 = 51.0;
#[allow(unused)] pub const SE_FIREWORK: f32 = 52.0;
#[allow(unused)] pub const SE_CAMERA: f32 = 53.0;
#[allow(unused)] pub const SE_CAMERA2: f32 = 54.0;
#[allow(unused)] pub const SE_HIRARINUNO: f32 = 55.0;
#[allow(unused)] pub const SE_HIRARINUNO2: f32 = 56.0;
#[allow(unused)] pub const SE_HAMMER: f32 = 57.0;
#[allow(unused)] pub const SE_HAMMER2: f32 = 58.0;
#[allow(unused)] pub const SE_SYOUKAI: f32 = 59.0;
#[allow(unused)] pub const SE_BUBBLE: f32 = 60.0;
#[allow(unused)] pub const SE_MOCHI1: f32 = 61.0;
#[allow(unused)] pub const SE_MOCHI2: f32 = 62.0;
#[allow(unused)] pub const SE_CHANDELIER1: f32 = 63.0;
#[allow(unused)] pub const SE_CHANDELIER2: f32 = 64.0;
#[allow(unused)] pub const SE_READYVOICE: f32 = 65.0;
#[allow(unused)] pub const SE_COUNTDOWN: f32 = 66.0;
#[allow(unused)] pub const SE_GOVOICE: f32 = 67.0;
#[allow(unused)] pub const SE_HURRY: f32 = 68.0;
#[allow(unused)] pub const SE_JERRY: f32 = 69.0;
#[allow(unused)] pub const SE_SIGEMI: f32 = 70.0;

pub const BGM_KAISOU: u32 = 85;

pub const BGM_EDITOR_MENU: u32 = BGM_KAISOU;

static ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_bgm(command_sender: &mut dyn CommandOutput, track: u32) {
	command_sender.send(Command::F32(vec![track as f32]));
	command_sender.send(Command::SetMusic);
}

pub fn play(command_sender: &mut CommandSender, sound: f32) {
	if ENABLED.load(Ordering::Relaxed) {
		command_sender.send(Command::F32(vec![sound]));
		command_sender.send(Command::PlaySound);
	}
}

pub fn enable() {
	ENABLED.store(true, Ordering::Relaxed);
}

pub fn disable() {
	ENABLED.store(false, Ordering::Relaxed);
}