use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic_analyser::analyser::Analyser;
use catastrophic_interpreter::interpreter::Interpreter;
use catastrophic_parser::parser::Parser;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    input: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let ast = Parser::parse_file(args.input).with_context(|| "Unable to parse input")?;
    let ir = Analyser::analyse_ast(ast).with_context(|| "Unable to compile input")?;
    Interpreter::interpret(ir).with_context(|| "Unable to run input")?;

    Ok(())
}
