use std::path::Path;

use catastrophic_ast::token::Token;
use catastrophic_core::span::Span;
use ruinous::lexer::{Error as RuinousError, Lexer as RuinousLexer};

pub use self::state::State;
pub type Error = RuinousError<State>;

mod error;
mod state;

pub struct Lexer;

impl Lexer {
    pub fn lex_file<P: AsRef<Path>, Callback: FnMut(Span<Token>)>(path: P, callback: Callback) -> Result<(), Error> {
        RuinousLexer::lex_file(path, State::new(), callback)
    }
}
