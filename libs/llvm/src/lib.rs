use std::fmt::{self, Debug, Write};

use catastrophic_core::pretty::{PrettyDebug, PrettyFormatter};

pub use dragon_tamer as llvm;

#[derive(Debug)]
pub struct FinishedModule(llvm::Module);

impl FinishedModule {
    #[must_use]
    pub fn new(module: llvm::Module) -> Self {
        Self(module)
    }

    pub fn compile_for_host(&self, output: llvm::CompileOutput) -> Result<llvm::OutputFile, String> {
        self.0.compile_for_host(output)
    }
}

impl PrettyDebug for FinishedModule {
    fn pretty_debug(&self, formatter: &mut PrettyFormatter) -> fmt::Result {
        write!(formatter, "{:?}", &self.0)
    }
}
