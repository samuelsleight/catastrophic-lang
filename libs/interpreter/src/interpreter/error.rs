use std::fmt::Display;

use catastrophic_ir::{ir::Builtin, span::Span};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Debug)]
pub enum RuntimeError {
    CalledEmptyStack(Span<()>),
    CalledNumber(Span<()>),
    CalledInvalidBlock(Span<()>),
    InvalidArgsForBuiltin(Span<()>, Builtin),
    InsufficientArgsForFunction(Span<()>),
}

impl std::error::Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: Pretty error formatting, inclusing highlighting the span in the input
            RuntimeError::CalledEmptyStack(span) => writeln!(f, "Attempted to call empty stack at {:?}", span)?,
            RuntimeError::CalledNumber(span) => writeln!(f, "Attempted to call number at {:?}", span)?,
            RuntimeError::CalledInvalidBlock(span) => writeln!(f, "Attempted to call invalid block {:?}", span)?,
            RuntimeError::InvalidArgsForBuiltin(span, builtin) => write!(f, "Invalid args for calling builtin {:?} at {:?}", builtin, span)?,
            RuntimeError::InsufficientArgsForFunction(span) => write!(f, "Insufficient args on stack for function call at {:?}", span)?,
        }

        Ok(())
    }
}
