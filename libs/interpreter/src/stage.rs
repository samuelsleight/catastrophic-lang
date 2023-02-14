use catastrophic_core::{profiling::TimeScope, stage::Stage};
use catastrophic_hir::hir;

use crate::interpreter::{Interpreter, RuntimeError};

pub struct InterpreterStage;

impl Stage<Vec<hir::Block>> for InterpreterStage {
    type Output = ();
    type Error = RuntimeError;

    fn run(self, input: Vec<hir::Block>, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Interpreter::interpret(input)
    }

    fn name() -> &'static str {
        "Runtime"
    }

    fn error_context() -> &'static str {
        "Runtime error"
    }
}
