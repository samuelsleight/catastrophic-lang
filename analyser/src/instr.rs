use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
    DividedBy,
    Equals,
    IfElse
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Function(Function),
    Op(Op),
}

#[derive(Clone, Debug)]
pub enum EnvValue {
    Value(Value),
    Arg(usize),
}

#[derive(Clone, Debug)]
pub enum InstrValue {
    Value(Value),
    Ident(String),
}

#[derive(Clone, Debug)]
pub enum Instr {
    Push(InstrValue),
    Apply,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub env: HashMap<String, EnvValue>,
    pub instrs: Vec<Instr>,
}

impl Function {
    pub(crate) fn new() -> Function {
        Function {
            env: Default::default(),
            instrs: Default::default(),
        }
    }
}

impl From<ast::Op> for Op {
    fn from(op: ast::Op) -> Op {
        match op {
            ast::Op::Plus => Op::Plus,
            ast::Op::Minus => Op::Minus,
            ast::Op::Times => Op::Times,
            ast::Op::DividedBy => Op::DividedBy,
            ast::Op::Equals => Op::Equals,
            ast::Op::IfElse => Op::IfElse,
        }
    }
}
