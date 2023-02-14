use std::io::{Read, Seek};

use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_hir::hir;
use catastrophic_mir::mir;

use crate::optimizer::Optimizer;

pub struct OptimizationStage;

impl Stage<Vec<hir::Block>> for OptimizationStage {
    type Output = Vec<mir::Block>;
    type Error = NoError;

    fn run(self, input: Vec<hir::Block>, time_scope: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Ok(Optimizer::optimize_hir(input, time_scope))
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
    fn write_errors<R: Read + Seek>(&self, _: &mut ErrorWriter<R>) -> std::fmt::Result {
        Ok(())
    }
}
