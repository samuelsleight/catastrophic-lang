use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to open file \"{}\"", .file.display())]
    FileOpen { file: PathBuf, source: std::io::Error },

    #[error("Error reading from file \"{}\"", .file.display())]
    FileRead { file: PathBuf, source: std::io::Error },
}

impl Error {
    #[must_use]
    pub fn file_open(path: PathBuf, source: std::io::Error) -> Self {
        Error::FileOpen { file: path, source }
    }

    #[must_use]
    pub fn file_read(path: PathBuf, source: std::io::Error) -> Self {
        Error::FileRead { file: path, source }
    }
}
