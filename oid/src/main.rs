use structopt::StructOpt;

mod cli;
use cli::Cli;

mod config;
use config::Config;

mod norm_path;
use norm_path::norm_path;

mod app;
mod detach;
mod on_timeout;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();

    if cli.detach {
        detach::detach(&cli)
    } else {
        app::app(cli).await
    }
}
