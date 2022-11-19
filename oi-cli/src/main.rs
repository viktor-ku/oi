use clap::Parser;

mod cli;
use cli::Cli;

mod commands;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.exec().await.unwrap();
}
