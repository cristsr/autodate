#![windows_subsystem = "windows"]

mod file;
mod tray;

use crate::file::config::FileWatcherConfig;
use crate::file::renamer::FileRenamer;
use crate::file::watcher::FileWatcher;
use crate::tray::menu::TrayMenu;
use crate::tray::runner::TrayRunner;
use crate::tray::tray::Tray;
use crate::tray::types::TrayEvent;
use std::process;
use std::thread::spawn;

fn main() -> notify::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Initializing application");

    let menu = TrayMenu::default();
    let tray = Tray::new(&menu);
    let runner = TrayRunner::new(tray, menu);

    let config = FileWatcherConfig {
        path: "C:\\Users\\styve\\Desktop\\test".to_string(),
    };

    let file_renamer = FileRenamer::default();
    let (mut file_watcher, handler) = FileWatcher::new(config.clone());

    spawn(move || {
        log::info!("File watcher thread started");

        file_watcher
            .listen(move |path, _| {
                match file_renamer.rename_file(path.as_ref()) {
                    Ok(new_path) => {
                        log::info!("File renamed successfully: {}", new_path.display());
                    }
                    Err(err) => {
                        log::error!("Failed to rename file {}: {}", path.display(), err);
                    }
                };

                log::info!("File detected callback: {}", path.display());
            })
            .expect("Error listening for file changes");
    });

    runner.run(move |event, tray_ref| match event {
        TrayEvent::Title => {
            log::info!("Opening monitored folder");

            if let Err(e) = open::that(config.path.clone()) {
                log::error!("Failed to open folder: {}", e);
            }
        }
        TrayEvent::Running => {
            log::info!("Application is currently {} ", tray_ref.is_running());

            let is_running = tray_ref.is_running();
            match is_running {
                true => handler.resume(),
                false => handler.pause(),
            }
        }
        TrayEvent::Exit => {
            log::info!("Application exiting");
            handler.stop();
            process::exit(0);
        }
    });

    Ok(())
}
