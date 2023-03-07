use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_mir::mir;

use crate::compiler::Compiler;

pub struct CompilationStage;

impl Stage<Vec<mir::Block>> for CompilationStage {
    type Output = ();
    type Error = NoError;

    fn run(self, input: Vec<mir::Block>, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Compiler::compile(input);
        Ok(())
    }

    fn name() -> &'static str {
        "Compilation"
    }

    fn error_context() -> &'static str {
        "Unable to compile input"
    }
}

#[derive(Debug)]
pub enum NoError {}

impl ErrorProvider for NoError {
    fn write_errors(&self, _: &mut dyn ErrorWriter) -> std::fmt::Result {
        Ok(())
    }
}
