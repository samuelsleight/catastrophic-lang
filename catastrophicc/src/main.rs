use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic_analyser::analyser::Analyser;
use catastrophic_compiler::compiler::Compiler;
use catastrophic_core::error::context::{ErrorContext, PackagedError};
use catastrophic_hir_optimizer::optimizer::Optimizer;
use catastrophic_parser::parser::Parser;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    input: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let error_context = ErrorContext::from_file(&args.input)?;

    let ast = Parser::parse_file(args.input)
        .map_err(|err| PackagedError::new(error_context.clone(), err))
        .with_context(|| "Unable to parse input")?;

    let hir = Analyser::analyse_ast(ast)
        .map_err(|err| PackagedError::new(error_context.clone(), err))
        .with_context(|| "Unable to compile input")?;

    let mir = Optimizer::optimize_hir(hir);

    Compiler::compile(mir);

    Ok(())
}
