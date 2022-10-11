use catastrophic_core::span::Span;
use catastrophic_mir::mir;

use crate::optimizer::context::OptimizationContext;

use super::OptimizationPass;

pub struct ImmediateCalls;

impl OptimizationPass for ImmediateCalls {
    fn run(&self, context: &OptimizationContext) -> Vec<Span<mir::Instr>> {
        let mut instrs: Vec<Span<mir::Instr>> = Vec::with_capacity(context.current_len());

        for instr in context.current_instrs() {
            if let mir::Instr::Command(mir::Command::Call) = instr.data {
                if let Some(last) = instrs.last_mut() {
                    let span = last.swap(());

                    if let mir::Instr::Push(mir::Value::Function(function)) = last.data {
                        *last = span.swap(mir::Instr::ImmediateCall(function));
                    } else {
                        instrs.push(instr.clone());
                    }
                }
            } else {
                instrs.push(instr.clone());
            }
        }

        instrs
    }
}
