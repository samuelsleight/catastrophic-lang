use catastrophic_hir::hir;
use catastrophic_mir::mir;

pub struct Optimizer;

impl Optimizer {
    pub fn optimize_hir(hir: Vec<hir::Block>) -> Vec<mir::Block> {
        hir
    }
}
