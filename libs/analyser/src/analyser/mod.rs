use catastrophic_ast::ast;
use catastrophic_ir::ir;

use self::state::State;

pub use self::error::CompileErrors;

mod error;
mod state;

pub struct Analyser;

impl Analyser {
    pub fn analyse_ast(top_level: ast::Block) -> Result<Vec<ir::Block>, CompileErrors> {
        State::new(top_level).analyse()
    }
}
