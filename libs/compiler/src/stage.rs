use std::path::PathBuf;

use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_llvm::FinishedModule;
use catastrophic_mir::mir;

use crate::compiler::Compiler;

pub struct CompilationStage {
    source_filename: PathBuf,
}

impl CompilationStage {
    pub fn new(source_filename: PathBuf) -> Self {
        Self { source_filename }
    }
}

impl Stage<Vec<mir::Block>> for CompilationStage {
    type Output = FinishedModule;
    type Error = NoError;

    fn run(self, input: Vec<mir::Block>, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Ok(Compiler::compile(input, self.source_filename))
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
