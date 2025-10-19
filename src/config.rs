use native_dialog::{MessageDialogBuilder, MessageLevel};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Clone, Debug, Deserialize)]
pub enum AppConfigError {
    #[error("Failed to load configuration {msg:?}")]
    LoadError { msg: String },
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub watch_path: String,
    pub file_format: String,
    pub delay_seconds: u64,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        envy::from_env::<AppConfig>()
            .map_err(|err| AppConfigError::LoadError {
                msg: err.to_string(),
            })
            .unwrap_or_else(|err| {
                show_error(err);
                std::process::exit(1);
            })
    }
}

fn show_error(error: AppConfigError) {
    MessageDialogBuilder::default()
        .set_level(MessageLevel::Error)
        .set_title("Error")
        .set_text(error)
        .alert()
        .show()
        .unwrap();
}
