use std::{fmt::Debug, path::PathBuf};

use anyhow::Result;
use catastrophic_analyser::stage::AnalysisStage;
use catastrophic_compiler::stage::CompilationStage;
use catastrophic_core::{
    error::context::ErrorContext,
    profiling::TimeKeeper,
    stage::{pipeline, Continue, Pipeline, PipelineError, RunPipeline, Stage, StageContext},
};
use catastrophic_hir_optimizer::stage::OptimizationStage;
use catastrophic_parser::stage::ParseStage;
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

fn debug_callback<Input: Debug>(debug: bool) -> impl FnOnce(&StageContext<Input>) -> Continue {
    move |input| {
        if debug {
            println!("{:#?}", input.input);
            Continue::Cancel
        } else {
            Continue::Continue
        }
    }
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let error_context = ErrorContext::from_file(&args.input)?;
    let time_keeper = TimeKeeper::new("Overall");
    let pipeline_context = StageContext::new(args.input, time_keeper, error_context);

    let result = pipeline(ParseStage.stage(), debug_callback(args.debug == Some(DebugMode::Ast)))
        .and_then(AnalysisStage.stage(), debug_callback(args.debug == Some(DebugMode::Hir)))
        .and_then(OptimizationStage.stage(), debug_callback(args.debug == Some(DebugMode::Mir)))
        .and_then(CompilationStage.stage(), |_| ())
        .run(pipeline_context);

    match result {
        Ok(context) => {
            if args.profile {
                context.time_keeper.finish()
            }

            Ok(())
        }
        Err(PipelineError::Cancelled) => Ok(()),
        Err(PipelineError::Err(error)) => Err(error),
    }
}
