use crate::commands;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create and launch a timer
    Run {
        /// Any valid input according to the runic
        ///
        /// Example: "I have meeting at 4pm utc-2 +10m"
        ///
        /// Docs: https://github.com/viktor-ku/runic
        timer: String,

        /// Print json into the stdout describing the timer
        #[arg(long)]
        json: bool,

        /// Do not print anything in the stdout, overriding --json argument if provided
        #[arg(short, long)]
        silent: bool,

        /// By default the client will try to send the timer request
        /// to the server, or run local timer if server is not responding
        ///
        /// However, you can choose to not even try to send the request to the
        /// server, for example in case you know there is no server in the first place
        ///
        /// In local mode it runs the timer and that's it, no data about the
        /// timer is stored anywhere on your device
        #[arg(short, long)]
        local: bool,
    },

    /// Delete the timer by id
    Rm {
        /// Timer id (uuid)
        id: uuid::Uuid,
    },

    /// Delete all active timers
    Clean {},
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        match self.command {
            Commands::Run {
                timer,
                json,
                silent,
                local,
            } => {
                commands::run::RunCommand {
                    timer: &timer,
                    json,
                    silent,
                    local,
                }
                .exec()
                .await
            }
            Commands::Rm { id } => commands::rm::RmCommand { timer_id: id }.exec().await,
            Commands::Clean {} => commands::clean::CleanCommand {}.exec().await,
        }
    }
}
