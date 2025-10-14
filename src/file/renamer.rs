use crate::file::config::FileRenamerConfig;
use chrono::Local;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum RenameError {
    NoExtension,
    NoParentDirectory,
    IoError(std::io::Error),
}

impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenameError::NoExtension => write!(f, "File has no extension"),
            RenameError::NoParentDirectory => write!(f, "File has no parent directory"),
            RenameError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for RenameError {}

impl From<std::io::Error> for RenameError {
    fn from(err: std::io::Error) -> Self {
        RenameError::IoError(err)
    }
}

pub struct FileRenamer {
    config: FileRenamerConfig,
}

impl FileRenamer {
    pub fn new(config: FileRenamerConfig) -> Self {
        Self { config }
    }

    pub fn rename_file(&self, path: &Path) -> Result<PathBuf, RenameError> {
        log::info!("Starting rename process for: {}", path.display());

        // Wait before renaming to ensure file is fully written
        if self.config.delay_seconds > 0 {
            log::debug!(
                "Waiting {} seconds before rename",
                self.config.delay_seconds
            );
            thread::sleep(Duration::from_secs(self.config.delay_seconds));
        }

        // Get the file extension
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(RenameError::NoExtension)?;

        // Generate new filename with current date
        let date = Local::now().format(&self.config.date_format);
        let new_filename = format!("{}.{}", date, extension);

        log::debug!("New filename will be: {}", new_filename);

        // Get the parent directory
        let parent = path.parent().ok_or(RenameError::NoParentDirectory)?;
        let new_path = parent.join(new_filename);

        log::info!("Renaming {} to {}", path.display(), new_path.display());

        // Perform the rename
        std::fs::rename(path, &new_path)?;

        log::info!("File renamed successfully");

        Ok(new_path)
    }
}

impl Default for FileRenamer {
    fn default() -> Self {
        let config = FileRenamerConfig::default();
        Self::new(config)
    }
}
