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

struct App {
    args: Args,
}

impl App {
    fn new(args: Args) -> Self {
        Self { args }
    }

    fn run(&self) -> Result<()> {
        let pipeline_context = self.make_context()?;
        let pipeline = self.make_pipeline();

        let result = pipeline.run(pipeline_context);
        self.finish(result)
    }

    fn make_context(&self) -> Result<StageContext<PathBuf>> {
        let error_context = ErrorContext::from_file(&self.args.input)?;
        let time_keeper = TimeKeeper::new("Overall");
        let pipeline_context = StageContext::new(self.args.input.clone(), time_keeper, error_context);
        Ok(pipeline_context)
    }

    fn make_pipeline(&self) -> impl RunPipeline<anyhow::Error, Start = StageContext<PathBuf>, End = StageContext<()>> {
        pipeline(ParseStage.stage(), self.debug_callback(DebugMode::Ast))
            .and_then(AnalysisStage.stage(), self.debug_callback(DebugMode::Hir))
            .and_then(OptimizationStage.stage(), self.debug_callback(DebugMode::Mir))
            .and_then(CompilationStage.stage(), |_| ())
    }

    fn debug_callback<Input: Debug>(&self, debug: DebugMode) -> impl FnOnce(&StageContext<Input>) -> Continue {
        let debug = self.args.debug == Some(debug);

        move |input| {
            if debug {
                println!("{:#?}", input.input);
                Continue::Cancel
            } else {
                Continue::Continue
            }
        }
    }

    fn finish(&self, result: Result<StageContext<()>, PipelineError<anyhow::Error>>) -> Result<()> {
        match result {
            Ok(context) => {
                if self.args.profile {
                    context.time_keeper.finish()
                }

                Ok(())
            }
            Err(PipelineError::Cancelled) => Ok(()),
            Err(PipelineError::Err(error)) => Err(error),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;
    App::new(args).run()
}
