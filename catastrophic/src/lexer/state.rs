use crate::span::{Location, Span};

use super::{reader::Continuation, token::Token};

#[derive(Debug, Copy, Clone)]
enum Mode {
    Main,
    Comment,
    Ident,
    Number,
    Minus,
}

pub struct State {
    buffer: String,
    number: u64,
    mode: Mode,
    start: Location,
}

type StateResult = (Option<Span<Token>>, Continuation);

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

            c @ ('a'..='z' | 'A'..='Z') => {
                self.start = input.start;
                self.mode = Mode::Ident;

                self.buffer.clear();
                self.buffer.push(c);
                None
            }

            c @ '0'..='9' => {
                self.start = input.start;
                self.mode = Mode::Number;

                self.number = 0;
                self.number += c as u64 - '0' as u64;
                None
            }

            '-' => {
                self.start = input.start;
                self.mode = Mode::Minus;
                None
            }

            '+' => Some(input.swap(Token::Plus)),

            '=' => Some(input.swap(Token::Equals)),
            '>' => Some(input.swap(Token::GreaterThan)),

            ':' => Some(input.swap(Token::Colon)),
            '?' => Some(input.swap(Token::Question)),

            '{' => Some(input.swap(Token::LCurly)),
            '}' => Some(input.swap(Token::RCurly)),
            '(' => Some(input.swap(Token::LParen)),
            ')' => Some(input.swap(Token::RParen)),

            c => (!c.is_whitespace()).then(|| input.swap(Token::Unexpected(c))),
        };

        (token, Continuation::Consume)
    }

    fn process_comment(&mut self, input: Span<char>) -> StateResult {
        if let '\n' | '\r' = input.data {
            self.mode = Mode::Main
        }

        (None, Continuation::Consume)
    }

    fn process_ident(&mut self, input: Span<char>) -> StateResult {
        if let c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = input.data {
            self.buffer.push(c);
            (None, Continuation::Consume)
        } else {
            self.mode = Mode::Main;
            (Some(Span::new(self.start, input.start, Token::Ident(self.buffer.clone()))), Continuation::Peek)
        }
    }

    fn process_number(&mut self, input: Span<char>) -> StateResult {
        if let c @ '0'..='9' = input.data {
            self.number *= 10;
            self.number += c as u64 - '0' as u64;
            (None, Continuation::Consume)
        } else {
            self.mode = Mode::Main;
            (Some(Span::new(self.start, input.start, Token::Integer(self.number))), Continuation::Peek)
        }
    }

    fn process_minus(&mut self, input: Span<char>) -> StateResult {
        if let '>' = input.data {
            self.mode = Mode::Main;
            (Some(Span::new(self.start, input.end, Token::Arrow)), Continuation::Consume)
        } else {
            self.mode = Mode::Main;
            (Some(Span::new(self.start, input.start, Token::Minus)), Continuation::Peek)
        }
    }

    fn process_state(&mut self, input: Span<char>) -> StateResult {
        match self.mode {
            Mode::Main => self.process_main(input),
            Mode::Comment => self.process_comment(input),
            Mode::Ident => self.process_ident(input),
            Mode::Number => self.process_number(input),
            Mode::Minus => self.process_minus(input),
        }
    }

    pub fn process<Callback: FnMut(Span<Token>)>(&mut self, input: Span<char>, callback: &mut Callback) -> Continuation {
        let (token, continuation) = self.process_state(input);

        if let Some(token) = token {
            callback(token)
        }

        continuation
    }
}
