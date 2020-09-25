use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(long, default_value = "default")]
    pub sandbox: String,

    #[structopt(long)]
    pub detach: bool,
}
