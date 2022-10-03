use std::{
    io::{Read, Seek},
    path::PathBuf,
};

use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    span::Span,
};

#[derive(Debug)]
pub enum Error {
    FileOpen { file: PathBuf, source: std::io::Error },
    FileRead { file: PathBuf, source: std::io::Error },
    LexError(LexError),
}

#[derive(Debug, Copy, Clone)]
pub enum LexError {
    UnterminatedString(Span<()>),
}

impl From<LexError> for Error {
    fn from(error: LexError) -> Self {
        Error::LexError(error)
    }
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

impl ErrorProvider for Error {
    fn write_errors<R: Read + Seek>(&self, writer: &mut ErrorWriter<R>) -> std::fmt::Result {
        match self {
            Error::FileOpen { file, source } => writer.error(None, &format!("Unable to open file `{}`: {}", file.display(), source)),
            Error::FileRead { file, source } => writer.error(None, &format!("Unable to read file `{}`: {}", file.display(), source)),
            Error::LexError(error) => match *error {
                LexError::UnterminatedString(span) => writer.error(span, "Unterminated string literal"),
            },
        }
    }
}
