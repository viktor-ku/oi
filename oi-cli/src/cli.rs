use chrono::Local;
use lib_api::{self as api, Client};
use lib_config::Config;
use notify_rust::Notification;
use runic::Runic;
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
