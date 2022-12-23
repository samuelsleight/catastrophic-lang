use std::path::Path;

use catastrophic_ast::ast;
use ruinous::parser::{Error as RuinousError, Parser as RuinousParser};

use crate::lexer::State as Lexer;

pub use self::state::State;
pub type Error = RuinousError<Lexer, State>;

mod error;
mod state;
pub struct Parser;

impl Parser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<ast::Block, Error> {
        RuinousParser::parse_file(path, Lexer::new(), State::new())
    }
}
