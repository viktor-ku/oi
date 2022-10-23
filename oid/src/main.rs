use apidoc::ApiDoc;
use structopt::StructOpt;

mod cli;
use cli::Cli;
use utoipa::OpenApi;

mod detach;

pub mod app;
pub mod apidoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();

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
