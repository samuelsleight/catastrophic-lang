use std::path::PathBuf;

use catastrophic_hir_optimizer::optimizer::Optimization;
use clap::Parser;

pub mod flags;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(long)]
    pub debug: Option<flags::DebugMode>,

    #[arg(long)]
    pub pretty: bool,

    #[arg(long)]
    pub profile: bool,

    #[arg(long = "opt", default_value = "all")]
    pub opt: flags::Optimization,

    pub input: PathBuf,
}

impl Args {
    pub fn try_parse() -> Result<Self, clap::error::Error> {
        <Self as Parser>::try_parse()
    }
}

impl From<flags::Optimization> for Optimization {
    fn from(value: flags::Optimization) -> Self {
        match value {
            flags::Optimization::None => Optimization::None,
            flags::Optimization::All => Optimization::All,
        }
    }
}
