use catastrophic_core::span::Span;
pub use catastrophic_hir::hir::*;

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    Command(Command),
    Push(Value),
    ImmediateCall(Function),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub offset: usize,
    pub args: usize,
    pub instrs: Vec<Span<Instr>>,
}
