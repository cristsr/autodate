use crate::config::AppConfig;
use crate::file::handler::WatcherHandler;
use notify::EventKind::Create;
use notify::event::CreateKind;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{Metadata, metadata};
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

type FileReceiver = Arc<Mutex<Receiver<notify::Result<Event>>>>;

pub struct FileWatcher {
    receiver: FileReceiver,
    config: AppConfig,
    file_watcher: Option<RecommendedWatcher>,
    handler: Arc<WatcherHandler>,
}

impl FileWatcher {
    pub fn new(config: AppConfig) -> Self {
        let (sender, receiver) = channel();

        let file_watcher = match RecommendedWatcher::new(sender, Config::default()) {
            Ok(mut watcher) => {
                watcher
                    .watch(Path::new(&config.watch_path), RecursiveMode::Recursive)
                    .unwrap();
                Some(watcher)
            }
            Err(err) => {
                log::error!("Error creating file watcher: {:?}", err);
                None
            }
        };

        Self {
            config,
            file_watcher,
            receiver: Arc::new(Mutex::new(receiver)),
            handler: Arc::new(WatcherHandler::default()),
        }
    }

    pub fn listen(&self, mut callback: impl FnMut(&Path) + Send + Sync + 'static) {
        log::info!("Starting file watcher listener");

        let handler = self.handler.clone();
        let receiver = self.receiver.clone();

        spawn(move || {
            loop {
                let Some(event) = receive_event(&receiver) else {
                    continue;
                };

                if handler.is_paused() {
                    log::info!("File watcher paused, ignoring event");
                    continue;
                }

                if !matches!(event.kind, Create(CreateKind::Any)) {
                    continue;
                }

                let Some(path) = event.paths.first() else {
                    continue;
                };

                let Some(metadata) = get_file_metadata(path) else {
                    continue;
                };

                if !metadata.is_file() {
                    continue;
                }

                log::info!("New file detected: {}", path.display());

                callback(path);
            }
        });
    }

    pub fn get_handler(&mut self) -> Arc<WatcherHandler> {
        self.handler.clone()
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        if let Some(mut watcher) = self.file_watcher.take() {
            watcher.unwatch(Path::new(&self.config.watch_path)).unwrap();
        }
    }
}

fn receive_event(receiver: &FileReceiver) -> Option<Event> {
    match receiver.lock().unwrap().recv() {
        Ok(result) => match result {
            Ok(event) => Some(event),
            Err(err) => {
                log::warn!("File watcher error (continuing): {:?}", err);
                None
            }
        },
        Err(err) => {
            log::error!(
                "Event channel disconnected - file watcher stopped - {:?}",
                err
            );
            None
        }
    }
}

fn get_file_metadata(path: &Path) -> Option<Metadata> {
    match metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                log::debug!("File detected: {}", path.display());
                Some(metadata)
            } else {
                log::debug!("Ignoring directory: {}", path.display());
                None
            }
        }
        Err(err) => {
            log::error!(
                "Error reading file metadata for {}: {:?}",
                path.display(),
                err
            );
            None
        }
    }
}
