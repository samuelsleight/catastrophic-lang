use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic_analyser::analyser::Analyser;
use catastrophic_error::context::{ErrorContext, PackagedError};
use catastrophic_interpreter::interpreter::Interpreter;
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

    let ir = Analyser::analyse_ast(ast)
        .map_err(|err| PackagedError::new(error_context.clone(), err))
        .with_context(|| "Unable to compile input")?;

    Interpreter::interpret(ir)
        .map_err(|err| PackagedError::new(error_context, err))
        .with_context(|| "Unable to run input")?;

    Ok(())
}
