mod loc;
mod command_handler;
#[cfg(not(feature = "verify"))]
pub mod menu;
#[cfg(not(feature = "verify"))]
mod event;
mod game_object;
mod sprite;
mod bg;
mod sound;
#[cfg(not(feature = "verify"))]
mod global_state;
pub mod level;
pub mod fs;
mod internal_command;
#[cfg(not(feature = "verify"))]
mod raw_input;
#[cfg(not(feature = "verify"))]
mod mod_input;
#[cfg(not(feature = "verify"))]
mod control_settings;
#[cfg(not(feature = "verify"))]
pub mod net;
#[cfg(not(feature = "verify"))]
mod config;
mod font;
pub mod undo;
#[cfg(not(feature = "verify"))]
mod im;

use std::{mem, process, time::Duration};

use command_handler::CommandOutput;
#[cfg(not(feature = "verify"))]
use im::IMPacket;
#[cfg(not(feature = "verify"))]
use mod_input::ModInput;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{tcp::OwnedReadHalf, TcpListener, TcpStream}, select, sync::mpsc::{channel, Receiver}, time::interval};
use anyhow::Result;

#[cfg(not(feature = "verify"))]
use self::{command_handler::{CommandSender, Command}, menu::{Menu, not_in_editor::NotInEditor}, event::Event, global_state::ControllerGlobalState, internal_command::InternalCommand, raw_input::RawInput};

pub const EDITOR_ROOM: u32 = 165;

#[cfg(not(feature = "verify"))]
pub async fn run() {
    use port_check::free_local_ipv4_port_in_range;

	let port = if cfg!(windows) {
		free_local_ipv4_port_in_range(58001..).unwrap()
	} else { 58008 };
	let listener = TcpListener::bind(("127.0.0.1", port)).await.unwrap();
	let key = gen_key();
	fs::write_portkey(port, key);

	let im_key = gen_key();

	config::load();
	tokio::spawn(net::setup());

	loop {
		if let Ok((sock, _)) = listener.accept().await {
			tokio::spawn(on_connect(sock, key, im_key));
		}
	}
}

fn gen_key() -> [u8; 16] {
	rand::random::<u128>().to_ne_bytes()
}

#[cfg(not(feature = "verify"))]
async fn on_connect(mut sock: TcpStream, key: [u8; 16], im_key: [u8; 16]) -> Result<()> {
	println!("Received connection");

	let mut keybuf = [0u8; 16];
	sock.read_exact(&mut keybuf).await?;

	if im_key == keybuf {
		println!("IM key match");
		im::on_im_connect(sock).await;
		return Ok(());
	}

	if cfg!(windows) && key != keybuf {
		sock.shutdown().await?;
		return Ok(());
	}

	println!("Key match");

	let port = sock.local_addr().unwrap().port();

	if let Err(err) = Controller::new(sock, port, im_key).run().await {
		println!("Error while running controller: {:?}", err);
	}

	process::exit(0)
}

#[cfg(not(feature = "verify"))]
struct Controller {
	sock: OwnedReadHalf,
	command_sender: CommandSender,
	menu: Box<dyn Menu + Send>,
	global: ControllerGlobalState,
	im_recv: Receiver<IMPacket>,
	im_key: [u8; 16],
	port: u16,
}

#[cfg(not(feature = "verify"))]
impl Controller {
	fn new(sock: TcpStream, port: u16, im_key: [u8; 16]) -> Self {
		println!("Constructing controller");

		let (sock, sock_w) = sock.into_split();
		let command_sender = CommandSender::new(sock_w);
		let (im_send, im_recv) = channel(32);
		*im::IM_EVENT_PRODUCER.lock().unwrap() = Some(im_send);
		Self {
			command_sender,
			sock,
			menu: Box::from(NotInEditor {}),
			global: ControllerGlobalState::default(),
			im_recv,
			im_key,
			port,
		}
	}
	
	async fn run(&mut self) -> Result<()> {
		println!("Running controller");

		//self.command_sender.send(Command::GameData);
		self.global.window_focused = true;

		if config::get().use_custom_mouse_handler {
			if cfg!(windows) && std::fs::exists("Z:\\etc\\passwd").is_ok_and(|x| x) {
				//#[cfg(windows)]
				//im::launch_proton(self.port, self.im_key);
			} else {
				#[cfg(windows)]
				im::launch_windows();
				#[cfg(not(windows))]
				im::launch_linux(self.port, self.im_key);
			}
		}

		let mut timer = interval(Duration::from_nanos(16666666));
		let mut exiting = false;
		//let mut drop_click_events = false;

		while !exiting {
			let mut im_packet = IMPacket::default();

			let event = select! {
				e = self.sock.read_u8() => e.unwrap_or_else(|_| {
					exiting = true;
					128
				}),
				Some(imp) = self.im_recv.recv() => {
					im_packet = imp;
					64
				}
				_ = timer.tick() => 255u8
			};
			
			let mut event = Event::from(event);

			if event == Event::IMData && self.global.window_focused {
				event = im_packet.event;
				match event {
					Event::MouseUp |
					Event::MouseDown => if im_packet.data > 3 {
						self.global.last_raw_input = RawInput::Mouse(im_packet.data as u8);
						self.global.last_mod_input = ModInput::from(self.global.last_raw_input);
					} else {
						continue;
					}

					_ => {
						//drop_click_events = true;
						continue;
					}
				}
			} /*else if drop_click_events && (event == Event::MouseDown || event == Event::MouseUp) {
				let _ = self.sock.read_u8().await;
				continue;
			}*/ else {
				self.handle_event_global(event).await?;
			}

			if !self.global.input_focused || (event != Event::KeyDown && event != Event::KeyUp) {
				self.menu.on_event(&mut self.command_sender, event, &mut self.global);
			}

			for command in internal_command::get_queue() {
				match command {
					InternalCommand::SwitchToMenu(menu) => {
						self.menu.on_leave(&mut self.command_sender);
						println!("Switching to menu {}", menu.name());
						self.menu = menu;
						self.menu.on_enter(&mut self.command_sender);
					}
					InternalCommand::SendExternalCommand(command) => self.command_sender.send_immut(command),
					InternalCommand::ClearObjectButtons(id) => {
						self.global.recieved_real = unsafe { mem::transmute(id) };
						self.menu.on_event(&mut self.command_sender, Event::ClearObjectButtons, &mut self.global);
					}
					InternalCommand::CreateObjectButton(button) => {
						self.global.object_button_massive_hack = Some(button);
						self.menu.on_event(&mut self.command_sender, Event::CreateObjectButton, &mut self.global);
					}
					InternalCommand::CreateChainObjectInserter(object, index, sprite) => {
						self.menu.create_chain_object_inserter(&mut self.command_sender, object, index, sprite);
					}
				}
			}

			if !exiting && !self.command_sender.empty() {
				self.command_sender.send(Command::AtomicBlockEnd);
				self.command_sender.flush().await?;
			}
		}

		Ok(())
	}

	async fn handle_event_global(&mut self, event: Event) -> Result<()> {
		match event {
			Event::MouseMove => {
				let old_mouse_x = self.global.mouse_x;
				let old_mouse_y = self.global.mouse_y;
				self.global.mouse_x = self.sock.read_f64_le().await? as f32;
				self.global.mouse_y = self.sock.read_f64_le().await? as f32;
				self.global.mouse_dx = self.global.mouse_x - old_mouse_x;
				self.global.mouse_dy = self.global.mouse_y - old_mouse_y;
				let (device_x, device_y) = match mouse_position::mouse_position::Mouse::get_mouse_position() {
					mouse_position::mouse_position::Mouse::Position { x, y } => (x, y),
					_ => (0, 0)
				};
				self.global.was_mouse_actually_moved = self.global.mouse_raw.0 != device_x || self.global.mouse_raw.1 != device_y;
				self.global.mouse_raw = (device_x, device_y);
			}
			Event::MouseUp |
			Event::MouseDown => {
				let button = self.sock.read_u8().await?;
				self.global.last_raw_input = RawInput::Mouse(button);
				self.global.last_mod_input = ModInput::from(self.global.last_raw_input);
			}
			Event::ButtonUp |
			Event::ButtonDown => {
				let button = self.sock.read_u8().await?;

				self.global.last_raw_input = RawInput::ControllerButton(button);
				self.global.last_mod_input = ModInput::new(self.global.last_raw_input, &self.global.controller_buttons);
				let ind = button as usize / 64;
				let bit = 1 << (button % 64);
				if event == Event::ButtonDown {
					self.global.controller_buttons[ind] |= bit;
				} else {
					self.global.controller_buttons[ind] &= !bit;
				}
			}
			Event::KeyUp |
			Event::KeyDown => {
				let key = match self.sock.read_f64_le().await? as u32 {
					160 => 16,
					161 => 16,
					162 => 17,
					163 => 17,
					164 => 18,
					165 => 18,
					x => x
				};

				self.global.last_raw_input = RawInput::Key(key);
				self.global.last_mod_input = ModInput::new(self.global.last_raw_input, &self.global.keyboard_buttons);
				if key < 512 {
					let ind = key as usize / 64;
					let bit = 1 << (key % 64);
					if event == Event::KeyDown {
						self.global.keyboard_buttons[ind] |= bit;
					} else {
						self.global.keyboard_buttons[ind] &= !bit;
					}
				}
			}

			Event::RoomLoad => {
				self.global.controller_buttons = [0; 4];
				self.global.keyboard_buttons = [0; 8];
			}

			Event::FocusIn |
			Event::FocusOut => {
				self.global.controller_buttons = [0; 4];
				self.global.keyboard_buttons = [0; 8];
				self.global.window_focused = event == Event::FocusIn;

			}

			Event::InputFocus => self.global.input_focused = true,
			Event::InputUnfocus => {
				self.global.input_focused = false;
				self.global.recieved_real = self.sock.read_u8().await? as f64;
			}

			Event::LevelCompletePP |
			Event::LevelCompleteNoPP => self.global.level_clear_time = self.sock.read_f64_le().await? as u32,

			Event::ViewMove => {
				self.global.view_x = self.sock.read_f64_le().await? as f32;
				self.global.view_y = self.sock.read_f64_le().await? as f32;
			}
			Event::GetString => {
				let len = self.sock.read_f64_le().await? as usize;
				let mut buf = vec![0u8; len];
				self.sock.read_exact(&mut buf).await?;
				self.global.recieved_string = String::from_utf8(buf)?;
			}
			Event::GetReal => {
				self.global.recieved_real = self.sock.read_f64_le().await?;
			}
			_ => ()
		}

		/*match event {
			Event::MouseUp | Event::MouseDown | Event::KeyDown | Event::KeyUp | Event::ButtonDown | Event::ButtonUp => {
				println!("{:?} {:?}", event, self.global.last_mod_input);
			}
			_ => ()
		}*/

		Ok(())
	}
}