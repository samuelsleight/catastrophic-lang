use catastrophic_core::span::Span;
use catastrophic_mir::mir;

use crate::optimizer::context::OptimizationContext;

use super::OptimizationPass;

pub struct ImmediateOperations;

fn run_pass<'a>(input: impl Iterator<Item = &'a Span<mir::Instr>>, len: usize) -> (bool, Vec<Span<mir::Instr>>) {
    let mut instrs: Vec<Span<mir::Instr>> = Vec::with_capacity(len);
    let mut changes_made = false;

    for instr in input {
        if instrs.len() < 2 {
            instrs.push(instr.clone());
            continue;
        }

        let handled = match instr.data {
            mir::Instr::ImmediateCall(mir::Function::BinOp(op)) if instrs.len() >= 2 => {
                let x = instrs.pop().unwrap();
                let y = instrs.pop().unwrap();

                if let (mir::Instr::Push(x), mir::Instr::Push(y)) = (&x.data, &y.data) {
                    instrs.push(instr.swap(mir::Instr::Push(mir::Value::ImmediateBinOp(op, Box::new(x.clone()), Box::new(y.clone())))));
                    true
                } else {
                    instrs.push(y);
                    instrs.push(x);
                    false
                }
            }
            mir::Instr::ImmediateCall(mir::Function::TriOp(op)) if instrs.len() >= 3 => {
                let x = instrs.pop().unwrap();
                let y = instrs.pop().unwrap();
                let z = instrs.pop().unwrap();

                if let (mir::Instr::Push(x), mir::Instr::Push(y), mir::Instr::Push(z)) = (&x.data, &y.data, &z.data) {
                    instrs.push(instr.swap(mir::Instr::Push(mir::Value::ImmediateTriOp(
                        op,
                        Box::new(x.clone()),
                        Box::new(y.clone()),
                        Box::new(z.clone()),
                    ))));
                    true
                } else {
                    instrs.push(z);
                    instrs.push(y);
                    instrs.push(x);
                    false
                }
            }

            _ => false,
        };

        changes_made |= handled;

        if !handled {
            instrs.push(instr.clone());
        }
    }

    (changes_made, instrs)
}

impl OptimizationPass for ImmediateOperations {
    fn name(&self) -> &'static str {
        "Immediate Operation Pass"
    }

    fn run(&self, context: &OptimizationContext) -> Vec<Span<mir::Instr>> {
        let (mut changes_made, mut instrs) = run_pass(context.current_instrs(), context.current_len());

        while changes_made {
            (changes_made, instrs) = run_pass(instrs.iter(), instrs.len());
        }

        instrs
    }
}
