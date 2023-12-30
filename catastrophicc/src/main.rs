use std::{fmt::Debug, path::PathBuf};

use anyhow::Result;
use args::{flags::DebugMode, Args};
use catastrophic_analyser::stage::AnalysisStage;
use catastrophic_compiler::stage::CompilationStage;
use catastrophic_core::{
    error::context::ErrorContext,
    pretty::{PrettyDebug, PrettyDebugger},
    profiling::TimeKeeper,
    stage::{pipeline, Continue, Pipeline, PipelineResult, RunPipeline, Stage, StageContext},
};
use catastrophic_hir_optimizer::stage::OptimizationStage;
use catastrophic_parser::stage::ParseStage;

mod args;

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
            .and_then(OptimizationStage::new(self.args.opt.into()).stage(), self.debug_callback(DebugMode::Mir))
            .and_then(CompilationStage.stage(), |_| ())
    }

    fn debug_callback<Input: Debug + PrettyDebug>(&self, debug: DebugMode) -> for<'a> fn(&'a StageContext<Input>) -> Continue {
        if self.args.debug != Some(debug) {
            |_| Continue::Continue
        } else if self.args.pretty {
            |input| {
                println!("{}", PrettyDebugger(&input.input));
                Continue::Cancel
            }
        } else {
            |input| {
                println!("{:#?}", input.input);
                Continue::Cancel
            }
        }
    }

    fn finish(&self, result: PipelineResult<StageContext<()>, anyhow::Error>) -> Result<()> {
        match result {
            PipelineResult::Ok(context) => {
                if self.args.profile {
                    context.time_keeper.finish()
                }

                Ok(())
            }
            PipelineResult::Cancelled => Ok(()),
            PipelineResult::Err(error) => Err(error),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;
    App::new(args).run()
}
