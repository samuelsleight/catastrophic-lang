use std::ops::Deref;

use catastrophic_core::span::Span;
use catastrophic_mir::mir;

use crate::optimizer::context::OptimizationContext;

use super::OptimizationPass;

pub struct ImmediateConditionalCalls;

impl OptimizationPass for ImmediateConditionalCalls {
    fn run(&self, context: &OptimizationContext) -> Vec<Span<mir::Instr>> {
        let mut instrs: Vec<Span<mir::Instr>> = Vec::with_capacity(context.current_len());

        for instr in context.current_instrs() {
            if let mir::Instr::Command(mir::Command::Call) = instr.data {
                if let Some(last) = instrs.last_mut() {
                    let span = last.swap(());

                    match &last.data {
                        mir::Instr::Push(mir::Value::ImmediateTriOp(mir::TriOp::IfThenElse, value, x, y)) => match (x.deref(), y.deref()) {
                            (&mir::Value::Function(a), &mir::Value::Function(b)) => {
                                *last = span.swap(mir::Instr::ImmediateConditionalCall(value.deref().clone(), a, b));
                            }
                            _ => instrs.push(instr.clone()),
                        },
                        _ => instrs.push(instr.clone()),
                    }
                }
            } else {
                instrs.push(instr.clone());
            }
        }

        instrs
    }
}
