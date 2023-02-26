use std::path::Path;

use catastrophic_ast::ast;
use catastrophic_core::{profiling::TimeScope, stage::Stage};

use crate::parser::{Error, Parser};

pub struct ParseStage;

impl<P: AsRef<Path>> Stage<P> for ParseStage {
    type Output = ast::Block;
    type Error = Error;

    fn run(self, input: P, _: &mut TimeScope) -> Result<Self::Output, Self::Error> {
        Parser::with_file(input)?.parse()
    }

    fn name() -> &'static str {
        "Parsing"
    }

    fn error_context() -> &'static str {
        "Unable to parse input"
    }
}
