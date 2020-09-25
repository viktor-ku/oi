use structopt::StructOpt;

mod cli;
use cli::Cli;

mod config;
pub use config::Config;

mod norm_path;
pub use norm_path::norm_path;

mod on_timeout;

mod app;
use app::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();

    if cli.detach {
        println!("Spawning child...");
        let args: Vec<String> = std::env::args().filter(|arg| arg != "--detach").collect();
        let mut cmd = std::process::Command::new(args.first().unwrap());
        cmd.args(&args[1..]);
        if let Ok(child) = cmd.spawn() {
            println!("Spawn OK ({})", child.id());
        } else {
            eprintln!("Could not spawn a child, exiting...");
            std::process::exit(1);
        }
    } else {
        app(cli).await?;
    }

    Ok(())
}
