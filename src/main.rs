extern crate catastrophic_analyser as analyser;
extern crate catastrophic_parser as parser;

use std::path::PathBuf;

use structopt::StructOpt;

use analyser::analyse;
use parser::parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "catastrophic")]
struct Args {
    file: PathBuf,
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let ast = parse(args.file)?;
    println!("{:#?}", ast);
    let instrs = analyse(ast)?;
    println!("{:#?}", instrs);
    Ok(())
}
