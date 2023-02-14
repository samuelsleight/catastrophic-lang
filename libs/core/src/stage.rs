use std::{
    fs::File,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    error::context::{ErrorContext, ErrorProvider, PackagedError},
    profiling::{TimeKeeper, TimeScope},
};

use anyhow::{Context, Result};

pub use pipeline::{pipeline, Continue, Err as PipelineError, Pipeline, RunPipeline, Stage as PipelineStage};

pub struct StageWrapper<Input, S: Stage<Input>>(S, PhantomData<Input>);

pub struct StageContext<Input> {
    pub input: Input,
    pub time_keeper: TimeKeeper,
    pub error_context: Arc<Mutex<ErrorContext<File>>>,
}

pub trait Stage<Input>: Sized {
    type Output;
    type Error: ErrorProvider + Send + Sync + 'static;

    fn stage(self) -> StageWrapper<Input, Self> {
        StageWrapper(self, PhantomData)
    }

    fn run(self, input: Input, timing: &mut TimeScope) -> Result<Self::Output, Self::Error>;

    fn name() -> &'static str;
    fn error_context() -> &'static str;
}

impl<Input, S: Stage<Input>> PipelineStage<anyhow::Error> for StageWrapper<Input, S> {
    type Input = StageContext<Input>;
    type Output = StageContext<<S as Stage<Input>>::Output>;

    fn run(self, context: Self::Input) -> anyhow::Result<Self::Output> {
        let StageContext {
            input,
            mut time_keeper,
            error_context,
        } = context;

        let timed_result = {
            let mut timing = time_keeper.scope(S::name());
            self.0.run(input, &mut timing)
        };

        timed_result
            .map_err(|err| PackagedError::new(error_context.clone(), err))
            .map(|output| StageContext {
                input: output,
                time_keeper,
                error_context,
            })
            .with_context(S::error_context)
    }
}

impl<Input> StageContext<Input> {
    pub fn new(input: Input, time_keeper: TimeKeeper, error_context: Arc<Mutex<ErrorContext<File>>>) -> Self {
        Self {
            input,
            time_keeper,
            error_context,
        }
    }
}
