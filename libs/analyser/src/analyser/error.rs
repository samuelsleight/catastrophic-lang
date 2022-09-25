use std::io::{Read, Seek};

use catastrophic_ast::span::Span;
use catastrophic_error::{context::ErrorProvider, writer::ErrorWriter};

#[derive(Debug)]
pub struct CompileErrors {
    errors: Vec<CompileError>,
}

#[derive(Debug)]
pub enum CompileError {
    UndefinedSymbolError(Span<String>),
}

impl From<Vec<CompileError>> for CompileErrors {
    fn from(errors: Vec<CompileError>) -> Self {
        Self { errors }
    }
}

impl ErrorProvider for CompileErrors {
    fn write_errors<R: Read + Seek>(&self, writer: &mut ErrorWriter<R>) -> std::fmt::Result {
        for error in &self.errors {
            match error {
                CompileError::UndefinedSymbolError(ref symbol) => {
                    writer.error(symbol.swap(()), &format!("Use of undefined symbol `{}`", symbol.data))?
                }
            }
        }

        Ok(())
    }
}
