use crate::config::AppConfig;
use chrono::Local;
use regex::Regex;
use std::iter;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenameError {
    #[error("File has no stem")]
    NoStem,

    #[error("File has no extension")]
    NoExtension,

    #[error("File has no parent directory")]
    NoParentDirectory,

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Could not find available filename")]
    NoAvailableFilename,

    #[error("Invalid date format")]
    InvalidDateFormat,
}

pub struct FileRenamer {
    config: AppConfig,
}

impl FileRenamer {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn rename_file(&self, path: &Path) -> Result<PathBuf, RenameError> {
        thread::sleep(Duration::from_secs(self.config.delay_seconds));

        // Generate new filename with current date
        let parent = path.parent().ok_or(RenameError::NoParentDirectory)?;

        let date = Local::now().format(&self.config.file_format);

        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or(RenameError::NoStem)?;

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(RenameError::NoExtension)?;

        let is_valid_name = Regex::new(&self.config.date_validation)
            .map_err(|_| RenameError::InvalidDateFormat)?
            .is_match(stem);

        if is_valid_name {
            log::info!("File already has a valid date in the name, skipping");
            return Ok(path.to_path_buf());
        }

        let new_path = iter::once(format!("{}.{}", date, extension))
            .chain((1..).map(|n| format!("{} ({}).{}", date, n, extension)))
            .map(|filename| parent.join(filename))
            .find(|path| !path.exists())
            .ok_or(RenameError::NoAvailableFilename)?;

        // Rename file
        std::fs::rename(path, &new_path)?;

        log::info!("File renamed successfully to: {}", new_path.display());

        Ok(new_path)
    }
}
