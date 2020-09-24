use structopt::StructOpt;

mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();
    cli.exec().await;
}
