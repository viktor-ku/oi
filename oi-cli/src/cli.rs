use chrono::Local;
use lib_duration::duration;
use notify_rust::Notification;
use serde::Serialize;
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

#[derive(Debug, Serialize)]
struct Body {
    start: i64,
    duration: u64,
    message: String,
}

impl Cli {
    #[inline]
    pub fn exec(&self) {
        match self {
            Cli::Run(props) => self.run(props),
        }
    }

    fn run(&self, props: &RunProps) {
        let now = Local::now();
        let input = &props.timer;
        let duration = duration(input, &now).expect("failed to parse input");

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("http://localhost:8080/timer")
            .json(&Body {
                start: now.timestamp_millis(),
                duration: duration.secs(),
                message: input.to_owned(),
            })
            .send();

        match response {
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
