use catastrophic_ir::ir;

use self::state::State;

pub use self::error::Error;

mod error;
mod state;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(ir: Vec<ir::Block>) -> Result<(), Error> {
        Ok(State::new(ir).interpret()?)
    }
}
