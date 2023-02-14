use std::path::PathBuf;

use anyhow::{Context, Result};
use catastrophic_analyser::analyser::Analyser;
use catastrophic_compiler::compiler::Compiler;
use catastrophic_core::{
    error::context::{ErrorContext, PackagedError},
    profiling::TimeKeeper,
};
use catastrophic_hir_optimizer::optimizer::Optimizer;
use catastrophic_parser::parser::Parser;
use clap::{Parser as ArgParser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DebugMode {
    Ast,
    Hir,
    Mir,
}

#[derive(Debug, Clone, ArgParser)]
struct Args {
    #[arg(long)]
    debug: Option<DebugMode>,

    #[arg(short, long)]
    profile: bool,

    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let error_context = ErrorContext::from_file(&args.input)?;

    let mut time_keeper = TimeKeeper::new("Overall");

    let ast = {
        let _scope = time_keeper.scope("Parsing");

        let ast = Parser::from_file(args.input)
            .and_then(Parser::parse)
            .map_err(|err| PackagedError::new(error_context.clone(), err))
            .with_context(|| "Unable to parse input")?;

        if args.debug == Some(DebugMode::Ast) {
            println!("{:#?}", ast);
            return Ok(());
        }

        ast
    };

    let hir = {
        let _scope = time_keeper.scope("AST Analysis");

        let hir = Analyser::analyse_ast(ast)
            .map_err(|err| PackagedError::new(error_context.clone(), err))
            .with_context(|| "Unable to compile input")?;

        if args.debug == Some(DebugMode::Hir) {
            println!("{:#?}", hir);
            return Ok(());
        }

        hir
    };

    let mir = {
        let _scope = time_keeper.scope("Optimisation");

        let mir = Optimizer::optimize_hir(hir);

        if args.debug == Some(DebugMode::Mir) {
            println!("{:#?}", mir);
            return Ok(());
        }

        mir
    };

    {
        let _scope = time_keeper.scope("Compilation");

        Compiler::compile(mir);
    }

    if args.profile {
        time_keeper.finish();
    }

    Ok(())
}
