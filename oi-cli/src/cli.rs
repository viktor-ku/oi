use chrono::Local;
use lib_api::{self as api, Client};
use lib_config::Config;
use notify_rust::{Notification, Urgency};
use runic::Runic;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunProps {
    pub timer: String,
}

#[derive(Debug, StructOpt)]
pub struct RmProps {
    pub timer_uuid: uuid::Uuid,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Cli {
    #[structopt(about = "Start the timer")]
    Run(RunProps),

    #[structopt(about = "Delete the timer")]
    Rm(RmProps),
}

impl Cli {
    #[inline]
    pub async fn exec(&self) {
        match self {
            Cli::Run(props) => {
                self.run(props).await;
            }
            Cli::Rm(props) => {
                self.rm(props).await;
            }
        };
    }

    async fn rm(&self, props: &RmProps) {
        let config = Config::new();
        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client.timers.delete_by_uuid(&props.timer_uuid).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.data.unwrap().uuid {
                        Some(_) => {
                            Notification::new()
                                .summary("timer was deleted")
                                .body(&props.timer_uuid.to_hyphenated().to_string())
                                .timeout(2_500)
                                .show()
                                .unwrap();
                        }
                        None => {
                            Notification::new()
                                .summary("timer was not found")
                                .body(&props.timer_uuid.to_hyphenated().to_string())
                                .timeout(2_500)
                                .urgency(Urgency::Critical)
                                .show()
                                .unwrap();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not connect to a server");
                eprintln!("{}", e);
            }
        }
    }

    async fn run(&self, props: &RunProps) {
        let now = Local::now();
        let input = &props.timer;
        let rune = Runic::describe(input, now.timestamp() as u64);
        let duration = rune.total();

        let config = Config::new();
        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client
            .timers
            .create(&api::timer::CreateTimerInput {
                start: now.timestamp_millis(),
                duration: duration as u64,
                message: input.to_owned(),
            })
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Notification::new()
                        .summary("timer is now running")
                        .body(&Runic::translate(duration))
                        .timeout(2_500)
                        .show()
                        .unwrap();
                }
            }
            Err(e) => {
                eprintln!("Could not connect to a server");
                eprintln!("{}", e);
            }
        }
    }
}
