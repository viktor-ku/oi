use chrono::Local;
use lib_api::{self as api, Client};
use lib_duration::duration;
use notify_rust::Notification;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunProps {
    pub timer: String,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Cli {
    #[structopt(about = "Starts the timer")]
    Run(RunProps),
}

impl Cli {
    #[inline]
    pub async fn exec(&self) {
        match self {
            Cli::Run(props) => {
                self.run(props).await;
            }
        };
    }

    async fn run(&self, props: &RunProps) {
        let now = Local::now();
        let input = &props.timer;
        let duration = duration(input, &now).expect("failed to parse input");

        let client = Client::new("http://localhost:8080");

        match client
            .timers
            .create(&api::CreateTimer {
                start: now.timestamp_millis(),
                duration: duration.total(),
                message: input.to_owned(),
            })
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Notification::new()
                        .summary("timer is now running")
                        .body(&duration.format())
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
