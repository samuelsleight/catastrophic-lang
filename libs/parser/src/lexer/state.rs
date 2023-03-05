use catastrophic_ast::token::Token;
use catastrophic_core::{
    defines::ValueType,
    span::{Location, Span},
};
use ruinous::lexer::state::{Continuation, State as LexerState};
use unic_emoji::char::is_emoji;

use super::error::LexError;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Main,
    Comment,
    Ident,
    String,
    Number,
    Minus,
    LParen,
}

pub struct State {
    buffer: String,
    number: ValueType,
    mode: Mode,
    start: Location,
}

type StateResult = (Option<Span<Token>>, Continuation);

fn is_ident_starter(c: char) -> bool {
    if let 'a'..='z' | 'A'..='Z' | '_' = c {
        true
    } else {
        is_emoji(c)
    }
}

fn is_ident_continuation(c: char) -> bool {
    if let '0'..='9' = c {
        true
    } else {
        is_ident_starter(c)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            number: 0,
            mode: Mode::Main,
            start: Location::default(),
        }
    }

    fn process_main(&mut self, input: Span<char>) -> StateResult {
        let token = match input.data {
            '#' => {
                self.mode = Mode::Comment;
                None
            }

            '"' => {
                self.start = input.start;
                self.mode = Mode::String;

                self.buffer.clear();
                None
            }

            '-' => {
                self.start = input.start;
                self.mode = Mode::Minus;
                None
            }

            '(' => {
                self.start = input.start;
                self.mode = Mode::LParen;
                None
            }

            '+' => Some(input.swap(Token::Plus)),
            '*' => Some(input.swap(Token::Multiply)),
            '/' => Some(input.swap(Token::Divide)),

            '=' => Some(input.swap(Token::Equals)),
            '<' => Some(input.swap(Token::LessThan)),
            '>' => Some(input.swap(Token::GreaterThan)),

            '.' => Some(input.swap(Token::Period)),
            ',' => Some(input.swap(Token::Comma)),

            '&' => Some(input.swap(Token::Ampersand)),
            '~' => Some(input.swap(Token::Tilde)),

            ':' => Some(input.swap(Token::Colon)),
            '?' => Some(input.swap(Token::Question)),

            '{' => Some(input.swap(Token::LCurly)),
            '}' => Some(input.swap(Token::RCurly)),

            c @ '0'..='9' => {
                self.start = input.start;
                self.mode = Mode::Number;

                self.number = 0;
                self.number += c as ValueType - '0' as ValueType;
                None
            }

            c if is_ident_starter(c) => {
                self.start = input.start;
                self.mode = Mode::Ident;

                self.buffer.clear();
                self.buffer.push(c);
                None
            }

            c => (!c.is_whitespace()).then(|| input.swap(Token::Unexpected(c))),
        };

        (token, Continuation::Consume)
    }

    fn process_comment(&mut self, input: Span<char>) -> StateResult {
        if let '\n' | '\r' = input.data {
            self.mode = Mode::Main;
        }

        (None, Continuation::Consume)
    }

    fn process_ident(&mut self, input: Span<char>) -> StateResult {
        if is_ident_continuation(input.data) {
            self.buffer.push(input.data);
            (None, Continuation::Consume)
        } else {
            self.mode = Mode::Main;
            (
                Some(Span::new(self.start, input.start, Token::Ident(self.buffer.clone()))),
                Continuation::Peek,
            )
        }
    }

    fn process_string(&mut self, input: Span<char>) -> StateResult {
        if input.data == '"' {
            self.mode = Mode::Main;
            (
                Some(Span::new(self.start, input.end, Token::String(self.buffer.clone()))),
                Continuation::Consume,
            )
        } else {
            self.buffer.push(input.data);
            (None, Continuation::Consume)
        }
    }

    fn process_number(&mut self, input: Span<char>) -> StateResult {
        if let c @ '0'..='9' = input.data {
            self.number *= 10;
            self.number += c as ValueType - '0' as ValueType;
            (None, Continuation::Consume)
        } else {
            self.mode = Mode::Main;
            (Some(Span::new(self.start, input.start, Token::Integer(self.number))), Continuation::Peek)
        }
    }

    fn process_minus(&mut self, input: Span<char>) -> StateResult {
        self.mode = Mode::Main;

        if input.data == '>' {
            (Some(Span::new(self.start, input.end, Token::Arrow)), Continuation::Consume)
        } else {
            (Some(Span::new(self.start, input.start, Token::Minus)), Continuation::Peek)
        }
    }

    fn process_lparen(&mut self, input: Span<char>) -> StateResult {
        self.mode = Mode::Main;

        if input.data == ')' {
            (Some(Span::new(self.start, input.end, Token::Parens)), Continuation::Consume)
        } else {
            (Some(Span::new(self.start, input.start, Token::Unexpected('('))), Continuation::Peek)
        }
    }

    fn process_state(&mut self, input: Span<char>) -> StateResult {
        match self.mode {
            Mode::Main => self.process_main(input),
            Mode::Comment => self.process_comment(input),
            Mode::Ident => self.process_ident(input),
            Mode::String => self.process_string(input),
            Mode::Number => self.process_number(input),
            Mode::Minus => self.process_minus(input),
            Mode::LParen => self.process_lparen(input),
        }
    }

    pub fn process<Callback: FnMut(Span<Token>)>(&mut self, input: Span<char>, callback: &mut Callback) -> Continuation {
        let (token, continuation) = self.process_state(input);

        if let Some(token) = token {
            callback(token);
        }

        continuation
    }

    pub fn finish(mut self) -> Result<(), LexError> {
        let start = self.start;
        self.start.advance();

        match self.mode {
            Mode::String => Err(LexError::UnterminatedString(Span::new(start, self.start, ()))),
            _ => Ok(()),
        }
    }
}

impl LexerState for State {
    type Token = Token;
    type Error = LexError;

    fn process<Callback: FnMut(Span<Self::Token>)>(&mut self, input: Span<char>, callback: &mut Callback) -> Continuation {
        self.process(input, callback)
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.finish()
    }
}
