use std::path::PathBuf;

use anyhow::Result;
use catastrophic_analyser::stage::AnalysisStage;
use catastrophic_core::{
    error::context::ErrorContext,
    profiling::TimeKeeper,
    stage::{pipeline, Pipeline, PipelineError, RunPipeline, Stage, StageContext},
};
use catastrophic_interpreter::stage::InterpreterStage;
use catastrophic_parser::stage::ParseStage;
use clap::Parser as ArgParser;

#[derive(Debug, Clone, ArgParser)]
struct Args {
    input: PathBuf,

    #[arg(short, long)]
    profile: bool,
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let error_context = ErrorContext::from_file(&args.input)?;
    let time_keeper = TimeKeeper::new("Overall");
    let pipeline_context = StageContext::new(args.input, time_keeper, error_context);

    let result = pipeline(ParseStage.stage(), |_| ())
        .and_then(AnalysisStage.stage(), |_| ())
        .and_then(InterpreterStage.stage(), |_| ())
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
