use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to read input file")]
    FileReadError(#[from] io::Error),

    #[error("Unable to parse input file")]
    ParseError(#[from] lalrpop_util::ParseError<usize, String, String>),
}
