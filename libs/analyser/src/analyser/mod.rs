use catastrophic_ast::ast;
use catastrophic_hir::hir;

use self::state::State;

pub use self::error::CompileErrors;

mod error;
mod state;

pub struct Analyser;

impl Analyser {
    pub fn analyse_ast(top_level: ast::Block) -> Result<Vec<hir::Block>, CompileErrors> {
        State::new(top_level).analyse()
    }
}
