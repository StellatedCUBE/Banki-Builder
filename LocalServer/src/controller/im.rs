use std::{mem, process::Command, sync::Mutex};

use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc::Sender};

use super::event::Event;

#[derive(Clone, Copy)]
pub struct IMPacket {
	pub event: Event,
	pub data: u32,
}

impl IMPacket {
	fn from(data: u32) -> Self {
		Self {
			event: Event::from(data as u8),
			data: data >> 8,
		}
	}

	pub const fn default() -> Self {
		Self {
			event: Event::Tick,
			data: 0,
		}
	}
}

pub static IM_EVENT_PRODUCER: Mutex<Option<Sender<IMPacket>>> = Mutex::new(None);

pub async fn on_im_connect(sock: TcpStream) {
	let sender = mem::take(&mut *IM_EVENT_PRODUCER.lock().unwrap()).unwrap();
	let (mut sock, _) = sock.into_split();

	loop {
		match sock.read_u32_le().await {
			Ok(data) => {
				let _ = sender.send(IMPacket::from(data)).await;
			}
			Err(_) => break
		}
	}
}

#[cfg(windows)]
pub fn launch_proton(port: u16, key: [u8; 16]) {
	use std::os::windows::process::CommandExt;

	if let Ok(mut exe) = std::env::current_exe() {
		let k = rand::random::<u64>();
		exe.pop();
		exe.push("linux-im");
		let host_path = if let Some(p) = exe.to_str().filter(|_| exe.starts_with("z:")) {
			p[2..].replace('\\', "/")
		} else {
			let p = format!("/tmp/banki-builder-linux-im-{}", k);
			if std::fs::copy(exe, &p).is_err() {
				return;
			}
			p
		};

		if std::fs::write(
			format!("z:\\tmp\\banki-builder-linux-im-{}.sh", k),
			format!(
				"#rm /tmp/banki-builder-linux-im-{}.sh\nx='{}'\nchmod +x \"$x\"\n\"$x\" {} {}\nrm /tmp/banki-builder-linux-im-{}\n",
				k,
				host_path.replace('\'', "'\\''"),
				port,
				hex::encode(key),
				k,
			)
		).is_ok() {
			let _ = Command::new("cmd")
			.raw_arg(format!("/C start /unix /bin/sh /tmp/banki-builder-linux-im-{}.sh", k))
			.spawn();
		}
	}
}

pub fn launch_linux(port: u16, key: [u8; 16]) {
	let _ = Command::new("../linux-im/target/release/linux-im").arg(port.to_string()).arg(hex::encode(key)).spawn();
}

#[cfg(windows)]
pub fn launch_windows() {
	use rdev::{listen, EventType, Button};
	
	let sender: &_ = Box::leak(Box::new(mem::take(&mut *IM_EVENT_PRODUCER.lock().unwrap()).unwrap()));
	let tokio = tokio::runtime::Handle::current();
	std::thread::spawn(move || {
		let _ = listen(move |event| match event.event_type {
			EventType::ButtonPress(Button::Unknown(x)) => {
				tokio.spawn(async move {
					sender.send(IMPacket {
						event: Event::MouseDown,
						data: x as u32 + 7,
					}).await.unwrap();
				});
			}
			EventType::ButtonRelease(Button::Unknown(x)) => {
				tokio.spawn(async move {
					sender.send(IMPacket {
						event: Event::MouseUp,
						data: x as u32 + 7,
					}).await.unwrap();
				});
			}
			EventType::Wheel { delta_x: _, delta_y } => if delta_y < 0 {
				tokio.spawn(async {
					sender.send(IMPacket {
						event: Event::MouseDown,
						data: 5,
					}).await.unwrap();
					sender.send(IMPacket {
						event: Event::MouseUp,
						data: 5,
					}).await.unwrap();
				});
			} else if delta_y > 0 {
				let sender = sender.clone();
				tokio.spawn(async move {
					sender.send(IMPacket {
						event: Event::MouseDown,
						data: 4,
					}).await.unwrap();
					sender.send(IMPacket {
						event: Event::MouseUp,
						data: 4,
					}).await.unwrap();
				});
			}
			_ => ()
		});
	});
}