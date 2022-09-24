use std::path::Path;

use crate::lexer::Lexer;

pub use self::error::Error;
use self::{ast::Block, state::State};

pub mod ast;

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
