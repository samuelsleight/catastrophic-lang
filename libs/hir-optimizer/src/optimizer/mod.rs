use catastrophic_core::profiling::TimeScope;
use catastrophic_hir::hir;
use catastrophic_mir::mir;

use self::context::OptimizationContext;
use self::pass::OptimizationPass;

mod context;
mod convert;
mod pass;

pub struct Options {
    passes: Vec<Box<dyn OptimizationPass>>,
}

pub struct Optimizer;

impl Options {
    #[must_use]
    pub fn no_passes() -> Self {
        Self { passes: Vec::new() }
    }

    #[must_use]
    pub fn all_passes() -> Self {
        Self { passes: pass::passes() }
    }

    #[must_use]
    pub fn without_pass(name: &str) -> Self {
        Self {
            passes: pass::passes()
                .into_iter()
                .filter(|pass| pass.name() != name)
                .collect(),
        }
    }
}

impl Optimizer {
    #[must_use]
    pub fn pass_names() -> Vec<&'static str> {
        pass::pass_names()
    }

    pub fn optimize_hir<'a, 'b: 'a>(options: Options, higher_ir: Vec<hir::Block>, time_scope: &'a mut TimeScope<'b>) -> Vec<mir::Block> {
        let mut middle_ir = {
            let _scope = time_scope.scope(&"Conversion");
            convert::convert_blocks(higher_ir)
        };

        for pass in options.passes {
            let _scope = time_scope.scope(&format!("{} Pass", &pass.name()));

            for input in &mut middle_ir {
                let context = OptimizationContext::new(input);
                let instrs = pass.run(&context);
                input.instrs = instrs;
            }
        }

        middle_ir
    }
}
