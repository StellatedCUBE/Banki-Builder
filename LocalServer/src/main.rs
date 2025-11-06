#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;

#[cfg(not(feature = "verify"))]
#[tokio::main]
async fn main() {
	#[cfg(windows)]
	{
		use std::env::args_os;
		use std::{process::exit, thread, time::Duration};
		use windows_sys::Win32::System::Threading::WaitForSingleObject;
		use windows_sys::Win32::System::Threading::OpenProcess;

		println!("Daemon start");

		let mut args = args_os();
		controller::fs::set_save_directory(args.next().unwrap().into());
		controller::net::user::SELF.store(args.next().unwrap().to_str().unwrap().parse().unwrap(), std::sync::atomic::Ordering::Relaxed);

		unsafe {
			let pid = args.next().unwrap().to_str().unwrap().parse().unwrap();

			thread::spawn(move || {
				let parent = OpenProcess(
					1048576u32,
					false as i32,
					pid
				);

				WaitForSingleObject(parent, 4294967295u32);
				println!("Parent has died");
				thread::sleep(Duration::from_secs(5));
				exit(0);
			});
		}
	}

	#[cfg(not(windows))]
	{
		controller::fs::set_save_directory("/home/gemdude46/repos/games/banki/savedata".into());
	}

	controller::run().await;
}

#[cfg(feature = "verify")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
	use std::{env::args, io::stdout, path::PathBuf};
	use controller::level::Level;
	use std::fs::File;
	use banki_common::publish_level::VerificationResponse;

	let mut args = args();
	args.next();
	let path: PathBuf = args.next().unwrap().into();
	let mut level = Level::load_from_file(path.clone()).await.unwrap();
	level.online_id = args.next().unwrap().parse().unwrap();
	level.author = args.next().unwrap().parse().unwrap();
	level.prepare();
	level.serialize(&mut File::create(path).unwrap()).unwrap();

	let character_bit = 1 << level.character() as u8;
	let metadata_buf = bincode::encode_to_vec(level.metadata(), bincode::config::standard()).unwrap();
	bincode::encode_into_std_write(VerificationResponse {
		name: level.name,
		tags: level.tags,
		character_bit,
		theme_bit: 1 << level.theme as u32,
		metadata_buf,
	}, &mut stdout(), bincode::config::standard()).unwrap();
}
