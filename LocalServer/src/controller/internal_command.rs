use std::{sync::Mutex, mem};

use super::{command_handler::Command, level::{ObjectKey, ObjectButton}};

#[cfg(not(feature = "verify"))]
use super::menu::Menu;

#[cfg(feature = "verify")]
trait Menu {}

pub enum InternalCommand {
	SwitchToMenu(Box<dyn Menu + Send>),
	SendExternalCommand(Command),
	ClearObjectButtons(ObjectKey),
	CreateObjectButton(ObjectButton),
	CreateChainObjectInserter(usize, usize, i32),
}

static QUEUE: Mutex<Vec<InternalCommand>> = Mutex::new(vec![]);

pub fn get_queue() -> Vec<InternalCommand> {
	mem::take(&mut QUEUE.lock().unwrap())
}

pub fn run(command: InternalCommand) {
	QUEUE.lock().unwrap().push(command);
}

impl InternalCommand {
	pub fn run(self) {
		run(self);
	}
}