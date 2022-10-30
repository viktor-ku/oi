use chrono::Utc;
use lib_api::Client;
use lib_config::Config;
use lib_store::TimerInput;
use notify_rust::Notification;
use runic::Runic;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunProps {
    pub timer: String,

    #[structopt(long)]
    pub json: bool,
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
            Ok(_) => {
                Notification::new()
                    .summary("timer was deleted")
                    .body(&props.timer_uuid.to_string())
                    .timeout(2_500)
                    .show()
                    .unwrap();
            }
            Err(e) => {
                eprintln!("Could not connect to a server");
                eprintln!("{}", e);
            }
        }
    }

    async fn run(&self, props: &RunProps) {
        let now = Utc::now();
        let input = &props.timer;
        let mut runic = Runic::new(input);
        runic.timestamp(now.timestamp());
        let duration = runic.describe().unwrap();

        let body = {
            let (h, m, s) = runic::hms(duration);
            format!("{}h {}m {}s", h, m, s)
        };

        let config = Config::new();
        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client
            .timers
            .create(&TimerInput {
                start: now.timestamp_millis() as _,
                duration: (duration as u64) * 1_000,
                message: input.to_owned(),
            })
            .await
        {
            Ok(timer) => {
                Notification::new()
                    .summary("timer is now running")
                    .body(&body)
                    .timeout(2_500)
                    .show()
                    .unwrap();

                if props.json {
                    println!("{}", serde_json::to_string_pretty(&timer).unwrap());
                }
            }
            Err(e) => {
                eprintln!("{}", bind);
                eprintln!("{}", e);
            }
        }
    }
}
