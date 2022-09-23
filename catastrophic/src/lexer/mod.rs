use std::path::Path;

use crate::span::Span;

use self::{reader::CharReader, state::State};

pub use self::{error::Error, token::Token};

mod error;
mod reader;
mod state;
mod token;

pub struct Lexer;

impl Lexer {
    pub fn lex_file<P: AsRef<Path>, Callback: FnMut(Span<Token>)>(path: P, mut callback: Callback) -> Result<(), Error> {
        let reader = CharReader::from_file(path)?;
        let mut state = State::new();

        reader.read(|span| state.process(span, &mut callback))?;

        Ok(())
    }
}
