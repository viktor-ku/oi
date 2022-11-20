use crate::commands;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the timer with given input
    Run {
        timer: String,

        #[arg(long)]
        json: bool,

        /// Do not print anything in the stdout
        #[arg(short, long)]
        silent: bool,
    },

    /// Delete a timer by its id (uuid)
    Rm {
        #[arg(value_name = "uuid")]
        timer_uuid: uuid::Uuid,
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
            } => {
                commands::run::RunCommand {
                    timer: &timer,
                    json,
                    silent,
                }
                .exec()
                .await
            }
            Commands::Rm { timer_uuid } => {
                commands::rm::RmCommand {
                    timer_id: timer_uuid,
                }
                .exec()
                .await
            }
            Commands::Clean {} => commands::clean::CleanCommand {}.exec().await,
        }
    }
}
