use catastrophic_hir::hir;
use catastrophic_mir::mir;

mod convert;
mod pass;

pub struct Optimizer;

impl Optimizer {
    pub fn optimize_hir(hir: Vec<hir::Block>) -> Vec<mir::Block> {
        let mut mir = convert::convert_blocks(hir);

        for pass in pass::passes() {
            for block in &mut mir {
                pass.run(block)
            }
        }

        mir
    }
}
