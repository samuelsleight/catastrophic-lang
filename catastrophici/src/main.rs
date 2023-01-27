use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic_analyser::analyser::Analyser;
use catastrophic_core::error::context::{ErrorContext, PackagedError};
use catastrophic_interpreter::interpreter::Interpreter;
use catastrophic_parser::parser::Parser;
use clap::Parser as ArgParser;

#[derive(Debug, Clone, ArgParser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let error_context = ErrorContext::from_file(&args.input)?;

    let ast = Parser::from_file(args.input)
        .and_then(Parser::parse)
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
