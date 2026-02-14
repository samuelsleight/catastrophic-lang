use std::path::PathBuf;

use catastrophic_llvm::FinishedModule;
use catastrophic_mir::mir::Block;

use self::state::State;

mod state;

pub struct Compiler;

impl Compiler {
    pub fn compile(ir: Vec<Block>, source_filename: PathBuf) -> FinishedModule {
        State::new(ir, source_filename).compile()
    }
}
