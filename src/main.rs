use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic::lexer::Lexer;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    input: PathBuf
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    Lexer::lex(args.input, |token| println!("{:?}", token)).with_context(|| "Unable to parse input")
}
