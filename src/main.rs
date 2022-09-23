use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic::parser::Parser;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    input: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let block = Parser::parse_file(args.input).with_context(|| "Unable to parse input")?;
    println!("RESULT: {:#?}", block);
    Ok(())
}
