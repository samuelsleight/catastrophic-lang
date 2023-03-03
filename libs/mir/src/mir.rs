use catastrophic_core::{defines::ValueType, span::Span};
pub use catastrophic_hir::hir::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Equals,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TriOp {
    IfThenElse,
}

#[derive(Debug, Copy, Clone)]
pub enum Function {
    Block(usize),
    BinOp(BinOp),
    TriOp(TriOp),
}

#[derive(Debug, Clone)]
pub enum Value {
    Arg(usize),
    Number(ValueType),
    Function(Function),
    ImmediateBinOp(BinOp, Box<Value>, Box<Value>),
    ImmediateTriOp(TriOp, Box<Value>, Box<Value>, Box<Value>),
}

#[derive(Debug, Clone)]
pub enum Instr {
    Command(Command),
    Push(Value),
    ImmediateCall(Function),
    ImmediateConditionalCall(Value, Function, Function),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub offset: usize,
    pub args: usize,
    pub instrs: Vec<Span<Instr>>,
}
