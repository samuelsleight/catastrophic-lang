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
    pub fn optimize_hir<'a, 'b: 'a>(opt: Optimization, hir: Vec<hir::Block>, time_scope: &'a mut TimeScope<'b>) -> Vec<mir::Block> {
        let mut mir = {
            let _scope = time_scope.scope("Conversion");
            convert::convert_blocks(hir)
        };

        if opt > Optimization::None {
            for pass in pass::passes() {
                let _scope = time_scope.scope(pass.name());

                for input in &mut mir {
                    let context = OptimizationContext::new(input);
                    let instrs = pass.run(&context);
                    input.instrs = instrs;
                }
            }
        }

        mir
    }
}
