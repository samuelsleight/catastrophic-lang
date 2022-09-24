use std::path::Path;

use catastrophic_ast::ast;

use crate::lexer::Lexer;

pub use self::error::Error;
use self::state::State;

mod error;
mod state;
pub struct Parser;

impl Parser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<ast::Block, Error> {
        let mut state = State::new();

        Lexer::lex_file(path, |token| state.process(token))?;

        Ok(state.finish()?)
    }
}
