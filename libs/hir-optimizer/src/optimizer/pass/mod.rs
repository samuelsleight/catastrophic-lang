use catastrophic_core::span::Span;
use catastrophic_mir::mir;

use self::{immediate_calls::ImmediateCalls, immediate_operations::ImmediateOperations};

use super::context::OptimizationContext;

mod immediate_calls;
mod immediate_operations;

pub trait OptimizationPass {
    fn run(&self, context: &OptimizationContext) -> Vec<Span<mir::Instr>>;
}

pub fn passes() -> Vec<Box<dyn OptimizationPass>> {
    vec![Box::new(ImmediateCalls), Box::new(ImmediateOperations)]
}
