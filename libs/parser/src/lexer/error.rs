use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    span::Span,
};

#[derive(Debug, Copy, Clone)]
pub enum LexError {
    UnterminatedString(Span<()>),
}

impl ErrorProvider for LexError {
    fn write_errors(&self, writer: &mut dyn ErrorWriter) -> std::fmt::Result {
        match self {
            LexError::UnterminatedString(span) => writer.error(Some(*span), "Unterminated string literal"),
        }
    }
}
