use crate::config::AppConfig;
use crate::file::renamer::FileRenamer;
use crate::file::watcher::FileWatcher;
use crate::tray::events::TrayEvent;
use crate::tray::menu::TrayMenu;
use crate::tray::runner::TrayRunner;
use crate::tray::tray::Tray;
use std::sync::Arc;

pub struct App {
    tray_runner: TrayRunner,
    file_watcher: FileWatcher,
    config: Arc<AppConfig>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let menu = TrayMenu::default();
        let tray = Tray::new(&menu);

        Self {
            tray_runner: TrayRunner::new(tray, menu),
            file_watcher: FileWatcher::new(config.clone()),
            config: Arc::new(config),
        }
    }

    pub fn listen_files(&mut self) -> &mut Self {
        let config = self.config.clone();

        self.file_watcher.listen(move |path| {
            let file_renamer = FileRenamer::new((*config).clone());

            match file_renamer.rename_file(&path) {
                Ok(new_path) => {
                    log::info!("File renamed successfully: {}", new_path.display());
                }
                Err(err) => {
                    log::error!("Failed to rename file {}: {}", path.display(), err);
                }
            }
        });

        self
    }

    pub fn run_tray(&mut self) -> &mut Self {
        let config = self.config.clone();
        let file_handler = self.file_watcher.get_handler();

        self.tray_runner.run(move |event, tray_ref| match event {
            TrayEvent::Title => {
                log::info!("Opening monitored folder");

                if let Err(e) = open::that(config.watch_path.clone()) {
                    log::error!("Failed to open folder: {}", e);
                }
            }
            TrayEvent::Running => {
                let is_running = tray_ref.is_running();

                if is_running {
                    log::info!("Application running");
                    file_handler.resume();
                } else {
                    log::info!("Application paused");
                    file_handler.pause();
                }
            }
            TrayEvent::Exit => {
                log::info!("Application exiting");
            }
        });

        self
    }
}
