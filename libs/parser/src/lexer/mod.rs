use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::Path,
};

use catastrophic_ast::token::Token;
use catastrophic_core::span::Span;
use ruinous::lexer::{Error as RuinousError, Lexer as RuinousLexer};

pub(crate) use self::state::State;

pub type Error = RuinousError<State>;

mod error;
mod state;

pub struct Lexer<R> {
    lexer: RuinousLexer<R>,
}

impl Lexer<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let lexer = RuinousLexer::from_file(path)?;
        Ok(Self { lexer })
    }
}

impl<'a> Lexer<Cursor<&'a str>> {
    pub fn from_str(input: &'a str) -> Self {
        let lexer = RuinousLexer::from_str(input);
        Self { lexer }
    }
}

impl<R: BufRead> Lexer<R> {
    pub fn lex<Callback: FnMut(Span<Token>)>(self, callback: Callback) -> Result<(), Error> {
        self.lexer.lex(State::new(), callback)
    }

    pub fn collect(self) -> Result<Vec<Span<Token>>, Error> {
        let mut vec = Vec::new();
        self.lex(|token| vec.push(token))?;
        Ok(vec)
    }
}
