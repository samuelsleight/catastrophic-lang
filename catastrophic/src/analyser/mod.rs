use crate::parser::ast;

use self::state::State;

pub use self::error::Error;

pub mod ir;

mod error;
mod state;

pub struct Analyser;

impl Analyser {
    pub fn analyse_ast(top_level: ast::Block) -> Result<Vec<ir::Block>, Error> {
        Ok(State::new(top_level).analyse()?)
    }
}
