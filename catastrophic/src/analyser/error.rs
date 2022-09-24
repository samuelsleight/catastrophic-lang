use std::fmt::Display;

use thiserror::Error;

use crate::span::Span;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CompilerErrors(#[from] CompileErrors),
}

#[derive(Debug)]
pub struct CompileErrors {
    errors: Vec<CompileError>,
}

#[derive(Debug)]
pub enum CompileError {
    UndefinedSymbolError(Span<String>),
}

impl std::error::Error for CompileErrors {}

impl From<Vec<CompileError>> for CompileErrors {
    fn from(errors: Vec<CompileError>) -> Self {
        Self { errors }
    }
}

impl Display for CompileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            match error {
                // TODO: Pretty error formatting, inclusing highlighting the span in the input
                CompileError::UndefinedSymbolError(span) => writeln!(f, "Undefined symbol {} at {:?}", span.data, span.swap(()))?,
            }
        }

        Ok(())
    }
}
