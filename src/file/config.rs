pub struct FileRenamerConfig {
    pub delay_seconds: u64,
    pub date_format: String,
}

impl Default for FileRenamerConfig {
    fn default() -> Self {
        Self {
            delay_seconds: 5,
            date_format: "%Y-%m".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileWatcherConfig {
    pub path: String,
}
