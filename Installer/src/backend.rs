use std::{fs::{exists, File}, io::Read, path::{Path, PathBuf}, sync::mpsc::{Receiver, SyncSender}, thread::sleep, time::Duration};

use minreq::get;
use rfd::FileDialog;
use sha256::digest;

use crate::{install::{self, install_to}, uninstall::uninstall, BackendMessage, EInstallStatus, UIMessage};

const INSTALLER_VERSION: u16 = 1;

// === LINUX ===

#[cfg(not(windows))]
fn get_prior_good_install_inner() -> Option<PathBuf> {
	dirs::config_dir().map(|mut path| {
		path.push("banki-builder");
		path
	})
}

#[cfg(not(windows))]
fn get_prior_good_install() -> Option<PathBuf> {
	get_prior_good_install_inner().map(|path| std::fs::read_link(&path).unwrap_or(path))
}

#[cfg(not(windows))]
fn get_default_path() -> Option<PathBuf> {
	dirs::data_dir().map(|mut path| {
		path.push("Steam/steamapps/common/クビナシリコレクション");
		path
	})
}

#[cfg(not(windows))]
fn save_path(path: &Path) {
	if let Some(slp) = get_prior_good_install_inner() {
		let _ = std::fs::remove_file(&slp);
		let _ = std::os::unix::fs::symlink(path, slp);
	}
}

// ===

// === WINDOWS ===

#[cfg(windows)]
const REG_KEY: &'static str = "software\\StellatedCUBE\\BankiBuilder";

#[cfg(windows)]
fn get_prior_good_install() -> Option<PathBuf> {
	windows_registry::CURRENT_USER.open(REG_KEY).ok().and_then(|k| k.get_hstring("kr_install").ok()).map(|h| PathBuf::from(h.to_os_string()))
}

#[cfg(windows)]
fn get_default_path() -> Option<PathBuf> {
	Some(PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\クビナシリコレクション"))
}

#[cfg(windows)]
fn save_path(path: &Path) {
	if let Ok(key) = windows_registry::CURRENT_USER.create(REG_KEY) {
		let _ = key.set_hstring("kr_install", &windows_registry::HSTRING::from(path.as_os_str()));
	}
}

// ===

fn install_status_at(mut path: PathBuf, latest_version: u16) -> EInstallStatus {
	if !path.is_absolute() {
		return EInstallStatus::Invalid;
	}

	path.push("Game.exe");
	let Ok(mut file) = File::open(&path) else { return EInstallStatus::Invalid };
	let mut buf = vec![0u8; 1 << 20];
	if file.read_exact(&mut buf).is_err() {
		return EInstallStatus::Invalid;
	}

	if digest(&buf) == "6ac0e70dd5db5cde70d6d1dce604ba5f1aefbf859ef55b0b32b9df866a1e1a02" {
		return EInstallStatus::Vanilla;
	}
	path.pop();

	path.push("bankidaemon.exe");
	if let Ok(mut file) = File::open(&path) {
		let mut buf = vec![0u8; 0x54];
		if file.read_exact(&mut buf).is_ok() && buf[0x50..0x52] == *b"Bb" {
			return if u16::from_le_bytes(buf[0x52..0x54].try_into().unwrap()) < latest_version {
				EInstallStatus::OutOfDate
			} else {
				EInstallStatus::BankiBuilder
			}
		}
	}
	
	EInstallStatus::Invalid
}

pub fn run(send: SyncSender<UIMessage>, recv: Receiver<BackendMessage>) {
	let latest_version = if let Some(version_data) = get("https://f002.backblazeb2.com/file/shinten/banki-builder/ver.dat")
	.with_timeout(8)
	.send()
	.ok()
	.filter(|r| r.status_code == 200 && r.as_bytes().len() == 4)
	.map(|r| r.into_bytes()) {
		let installer_version = u16::from_le_bytes(version_data[2..4].try_into().unwrap());
		if installer_version > INSTALLER_VERSION {
			send.send(UIMessage::Page(crate::EPage::OutOfDateInstaller)).unwrap();
			return;
		}
		u16::from_le_bytes(version_data[0..2].try_into().unwrap())
	} else { 0 };

	if let Some((path, status)) = get_prior_good_install().and_then(|path| match install_status_at(path.clone(), latest_version) {
		EInstallStatus::Invalid => None,
		status => Some((path, status))
	}) {
		send.send(UIMessage::SetInstallStatus(status)).unwrap();
		send.send(UIMessage::SetPath(path.to_string_lossy().into_owned())).unwrap();
		send.send(UIMessage::Page(crate::EPage::ManageInstall)).unwrap();
	}

	else {
		if let Some(default) = get_default_path() {
			if install_status_at(default.clone(), latest_version) != EInstallStatus::Invalid {
				if let Some(string) = default.as_os_str().to_str() {
					send.send(UIMessage::SetPath(string.to_owned())).unwrap();
				}
			}
		}

		send.send(UIMessage::Page(crate::EPage::SelectInstall)).unwrap();
	}

	let mut last_path = String::new();

	loop {
		match recv.recv().unwrap() {
			BackendMessage::SetPath(path) => if last_path != path {
				last_path = path.clone();
				send.send(UIMessage::SetValidity(install_status_at(PathBuf::from(&last_path), latest_version) != EInstallStatus::Invalid)).unwrap();
			}

			BackendMessage::Browse => {
				let mut fd = FileDialog::new().add_filter("Executable", &["exe"]).set_can_create_directories(false);
				let path = PathBuf::from(&last_path);

				if exists(&path).is_ok_and(|x| x) {
					fd = fd.set_directory(path);
				}

				if let Some(mut result) = fd.pick_file() {
					result.pop();
					if let Some(string) = result.as_os_str().to_str() {
						send.send(UIMessage::SetPath(string.to_owned())).unwrap();
					}
				}
			}

			BackendMessage::SelectInstall(path) => {
				let path = PathBuf::from(path);
				let status = install_status_at(path.clone(), latest_version);
				if status != EInstallStatus::Invalid {
					send.send(UIMessage::SetInstallStatus(status)).unwrap();
					send.send(UIMessage::Page(crate::EPage::ManageInstall)).unwrap();
					save_path(&path);
				}
			}

			BackendMessage::SwitchInstall => send.send(UIMessage::Page(crate::EPage::SelectInstall)).unwrap(),

			BackendMessage::Install => {
				send.send(UIMessage::Page(crate::EPage::Installing)).unwrap();
				let send2 = send.clone();
				match install_to(PathBuf::from(&last_path), move |stage| {
					let _ = send2.send(UIMessage::Progress(stage as f32 / install::STAGES as f32));
				}) {
					Ok(()) => {
						let status = install_status_at(PathBuf::from(&last_path), latest_version);
						sleep(Duration::from_millis(250));
						send.send(UIMessage::SetInstallStatus(status)).unwrap();
						send.send(UIMessage::Page(crate::EPage::ManageInstall)).unwrap();
					}

					Err(err) => {
						send.send(UIMessage::SetErrorMessage(err.to_string())).unwrap();
						send.send(UIMessage::Page(crate::EPage::Error)).unwrap();
					}
				}
			}

			BackendMessage::Uninstall => {
				send.send(UIMessage::Page(crate::EPage::Loading)).unwrap();
				match uninstall(&PathBuf::from(&last_path)) {
					Ok(()) => {
						let status = install_status_at(PathBuf::from(&last_path), latest_version);
						send.send(UIMessage::SetInstallStatus(status)).unwrap();
						send.send(UIMessage::Page(crate::EPage::ManageInstall)).unwrap();
					}

					Err(err) => {
						let status = install_status_at(PathBuf::from(&last_path), latest_version);
						send.send(UIMessage::SetInstallStatus(status)).unwrap();
						send.send(UIMessage::SetErrorMessage(err.to_string())).unwrap();
						send.send(UIMessage::Page(crate::EPage::Error)).unwrap();
					}
				}
			}
		}
	}
}
