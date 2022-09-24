use catastrophic_ast::ast;
use catastrophic_ir::ir;

use self::state::State;

pub use self::error::Error;

mod error;
mod state;

pub struct Analyser;

impl Analyser {
    pub fn analyse_ast(top_level: ast::Block) -> Result<Vec<ir::Block>, Error> {
        Ok(State::new(top_level).analyse()?)
    }
}
