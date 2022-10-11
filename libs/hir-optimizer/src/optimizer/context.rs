use catastrophic_core::span::Span;
use catastrophic_mir::mir;

pub struct OptimizationContext<'a> {
    current: &'a mir::Block,
}

impl<'a> OptimizationContext<'a> {
    pub fn new(current: &'a mir::Block) -> Self {
        Self { current }
    }

    pub fn current_instrs(&self) -> impl Iterator<Item = &Span<mir::Instr>> {
        self.current.instrs.iter()
    }

    pub fn current_len(&self) -> usize {
        self.current.instrs.len()
    }
}
