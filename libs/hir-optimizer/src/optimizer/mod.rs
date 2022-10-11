use catastrophic_hir::hir;
use catastrophic_mir::mir;

use self::context::OptimizationContext;

mod context;
mod convert;
mod pass;

pub struct Optimizer;

impl Optimizer {
    pub fn optimize_hir(hir: Vec<hir::Block>) -> Vec<mir::Block> {
        let mut mir = convert::convert_blocks(hir);

        for pass in pass::passes() {
            for index in 0..mir.len() {
                let context = OptimizationContext::new(&mir[index]);
                let instrs = pass.run(&context);
                mir[index].instrs = instrs;
            }
        }

        mir
    }
}
