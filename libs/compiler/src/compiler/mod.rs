use std::path::Path;

use catastrophic_llvm::FinishedModule;
use catastrophic_mir::mir::Block;

use self::state::State;

mod state;

pub struct Compiler;

impl Compiler {
    #[must_use]
    pub fn compile<P: AsRef<Path>>(ir: Vec<Block>, source_filename: P) -> FinishedModule {
        State::new(ir, source_filename.as_ref()).compile()
    }
}
