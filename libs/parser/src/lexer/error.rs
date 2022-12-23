use std::io::{Read, Seek};

use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    span::Span,
};

#[derive(Debug, Copy, Clone)]
pub enum LexError {
    UnterminatedString(Span<()>),
}

impl ErrorProvider for LexError {
    fn write_errors<R: Read + Seek>(&self, writer: &mut ErrorWriter<R>) -> std::fmt::Result {
        match self {
            LexError::UnterminatedString(span) => writer.error(*span, "Unterminated string literal"),
        }
    }
}
