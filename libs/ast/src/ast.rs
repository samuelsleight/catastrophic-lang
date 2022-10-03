use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

use catastrophic_core::{defines::ValueType, span::Span};

#[derive(Debug, Copy, Clone)]
pub enum Builtin {
    Plus,
    Minus,
    Equals,
    GreaterThan,
    LessThan,
    IfThenElse,
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Call,
    OutputChar,
    OutputNumber,
    InputChar,
    InputNumber,
}

#[derive(Debug, Clone)]
pub enum SymbolValue {
    Number(ValueType),
    Block(Block),
    Builtin(Builtin),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name_span: Span<()>,
    pub value: Span<SymbolValue>,
}

#[derive(Debug, Clone)]
pub enum InstrValue {
    Number(ValueType),
    Ident(String),
    Block(Block),
    Builtin(Builtin),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Command(Command),
    Push(InstrValue),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub args: Vec<Span<String>>,
    pub symbols: HashMap<String, Symbol>,
    pub instrs: Vec<Span<Instruction>>,
}

impl Symbol {
    #[must_use]
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

    pub fn with_symbol(&mut self, name: String) -> Entry<String, Symbol> {
        self.symbols.entry(name)
    }

    pub fn push_instruction(&mut self, instruction: Span<Instruction>) {
        self.instrs.push(instruction);
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Builtin::Plus => f.write_str("+"),
            Builtin::Minus => f.write_str("-"),
            Builtin::Equals => f.write_str("="),
            Builtin::GreaterThan => f.write_str(">"),
            Builtin::LessThan => f.write_str("<"),
            Builtin::IfThenElse => f.write_str("?"),
        }
    }
}