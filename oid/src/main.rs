use structopt::StructOpt;

mod cli;
use cli::Cli;

mod app;
mod detach;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();

    if cli.detach {
        detach::detach(&cli)
    } else {
        app::app(cli).await
    }
}
