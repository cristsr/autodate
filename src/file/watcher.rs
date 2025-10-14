use crate::file::config::FileWatcherConfig;
use crate::file::handler::{WatcherControl, WatcherHandle};

use notify::event::CreateKind;
use notify::EventKind::Create;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{metadata, Metadata};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};

pub struct FileWatcher {
    receiver: Receiver<notify::Result<Event>>,
    config: FileWatcherConfig,
    file_watcher: Option<RecommendedWatcher>,
    handler: WatcherHandle,
}

impl FileWatcher {
    pub fn new(config: FileWatcherConfig) -> (Self, WatcherHandle) {
        let (sender, receiver) = channel();

        let file_watcher = match RecommendedWatcher::new(sender, Config::default()) {
            Ok(mut watcher) => {
                watcher
                    .watch(Path::new(&config.path), RecursiveMode::Recursive)
                    .unwrap();
                Some(watcher)
            }
            Err(err) => {
                log::error!("Error creating file watcher: {:?}", err);
                None
            }
        };

        let handler = WatcherHandle::default();

        (
            Self {
                config,
                file_watcher,
                receiver,
                handler: handler.clone(),
            },
            handler.clone(),
        )
    }

    pub fn listen(
        &mut self,
        mut callback: impl FnMut(&Path, Metadata) + Send + Sync + 'static,
    ) -> notify::Result<()> {
        log::info!("Starting file watcher listener");

        let mut is_paused = false;

        loop {
            if let Ok(control) = self.handler.receiver.lock()?.try_recv() {
                match control {
                    WatcherControl::Pause => {
                        log::info!("File watcher paused");
                        is_paused = true;
                    }
                    WatcherControl::Resume => {
                        log::info!("File watcher resumed");
                        is_paused = false;
                    }
                    WatcherControl::Stop => {
                        log::info!("File watcher stopped");
                        return Ok(());
                    }
                }
            }

            let event = match self.get_event() {
                Ok(Some(event)) => event,
                Ok(None) => continue,
                Err(err) => return Err(err),
            };

            if is_paused {
                continue;
            }

            if !matches!(event.kind, Create(CreateKind::Any)) {
                continue;
            }

            let Some(path) = event.paths.first() else {
                continue;
            };

            let Some(metadata) = self.get_file_metadata(path) else {
                continue;
            };

            log::info!("New file detected: {}", path.display());

            callback(path, metadata);
        }
    }

    fn get_event(&self) -> Result<Option<Event>, notify::Error> {
        match self.receiver.try_recv() {
            Ok(result) => match result {
                Ok(event) => Ok(Some(event)),
                Err(err) => {
                    log::warn!("File watcher error (continuing): {:?}", err);
                    Ok(None)
                }
            },
            Err(err) => match err {
                TryRecvError::Empty => Ok(None),
                TryRecvError::Disconnected => {
                    log::error!("Event channel disconnected - file watcher stopped");
                    Err(notify::Error::generic("Event channel disconnected"))
                }
            },
        }
    }

    fn get_file_metadata(&self, path: &Path) -> Option<Metadata> {
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
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        if let Some(mut watcher) = self.file_watcher.take() {
            watcher.unwatch(Path::new(&self.config.path)).unwrap();
        }
    }
}
