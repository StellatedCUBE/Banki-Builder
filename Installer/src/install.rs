use std::{fs::{exists, read, write, File}, io::{copy, Cursor}, path::PathBuf};

use anyhow::anyhow;
use cab::{Cabinet, FileReader};
use minreq::get;
use sha256::digest;

pub const STAGES: u32 = 7;

pub fn install_to(mut path: PathBuf, stage: impl Fn(u32)) -> anyhow::Result<()> {
	path.push("bankidaemon.exe");
	let update = exists(&path)?;
	path.pop();

	stage(1);

	let mod_req = get("https://f002.backblazeb2.com/file/shinten/banki-builder/mod.cab").send()?;
	if mod_req.status_code != 200 {
		return Err(anyhow!("Bad net response"));
	}
	let mod_cursor = Cursor::new(mod_req.as_bytes());
	let mut mod_cab = Cabinet::new(mod_cursor)?;

	stage(2);
	
	if !update {
		path.push("Game.exe");
		let og_game = read(&path)?;
		path.pop();

		if digest(&og_game) != "89496ba0462586d66f02f055c7da59390b94ec9c3d15ecd5576e376ed9423d1d" {
			return Err(anyhow!("Bad game executable"));
		}

		stage(3);

		path.push("originalexe.exe");
		write(&path, &og_game)?;
		path.pop();

		stage(4);

		let cab_cursor = Cursor::new(&og_game[55468..74252658]);
		let mut game_cab = Cabinet::new(cab_cursor)?;
		
		let file_data = game_cab.folder_entries()
		.next()
		.unwrap()
		.file_entries()
		.map(|e| (match e.name() {
			"DullahanRecollection.exe" => "Game.exe",
			"data.win" => "originaldata.win",
			name => name
		}.to_owned(), e.uncompressed_offset(), e.uncompressed_size()))
		.collect::<Vec<_>>();
		
		let mut reader = game_cab.read_folder(0)?;
		for (name, offset, size) in file_data {
			reader.seek_to_uncompressed_offset(offset as u64)?;
			path.push(name);
			let mut fr = FileReader {
				reader,
				file_start_in_folder: offset as u64,
				offset: 0,
				size: size as u64,
			};
			copy(&mut fr, &mut File::create(&path)?)?;
			reader = fr.reader;
			path.pop();
		}

		stage(5);
	}

	let Some(folder) = mod_cab.folder_entries().next() else {
		return Err(anyhow!("Bad mod cabinet"))
	};
	
	let file_data = folder
	.file_entries()
	.map(|e| (e.name().to_owned(), e.uncompressed_offset(), e.uncompressed_size()))
	.collect::<Vec<_>>();

	let mut reader = mod_cab.read_folder(0)?;
	for (name, offset, size) in file_data {
		reader.seek_to_uncompressed_offset(offset as u64)?;
		path.push(&name);
		let mut fr = FileReader {
			reader,
			file_start_in_folder: offset as u64,
			offset: 0,
			size: size as u64,
		};
		copy(&mut fr, &mut File::create(&path)?)?;
		reader = fr.reader;
		path.pop();
	}

	stage(6);

	#[cfg(windows)]
	{
		std::fs::copy(path.join("bankibuilder.win"), path.join("data.win"))?;
	}

	#[cfg(not(windows))]
	if !update {
		path.push("data.win");
		let _ = std::fs::remove_file(&path);
		std::os::unix::fs::symlink("bankibuilder.win", &path)?;
	}

	stage(7);

	Ok(())
}