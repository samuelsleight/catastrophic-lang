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
    pub fn file_open(path: PathBuf, source: std::io::Error) -> Error {
        Error::FileOpen { file: path, source }
    }

    pub fn file_read(path: PathBuf, source: std::io::Error) -> Error {
        Error::FileRead { file: path, source }
    }
}
