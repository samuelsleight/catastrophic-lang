use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::Path,
};

use catastrophic_ast::ast;
use ruinous::parser::{Error as RuinousError, Parser as RuinousParser};

use crate::lexer::State as Lexer;

use self::state::State;

mod output;
mod state;
mod test;

pub use self::output::ParseOutput;

pub mod error;

pub type Error = RuinousError<Lexer, State>;

pub struct Parser<R> {
    parser: RuinousParser<R>,
    permissive: bool,
}

impl Parser<BufReader<File>> {
    pub fn with_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let parser = RuinousParser::from_file(path)?;
        Ok(Self::new(parser))
    }
}

impl<'a> Parser<Cursor<&'a str>> {
    #[must_use]
    pub fn with_str(input: &'a str) -> Self {
        let parser = RuinousParser::from_str(input);
        Self::new(parser)
    }
}

impl<R> Parser<R> {
    fn new(parser: RuinousParser<R>) -> Self {
        Self { parser, permissive: false }
    }

    #[must_use]
    pub fn permissive(mut self, value: bool) -> Self {
        self.permissive = value;
        self
    }
}

impl<R: BufRead> Parser<R> {
    pub fn parse(self) -> Result<ParseOutput, Error> {
        self.parser
            .parse(Lexer::new(), State::new(self.permissive))
    }
}
