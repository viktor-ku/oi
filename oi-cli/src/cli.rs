use super::{Config, Player};
use chrono::Local;
use lib_duration::duration;
use notify_rust::{Notification, Urgency};
use std::thread;
use std::time;
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
    pub fn exec(&self) {
        match self {
            Cli::Run(props) => self.run(props),
        }
    }

    fn run(&self, props: &RunProps) {
        let now = Local::now();
        let timer = &props.timer;
        let duration = duration(timer, &now).expect("failed to parse input");
        Notification::new()
            .summary("timer is now running")
            .body(&format!("{}", duration))
            .timeout(2_500)
            .show()
            .unwrap();

        thread::sleep(time::Duration::from_secs(duration.secs()));

        Notification::new()
            .summary("oi")
            .body(timer)
            .timeout(10_000)
            .urgency(Urgency::Critical)
            .show()
            .unwrap();

        let config = Config::new();

        if let Some(sound_path) = &config.on_timeout.play {
            if sound_path.is_file() {
                let player = Player::new(config.volume());
                player.play(&sound_path);
            }
        }
    }
}
