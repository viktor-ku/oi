use structopt::StructOpt;

mod config;
pub use config::Config;

mod player;
pub use player::Player;

pub mod on_timeout;

mod norm_path;
pub use norm_path::norm_path;

mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::from_args();
    cli.exec();
}
