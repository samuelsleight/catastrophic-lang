use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_hir::hir;
use catastrophic_mir::mir;

use crate::optimizer::{Optimizer, Options};

pub struct OptimizationStage {
    options: Options,
}

impl OptimizationStage {
    #[must_use]
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    #[must_use]
    pub fn pass_names() -> Vec<&'static str> {
        Optimizer::pass_names()
    }
}

impl Stage<Vec<hir::Block>> for OptimizationStage {
    type Output = Vec<mir::Block>;
    type Error = NoError;

    fn run(self, input: Vec<hir::Block>, time_scope: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Ok(Optimizer::optimize_hir(self.options, input, time_scope))
    }

    fn name() -> &'static str {
        "Optimization"
    }

    fn error_context() -> &'static str {
        "Unable to optimize input"
    }
}

#[derive(Debug)]
pub enum NoError {}

impl ErrorProvider for NoError {
    fn write_errors(&self, _: &mut dyn ErrorWriter) -> std::fmt::Result {
        Ok(())
    }
}
