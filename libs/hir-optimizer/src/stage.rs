use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_hir::hir;
use catastrophic_mir::mir;

use crate::optimizer::{Optimization, Optimizer};

pub struct OptimizationStage {
    opt: Optimization,
}

impl OptimizationStage {
    pub fn new(opt: Optimization) -> Self {
        Self { opt }
    }
}

impl Stage<Vec<hir::Block>> for OptimizationStage {
    type Output = Vec<mir::Block>;
    type Error = NoError;

    fn run(self, input: Vec<hir::Block>, time_scope: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Ok(Optimizer::optimize_hir(self.opt, input, time_scope))
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
