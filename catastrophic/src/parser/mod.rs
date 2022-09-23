use std::path::Path;

use crate::lexer::Lexer;

use self::state::State;
pub use self::{ast::Block, error::Error};

mod ast;
mod error;
mod state;
pub struct Parser;

impl Parser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Block, Error> {
        let mut state = State::new();

        Lexer::lex_file(path, |token| state.process(token))?;

        Ok(state.finish()?)
    }
}
