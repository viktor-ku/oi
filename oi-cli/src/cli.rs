use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the timer with given input
    Run {
        #[arg(value_name = "timer")]
        timer: String,

        #[arg(long)]
        json: bool,
    },

    /// Delete the timer by uuid
    Rm {
        #[arg(value_name = "uuid")]
        timer_uuid: uuid::Uuid,
    },
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
            Commands::Run { timer, json } => {
                commands::run::RunCommand {
                    timer: &timer,
                    json,
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
        }
    }
}
