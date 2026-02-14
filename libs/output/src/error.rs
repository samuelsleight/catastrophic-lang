use std::{fmt, io};

use catastrophic_core::error::{context::ErrorProvider, writer::ErrorWriter};

#[derive(Debug)]
pub enum LinkerError {
    IoError(io::Error),
    LinkerError(String),
}

#[derive(Debug)]
pub enum OutputError {
    CompileError(String),
    LinkerError(LinkerError),
}

impl ErrorProvider for OutputError {
    fn write_errors(&self, writer: &mut dyn ErrorWriter) -> fmt::Result {
        match self {
            OutputError::CompileError(message) | OutputError::LinkerError(LinkerError::LinkerError(message)) => writer.error(None, message),
            OutputError::LinkerError(LinkerError::IoError(error)) => writer.error(None, &format!("{error}")),
        }
    }
}
