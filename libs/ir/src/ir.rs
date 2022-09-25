use std::collections::HashMap;

pub use catastrophic_ast::ast::{Builtin, Command};
use catastrophic_ast::span::Span;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Arg(usize),
    Block(usize),
    Number(u64),
    Builtin(Builtin),
}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    Command(Command),
    Push(Value),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub offset: usize,
    pub args: usize,
    pub env: Vec<Value>,
    pub symbols: HashMap<String, usize>,
    pub instrs: Vec<Span<Instr>>,
}

impl Block {
    #[must_use]
    pub fn new(args: Vec<Span<String>>, parent: Option<&Block>) -> Self {
        let mut block = parent.map_or_else(
            || Self {
                offset: 0,
                args: args.len(),
                env: Vec::new(),
                symbols: HashMap::new(),
                instrs: Vec::new(),
            },
            |parent| Self {
                offset: parent.offset + parent.args,
                args: args.len(),
                env: parent.env.clone(),
                symbols: parent.symbols.clone(),
                instrs: Vec::new(),
            },
        );

        for (index, arg) in args.into_iter().enumerate() {
            block.symbols.insert(arg.data, block.env.len());
            block.env.push(Value::Arg(block.offset + index));
        }

        block
    }

    pub fn push_symbol(&mut self, name: String, value: Value) {
        let index = self.env.len();
        self.env.push(value);
        self.symbols.insert(name, index);
    }

    pub fn push_instr(&mut self, instr: Span<Instr>) {
        self.instrs.push(instr);
    }

    #[must_use]
    pub fn lookup_symbol(&self, name: &String) -> Option<Value> {
        self.symbols.get(name).map(|index| self.env[*index])
    }
}
