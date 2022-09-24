use catastrophic_ir::ir;

use self::state::State;

mod state;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(ir: Vec<ir::Block>) {
        State::new(ir).interpret();
    }
}
