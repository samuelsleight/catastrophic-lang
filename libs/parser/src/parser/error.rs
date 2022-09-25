use catastrophic_ast::span::Span;
use catastrophic_error::context::ErrorProvider;

use crate::lexer;

#[derive(Debug)]
pub enum Error {
    LexError(lexer::Error),
    ParseErrors(ParseErrors),
}

#[derive(Debug)]
pub struct ParseErrors {
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(Span<char>),
    BlockClosedWithoutOpening(Span<()>),
    BlockWithoutClosing(Span<()>),
    LabelWithoutName(Span<()>),
    LabelWithoutValue(Span<()>),
    ArrowWithoutArg(Span<()>),
    ArrowWithoutBlock(Span<()>),
    DuplicateSymbolError { first: Span<()>, duplicate: Span<()> },
}

impl From<Vec<ParseError>> for ParseErrors {
    fn from(errors: Vec<ParseError>) -> Self {
        Self { errors }
    }
}

impl From<ParseErrors> for Error {
    fn from(error: ParseErrors) -> Self {
        Error::ParseErrors(error)
    }
}

impl From<lexer::Error> for Error {
    fn from(error: lexer::Error) -> Self {
        Error::LexError(error)
    }
}

impl ErrorProvider for Error {
    fn write_errors<R: std::io::Read + std::io::Seek>(&self, writer: &mut catastrophic_error::writer::ErrorWriter<R>) -> std::fmt::Result {
        match self {
            Error::LexError(error) => error.write_errors(writer)?,
            Error::ParseErrors(errors) => {
                for error in &errors.errors {
                    match error {
                        ParseError::UnexpectedChar(span) => writer.error(span.swap(()), &format!("Encountered unexpected `{}`", span.data))?,
                        ParseError::BlockClosedWithoutOpening(span) => writer.error(*span, "Encountered `}` with no corresponding `{`")?,
                        ParseError::BlockWithoutClosing(span) => writer.error(*span, "Encountered `{` without corresponding `}`")?,
                        ParseError::LabelWithoutName(span) => writer.error(*span, "Encountered `:` without an accompanying symbol name")?,
                        ParseError::LabelWithoutValue(span) => writer.error(*span, "Encountered `:` without a corresponding symbol value")?,
                        ParseError::ArrowWithoutArg(span) => writer.error(*span, "Encountered `->` without a corresponding argument")?,
                        ParseError::ArrowWithoutBlock(span) => writer.error(*span, "Encountered `->` without a corresponding block")?,
                        ParseError::DuplicateSymbolError { first, duplicate } => {
                            writer.error(*duplicate, "Encountered a duplicate symbol definition")?;
                            writer.note(*first, "Symbol was previously defined here:")?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
