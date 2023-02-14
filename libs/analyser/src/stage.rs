use catastrophic_ast::ast;
use catastrophic_core::{profiling::TimeScope, stage::Stage};
use catastrophic_hir::hir;

use crate::analyser::{Analyser, CompileErrors};

pub struct AnalysisStage;

impl Stage<ast::Block> for AnalysisStage {
    type Output = Vec<hir::Block>;
    type Error = CompileErrors;

    fn run(self, input: ast::Block, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Analyser::analyse_ast(input)
    }

    fn name() -> &'static str {
        "AST Analysis"
    }

    fn error_context() -> &'static str {
        "Unable to compile input"
    }
}
