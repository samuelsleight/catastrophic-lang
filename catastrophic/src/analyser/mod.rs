use crate::parser::ast;

use self::state::State;

pub mod ir;

mod state;

pub struct Analyser;

impl Analyser {
    pub fn analyse_ast(top_level: ast::Block) -> Vec<ir::Block> {
        State::new(top_level).analyse()
    }
}
