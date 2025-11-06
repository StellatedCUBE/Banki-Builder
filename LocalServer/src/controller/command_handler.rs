use tokio::{net::tcp::OwnedWriteHalf, io::AsyncWriteExt};
use anyhow::Result;

use super::{game_object::GameObject, level::{simple_object::Direction, Character}};

pub const GLOBAL_QUICKRETRY: char = 'Q';

pub enum Command {
	Log(String),
	Quit,
	GotoRoom(u32),
	F32(Vec<f32>),
	CreateInstance(usize),
	DestroyInstance(usize),
	SetPosition(usize),
	SetSprite,
	SetAlpha,
	SetColour,
	SetScale,
	SetRotation,
	SetFrame,
	SetReal(u8),
	SetString(u8, String),
	SetObject(u8),
	GetString(u8),
	GetReal(u8),
	SetCannonDirection(Direction),
	SetBackgrounds,
	SetTile,
	AddTile,
	MoveCamera,
	Yield,
	CreateHead,
	SetLevelData(Character),
	SetGlobal(char),
	SetRoomStartCommands(Vec<Self>),
	SetMusic,
	PlaySound,
	AtomicBlockEnd,
	Zoom(u32),
	NinthHead(bool),
	SetTimes,
	WriteTAS(String)
}

impl Command {
	fn serialize(&self) -> String {
		match self {
			Self::Log(msg) => format!("l{}\0", msg),
			Self::Quit => "q\0".to_owned(),
			Self::GotoRoom(room) => format!(">{}\0", room),
			Self::F32(vec) => {
				let mut data = vec![0u8; 2 + vec.len() * 8];
				data[0] = 'f' as u8;

				for i in 0..vec.len() {
					let bytes = vec[i].to_le_bytes();
					for j in 0..4 {
						data[8 * i + 2 * j + 1] = 64 | (bytes[j] >> 4);
						data[8 * i + 2 * j + 2] = 64 | (bytes[j] & 15);
					}
				}

				String::from_utf8(data).unwrap()
			}
			Self::SetPosition(id) => format!("+{}\0", id),
			Self::SetSprite => "s\0".to_owned(),
			Self::SetAlpha => "A\0".to_owned(),
			Self::SetColour => "C\0".to_owned(),
			Self::SetScale => "X\0".to_owned(),
			Self::SetRotation => "D\0".to_owned(),
			Self::SetFrame => "F\0".to_owned(),
			Self::SetMusic => "M\0".to_owned(),
			Self::PlaySound => "P\0".to_owned(),
			Self::SetReal(var) => format!("R{}\0", var),
			Self::GetReal(var) => format!("U{}\0", var),
			Self::SetObject(var) => format!("O{}\0", var),
			Self::SetString(var, val) => format!("S{}{}\0", var, val),
			Self::GetString(var) => format!("T{}\0", var),
			Self::MoveCamera => "v\0".to_owned(),
			Self::CreateInstance(id) => format!("({}\0", id),
			Self::DestroyInstance(id) => format!("){}\0", id),
			Self::SetBackgrounds => "b\0".to_owned(),
			Self::SetTile => "g\0".to_owned(),
			Self::AddTile => "h\0".to_owned(),
			Self::Yield => "Y\0".to_owned(),
			Self::SetLevelData(Character::Banki) => "@sekibanki\0".to_owned(),
			Self::SetLevelData(Character::Cirno) => "@cirno\0".to_owned(),
			Self::SetLevelData(Character::Rumia) => "@rumia\0".to_owned(),
			Self::SetLevelData(Character::Seija) => "@seija\0".to_owned(),
			Self::SetGlobal(var) => format!("G{}\0", var),
			Self::CreateHead => "H\0".to_owned(),
			Self::AtomicBlockEnd => "_\0".to_owned(),
			Self::SetTimes => "t\0".to_owned(),
			Self::Zoom(vw) => format!("~{}\0", vw),
			Self::SetCannonDirection(Direction::Up) => "*UP\0".to_owned(),
			Self::SetCannonDirection(Direction::Down) => "*DOWN\0".to_owned(),
			Self::SetCannonDirection(Direction::Left) => "*LEFT\0".to_owned(),
			Self::SetCannonDirection(Direction::Right) => "*RIGHT\0".to_owned(),
			Self::NinthHead(false) => "9N\0".to_owned(),
			Self::NinthHead(true) => "9Y\0".to_owned(),
			Self::WriteTAS(filename) => format!("]{}\0", filename),
			Self::SetRoomStartCommands(commands) => {
				let mut data = "I".to_owned();

				for command in commands {
					let mut command_data = command.serialize();
					let command_bytes = unsafe { command_data.as_bytes_mut() };

					if command_bytes[command_bytes.len() - 1] != 0 {
						panic!("Command didn't end with null byte");
					}

					command_bytes[command_bytes.len() - 1] = 0x1e;

					if data.as_bytes().len() + command_bytes.len() > 65535 {
						let data_bytes = unsafe { data.as_bytes_mut() };

						if data_bytes[data_bytes.len() - 1] != 0x1e {
							panic!("Data doesn't end with split byte");
						}

						data_bytes[data_bytes.len() - 1] = 0;
						data += "i";
					}

					data += &command_data;
				}

				data + "\0"
			}
		}
	}
}

pub trait CommandOutput {
	fn send(&mut self, command: Command);
}

pub struct CommandSender {
	sock: OwnedWriteHalf,
	queue: std::sync::Mutex<Vec<Command>>
}

impl CommandSender {
	pub fn new(sock: OwnedWriteHalf) -> Self {
		Self {
			sock,
			queue: std::sync::Mutex::new(vec![])
		}
	}

	pub async fn flush(&mut self) -> Result<()>{
		let queue = self.queue.get_mut().unwrap();

		for command in queue.iter() {
			let data = command.serialize();
			self.sock.write_all(data.as_bytes()).await?;
		}

		queue.clear();

		Ok(())
	}

	pub fn send_immut(&self, command: Command) {
		let mut queue = self.queue.lock().unwrap();
		queue.push(command);
	}

	pub fn empty(&self) -> bool {
		self.queue.lock().unwrap().is_empty()
	}
}

impl CommandOutput for CommandSender {
	fn send(&mut self, command: Command) {
		if let Command::GotoRoom(_) = command {
			GameObject::clear_all();
		}
		
		self.queue.get_mut().unwrap().push(command);
	}
}

pub struct CommandRecorder {
	pub record: Vec<Command>
}

impl CommandRecorder {
	pub fn new() -> Self {
		Self {
			record: vec![]
		}
	}
}

impl CommandOutput for CommandRecorder {
    fn send(&mut self, command: Command) {
        self.record.push(command);
    }
}