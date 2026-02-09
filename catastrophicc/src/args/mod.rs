use std::path::PathBuf;

use clap::Parser;

pub mod flags;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    // Debug options
    #[arg(long, help_heading = "Debug")]
    pub list: Option<flags::List>,

    // Compilation options
    #[arg(long)]
    pub debug: Option<flags::DebugMode>,

    #[arg(long)]
    pub pretty: bool,

    #[arg(long)]
    pub profile: bool,

    #[arg(long, default_value = "all")]
    pub opt: flags::Optimization,

    #[arg(long)]
    pub skip_pass: Option<String>,

    // Compilation input
    #[arg(required_unless_present = "list")]
    pub input: Option<PathBuf>,
}

impl Args {
    pub fn try_parse() -> Result<Self, clap::error::Error> {
        <Self as Parser>::try_parse()
    }
}
