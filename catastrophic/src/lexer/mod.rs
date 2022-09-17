use std::path::Path;

use crate::span::Span;

use self::{error::Error, reader::CharReader, state::State, token::Token};

mod error;
mod reader;
mod state;
mod token;

pub struct Lexer;

impl Lexer {
    pub fn lex<P: AsRef<Path>, Callback: FnMut(Span<Token>)>(path: P, mut callback: Callback) -> Result<(), Error> {
        let reader = CharReader::from_file(path)?;
        let mut state = State::new();

        reader.read(|span| state.process(span, &mut callback))?;

        Ok(())
    }
}
