use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherControl {
    Pause,
    Resume,
    Stop,
}

#[derive(Debug, Clone)]
pub struct WatcherHandle {
    pub receiver: Arc<Mutex<Receiver<WatcherControl>>>,
    sender: Sender<WatcherControl>,
}

impl WatcherHandle {
    pub fn pause(&self) {
        if let Err(err) = self.sender.send(WatcherControl::Pause) {
            log::error!("Failed to send pause command: {}", err);
        }
    }

    pub fn resume(&self) {
        if let Err(err) = self.sender.send(WatcherControl::Resume) {
            log::error!("Failed to send resume command: {}", err);
        }
    }

    pub fn stop(&self) {
        if let Err(err) = self.sender.send(WatcherControl::Stop) {
            log::error!("Failed to send stop command: {}", err);
        }
    }
}

impl Default for WatcherHandle {
    fn default() -> Self {
        let (sender, receiver) = channel();

        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}
