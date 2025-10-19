use crate::config::AppConfig;
use chrono::Local;
use std::iter;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenameError {
    #[error("File has no extension")]
    NoExtension,

    #[error("File has no parent directory")]
    NoParentDirectory,

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Could not find available filename")]
    NoAvailableFilename,
}

pub struct FileRenamer {
    config: AppConfig,
}

impl FileRenamer {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn rename_file(&self, path: &Path) -> Result<PathBuf, RenameError> {
        log::info!("Starting rename process for: {}", path.display());

        thread::sleep(Duration::from_secs(self.config.delay_seconds));

        let parent = path.parent().ok_or(RenameError::NoParentDirectory)?;

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(RenameError::NoExtension)?;

        // Generate new filename with current date
        let date = Local::now().format(&self.config.file_format);

        let new_path = iter::once(format!("{}.{}", date, extension))
            .chain((1..).map(|n| format!("{} ({}).{}", date, n, extension)))
            .map(|filename| parent.join(filename))
            .find(|path| !path.exists())
            .ok_or(RenameError::NoAvailableFilename)?;

        log::debug!("New filename will be: {}", new_path.display());

        // Perform the rename
        std::fs::rename(path, &new_path)?;

        log::info!("File renamed successfully to: {}", new_path.display());

        Ok(new_path)
    }
}
