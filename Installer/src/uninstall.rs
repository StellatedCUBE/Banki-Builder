use std::{fs::{read_dir, remove_file, rename}, io::Result, path::Path};

pub fn uninstall(path: &Path) -> Result<()> {
	rename(path.join("originalexe.exe"), path.join("Game.exe"))?;
	
	for file in read_dir(path)? {
		let file = file?;
		if !file.file_type()?.is_dir() &&
		!file.file_name().eq_ignore_ascii_case("game.exe") &&
		!file.file_name().to_ascii_lowercase().to_string_lossy().ends_with(".txt") {
			remove_file(file.path())?;
		}
	}

	Ok(())
}