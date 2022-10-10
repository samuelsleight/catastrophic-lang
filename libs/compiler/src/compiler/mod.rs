use catastrophic_hir::hir::Block;

use self::state::State;

mod state;

pub struct Compiler;

impl Compiler {
    pub fn compile(ir: Vec<Block>) {
        State::new(ir).compile()
    }
}
