use std::path::Path;

use catastrophic_ast::{span::Span, token::Token};

use self::{reader::CharReader, state::State};

pub use self::error::Error;

mod error;
mod reader;
mod state;

pub struct Lexer;

impl Lexer {
    pub fn lex_file<P: AsRef<Path>, Callback: FnMut(Span<Token>)>(path: P, mut callback: Callback) -> Result<(), Error> {
        let reader = CharReader::from_file(path)?;
        let mut state = State::new();

        reader.read(|span| state.process(span, &mut callback))?;

        Ok(state.finish()?)
    }
}
