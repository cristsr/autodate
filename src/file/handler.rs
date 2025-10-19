use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct WatcherHandler {
    pause: AtomicBool,
}

impl WatcherHandler {
    pub fn pause(&self) {
        self.pause.store(true, Ordering::Relaxed);
    }

    pub fn resume(&self) {
        self.pause.store(false, Ordering::Relaxed);
    }

    pub fn is_paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }
}

impl Default for WatcherHandler {
    fn default() -> Self {
        Self {
            pause: AtomicBool::new(false),
        }
    }
}
