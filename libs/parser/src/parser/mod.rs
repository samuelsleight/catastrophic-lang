use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::Path,
};

use catastrophic_ast::ast;
use ruinous::parser::{Error as RuinousError, Parser as RuinousParser};

use crate::lexer::State as Lexer;

use self::state::State;

mod state;
mod test;

pub mod error;

pub type Error = RuinousError<Lexer, State>;

pub struct Parser<R> {
    parser: RuinousParser<R>,
}

impl Parser<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let parser = RuinousParser::from_file(path)?;
        Ok(Self { parser })
    }
}

impl<'a> Parser<Cursor<&'a str>> {
    pub fn from_str(input: &'a str) -> Self {
        let parser = RuinousParser::from_str(input);
        Self { parser }
    }
}

impl<R: BufRead> Parser<R> {
    pub fn parse(self) -> Result<ast::Block, Error> {
        self.parser
            .parse(Lexer::new(), State::new())
    }
}
