use catastrophic_mir::mir::Block;

use self::immediate_calls::ImmediateCalls;

mod immediate_calls;

pub trait OptimizationPass {
    fn run(&self, block: &mut Block);
}

pub fn passes() -> Vec<Box<dyn OptimizationPass>> {
    vec![Box::new(ImmediateCalls)]
}
