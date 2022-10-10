use catastrophic_core::span::Span;
use catastrophic_hir::hir::Command;
use catastrophic_mir::mir::{Block, Instr, Value};

use super::OptimizationPass;

pub struct ImmediateCalls;

impl OptimizationPass for ImmediateCalls {
    fn run(&self, block: &mut Block) {
        let mut instrs: Vec<Span<Instr>> = Vec::with_capacity(block.instrs.len());

        for instr in &block.instrs {
            if let Instr::Command(Command::Call) = instr.data {
                if let Some(last) = instrs.last_mut() {
                    let span = last.swap(());

                    if let Instr::Push(Value::Function(function)) = last.data {
                        *last = span.swap(Instr::ImmediateCall(function));
                    } else {
                        instrs.push(*instr);
                    }
                }
            } else {
                instrs.push(*instr);
            }
        }

        block.instrs = instrs;
    }
}
