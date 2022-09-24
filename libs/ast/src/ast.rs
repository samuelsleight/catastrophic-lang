use std::collections::{hash_map::Entry, HashMap};

use catastrophic_span::span::Span;

#[derive(Debug, Copy, Clone)]
pub enum Builtin {
    Plus,
    Minus,
    Equals,
    LessThan,
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
    pub name_span: Span<()>,
    pub value: Span<SymbolValue>,
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
