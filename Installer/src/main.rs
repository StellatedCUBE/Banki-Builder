// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, mem::forget, sync::mpsc::sync_channel, thread, time::Duration};

use slint::{Timer, TimerMode};

mod backend;
mod install;
mod uninstall;

slint::include_modules!();

enum UIMessage {
    Page(EPage),
    SetPath(String),
    SetValidity(bool),
    SetInstallStatus(EInstallStatus),
    Progress(f32),
    SetErrorMessage(String),
}

enum BackendMessage {
    SetPath(String),
    Browse,
    SelectInstall(String),
    SwitchInstall,
    Install,
    Uninstall,
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(not(windows))]
    if std::env::var("SLINT_SCALE_FACTOR").is_err() {
        unsafe {
            std::env::set_var("SLINT_SCALE_FACTOR", "1");
        }
    }

    let ui = AppWindow::new()?;
    let app = ui.clone_strong();

    ui.set_proton(!cfg!(windows));

    let (send, recv) = sync_channel(4);
    let (send2, recv2) = sync_channel(8);

    let t = Timer::default();
    let send22 = send2.clone();
    t.start(TimerMode::Repeated, Duration::from_millis(16), move || {
        while let Ok(message) = recv.try_recv() {
            update(&app, message);
        }
        let _ = send22.try_send(BackendMessage::SetPath(app.get_path().to_string()));
    });

    thread::spawn(move || backend::run(send, recv2));

    let send22 = send2.clone();
    ui.on_browse(move || {
        send22.send(BackendMessage::Browse).unwrap();
    });

    let app = ui.clone_strong();
    let send22 = send2.clone();
    ui.on_select(move || {
        app.set_install_valid(false);
        send22.send(BackendMessage::SelectInstall(app.get_path().to_string())).unwrap();
    });

    let send22 = send2.clone();
    ui.on_switch_install(move || {
        send22.send(BackendMessage::SwitchInstall).unwrap();
    });

    let send22 = send2.clone();
    ui.on_install(move || {
        send22.send(BackendMessage::Install).unwrap();
    });

    let send22 = send2.clone();
    ui.on_uninstall(move || {
        send22.send(BackendMessage::Uninstall).unwrap();
    });

    let app = ui.clone_strong();
    ui.on_play(move || {
        app.set_can_play(false);
        let _ = open::that("steam://rungameid/1635980");
        let t = Timer::default();
        let app = app.clone_strong();
        t.start(TimerMode::SingleShot, Duration::from_secs(3), move || {
            app.set_can_play(true);
        });
        forget(t);
    });

    ui.on_open_site(|| {
        let _ = open::that("https://banki-builder.shinten.moe/");
    });

    ui.run()?;

    Ok(())
}

fn update(app: &AppWindow, message: UIMessage) {
    match message {
        UIMessage::Page(page) => {
            app.set_page(page);
            app.set_progress(0.0);
        }
        UIMessage::SetPath(path) => app.set_path(path.into()),
        UIMessage::SetValidity(valid) => app.set_install_valid(valid),
        UIMessage::SetInstallStatus(status) => app.set_install_status(status),
        UIMessage::Progress(progress) => app.set_progress(progress),
        UIMessage::SetErrorMessage(message) => app.set_error_message(message.into()),
    }
}