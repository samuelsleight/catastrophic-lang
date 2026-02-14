use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_llvm::FinishedModule;

pub struct OutputStage;

impl Stage<FinishedModule> for OutputStage {
    type Output = ();
    type Error = NoError;

    fn run(self, _input: FinishedModule, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn name() -> &'static str {
        "Output"
    }

    fn error_context() -> &'static str {
        "Unable to output result"
    }
}

#[derive(Debug)]
pub enum NoError {}

impl ErrorProvider for NoError {
    fn write_errors(&self, _: &mut dyn ErrorWriter) -> std::fmt::Result {
        Ok(())
    }
}
