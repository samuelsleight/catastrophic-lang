use std::fmt::Display;

use thiserror::Error;

use crate::{lexer, span::Span};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LexError(#[from] lexer::Error),

    #[error(transparent)]
    ParseErrors(#[from] ParseErrors),
}

#[derive(Debug)]
pub struct ParseErrors {
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    LabelWithoutName(Span<()>),
    ArrowWithoutArg(Span<()>),
    DuplicateSymbolError { first: Span<()>, duplicate: Span<()> },
}

impl std::error::Error for ParseErrors {}

impl From<Vec<ParseError>> for ParseErrors {
    fn from(errors: Vec<ParseError>) -> Self {
        Self { errors }
    }
}

impl Display for ParseErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            match error {
                // TODO: Pretty error formatting, inclusing highlighting the span in the input
                ParseError::LabelWithoutName(span) => writeln!(f, "Label without symbol name {:?}", span)?,
                ParseError::ArrowWithoutArg(span) => writeln!(f, "Arrow without argument name {:?}", span)?,
                ParseError::DuplicateSymbolError { first, duplicate } => {
                    writeln!(f, "Duplicate symbol {:?}, first defined at {:?}", duplicate, first)?
                }
            }
        }

        Ok(())
    }
}
