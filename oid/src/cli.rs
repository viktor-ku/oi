use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(long, default_value = "default")]
    pub sandbox: String,

    #[structopt(long, short)]
    pub detach: bool,

    #[structopt(long)]
    pub pid: Option<PathBuf>,

    #[structopt(long)]
    pub port: Option<u32>,

    #[structopt(long, default_value = "1")]
    pub workers: usize,

    #[structopt(long)]
    pub generate_openapi: bool,

    /// Fire an expected event this amount
    /// of milliseconds earlier than specified duration.
    #[structopt(long, default_value = "0")]
    pub latency: usize,
}
