use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(long, default_value = "default")]
    pub sandbox: String,

    #[structopt(long)]
    pub detach: bool,

    #[structopt(long)]
    pub pid: Option<PathBuf>,

    #[structopt(long, default_value = "8080")]
    pub port: u32,
}
