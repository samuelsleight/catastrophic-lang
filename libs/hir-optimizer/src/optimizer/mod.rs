use catastrophic_core::profiling::TimeScope;
use catastrophic_hir::hir;
use catastrophic_mir::mir;

use self::context::OptimizationContext;

mod context;
mod convert;
mod pass;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Optimization {
    None,
    All,
}

pub struct Optimizer;

impl Optimizer {
    pub fn optimize_hir<'a, 'b: 'a>(opt: Optimization, higher_ir: Vec<hir::Block>, time_scope: &'a mut TimeScope<'b>) -> Vec<mir::Block> {
        let mut middle_ir = {
            let _scope = time_scope.scope(&"Conversion");
            convert::convert_blocks(higher_ir)
        };

        if opt > Optimization::None {
            for pass in pass::passes() {
                let _scope = time_scope.scope(&pass.name());

                for input in &mut middle_ir {
                    let context = OptimizationContext::new(input);
                    let instrs = pass.run(&context);
                    input.instrs = instrs;
                }
            }
        }

        middle_ir
    }
}
