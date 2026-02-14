use std::{path::PathBuf, process::Command};

use catastrophic_core::{
    error::{context::ErrorProvider, writer::ErrorWriter},
    profiling::TimeScope,
    stage::Stage,
};
use catastrophic_llvm::{llvm::CompileOutput, FinishedModule};

use crate::error::{LinkerError, OutputError};

pub struct OutputStage {
    output: PathBuf,
}

impl OutputStage {
    #[must_use]
    pub fn new(output: PathBuf) -> Self {
        Self { output }
    }
}

impl Stage<FinishedModule> for OutputStage {
    type Output = ();
    type Error = OutputError;

    fn run(self, input: FinishedModule, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        let output_file = input
            .compile_for_host(CompileOutput::Object)
            .map_err(OutputError::CompileError)?;

        let linker_result = Command::new("cc")
            .arg(output_file.path())
            .arg("-o")
            .arg(self.output)
            .arg("-no-pie")
            .output()
            .map_err(LinkerError::IoError)
            .map_err(OutputError::LinkerError)?;

        if linker_result.status.success() {
            Ok(())
        } else {
            Err(OutputError::LinkerError(LinkerError::LinkerError(
                String::from_utf8(linker_result.stderr).unwrap(),
            )))
        }
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
