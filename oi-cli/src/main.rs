use structopt::StructOpt;

mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::from_args();
    cli.exec();
}
