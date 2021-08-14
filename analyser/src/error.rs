use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Named value `{0}` cannot refer to another named value (`{1}`)")]
    NamedNamedValue(String, String),

    #[error("Named value `{0}` cannot be redefined in the same scope")]
    DuplicateNamedValue(String),

    #[error("Unable to analyse input file")]
    AnalysisError,
}
