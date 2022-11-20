use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(long, default_value = "default")]
    pub sandbox: String,

    #[arg(long, short)]
    pub detach: bool,

    #[arg(long)]
    pub pid: Option<PathBuf>,

    #[arg(long)]
    pub port: Option<u32>,

    #[arg(long, default_value = "1")]
    pub workers: usize,

    #[arg(long)]
    pub generate_openapi: bool,

    /// Fire an expected event this amount
    /// of milliseconds earlier than specified duration.
    #[arg(long, default_value = "0")]
    pub latency: usize,
}
