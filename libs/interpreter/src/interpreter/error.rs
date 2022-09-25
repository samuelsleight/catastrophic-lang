use std::io::{Read, Seek};

use catastrophic_error::{context::ErrorProvider, writer::ErrorWriter};
use catastrophic_ir::{ir::Builtin, span::Span};

#[derive(Debug, Clone, Copy)]
pub enum RuntimeError {
    CalledEmptyStack(Span<()>),
    CalledNumber(Span<()>),
    CalledInvalidBlock(Span<()>),
    InvalidArgsForBuiltin(Span<()>, Builtin),
    InsufficientArgsForFunction(Span<()>),
}

impl ErrorProvider for RuntimeError {
    fn write_errors<R: Read + Seek>(&self, writer: &mut ErrorWriter<R>) -> std::fmt::Result {
        match *self {
            RuntimeError::CalledEmptyStack(span) => writer.error(span, "Attempted to call a function with an empty stack"),
            RuntimeError::CalledNumber(span) => writer.error(span, "Attampted to call a number instead of a function"),
            RuntimeError::CalledInvalidBlock(span) => writer.error(span, "Attempted to call a block tht does not exist"),
            RuntimeError::InvalidArgsForBuiltin(span, builtin) => {
                writer.error(span, &format!("Invalid args for calling builtin function `{}`", builtin))
            }
            RuntimeError::InsufficientArgsForFunction(span) => writer.error(span, "Attempted to call a function with insufficient arguments"),
        }
    }
}
