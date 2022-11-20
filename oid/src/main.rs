use apidoc::ApiDoc;
use clap::Parser;

mod cli;
use cli::Cli;
use utoipa::OpenApi;

mod detach;
mod message;

pub mod apidoc;
pub mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if cli.generate_openapi {
        let spec = ApiDoc::openapi().to_pretty_json().unwrap();
        println!("{}", spec);
        return Ok(());
    }

    if cli.detach {
        detach::detach(&cli)
    } else {
        app::app(cli).await
    }
}
