use catastrophic_hir::hir;

use self::state::State;

pub use self::error::RuntimeError;

mod error;
mod state;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(ir: Vec<hir::Block>) -> Result<(), RuntimeError> {
        State::new(ir).interpret()
    }
}
