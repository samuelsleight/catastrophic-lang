use std::collections::{hash_map::Entry, HashMap};

use crate::span::Span;

use super::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub enum Builtin {
    Plus,
    Minus,
    Equals,
    IfThenElse,
}

#[derive(Debug, Clone)]
pub enum SymbolValue {
    Number(u64),
    Block(Block),
    Builtin(Builtin),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name_span: Span<()>,
    value: Span<SymbolValue>,
}

#[derive(Debug, Clone)]
pub enum InstrValue {
    Number(u64),
    Ident(String),
    Block(Block),
    Builtin(Builtin),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Call,
    Push(InstrValue),
}

#[derive(Debug, Clone)]
pub struct Block {
    args: Vec<Span<String>>,
    symbols: HashMap<String, Symbol>,
    instrs: Vec<Span<Instruction>>,
}

impl Symbol {
    pub fn new(name_span: Span<()>, value: Span<SymbolValue>) -> Self {
        Self { name_span, value }
    }
}

impl Block {
    #[must_use]
    pub fn no_args() -> Self {
        Self {
            args: Vec::new(),
            symbols: HashMap::new(),
            instrs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_args(args: Vec<Span<String>>) -> Self {
        Self {
            args,
            symbols: HashMap::new(),
            instrs: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, name: Span<String>, value: Span<SymbolValue>) -> Result<(), ParseError> {
        let name_span = name.swap(());

        match self.symbols.entry(name.data) {
            Entry::Occupied(entry) => Err(ParseError::DuplicateSymbolError {
                first: entry.get().name_span,
                duplicate: name_span,
            }),

            Entry::Vacant(entry) => {
                entry.insert(Symbol::new(name_span, value));
                Ok(())
            }
        }
    }

    pub fn push_instruction(&mut self, instruction: Span<Instruction>) {
        self.instrs.push(instruction);
    }
}
