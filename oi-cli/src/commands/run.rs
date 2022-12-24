use anyhow::Result;
use chrono::{Local, Utc};
use lib_api::Client;
use lib_config::Config;
use lib_player::Player;
use lib_store::TimerInput;
use notify_rust::{Notification, Urgency};
use runic::Runic;
use std::time::Duration;
use tokio::{task::spawn_blocking, time};

pub struct RunCommand<'c> {
    pub timer: &'c str,
    pub json: bool,
    pub silent: bool,
    pub local: bool,
}

impl<'c> RunCommand<'c> {
    async fn run_local_timer(self, config: Config, duration: u64) -> Result<()> {
        let body = RunCommand::format_body(duration);

        self.print_human_report(duration, &body);

        Notification::new()
            .summary("oi")
            .body(&body)
            .timeout(5_000)
            .urgency(Urgency::Normal)
            .show()
            .unwrap();

        time::sleep(Duration::from_secs(duration)).await;

        Notification::new()
            .summary("oi")
            .body(&body)
            .timeout(10_000)
            .urgency(Urgency::Critical)
            .show()
            .unwrap();

        if let Some(play_sound) = config.on_timeout.play {
            spawn_blocking(move || {
                let player = Player::new(1.0);
                player.play(play_sound.to_path_buf());
            })
            .await
            .unwrap();
        }

        Ok(())
    }

    #[inline]
    fn format_body(duration: u64) -> String {
        let (h, m, s) = runic::hms(duration);
        format!("{}h {}m {}s", h, m, s)
    }

    fn print_human_report(&self, duration: u64, body: &str) {
        if !self.silent && !self.json {
            let until = Local::now()
                .checked_add_signed(chrono::Duration::seconds(duration as _))
                .unwrap();
            println!("timer started for:");
            println!("       {}", body);
            println!("until: {}", until);
        }
    }

    pub async fn exec(self) -> Result<()> {
        let now = Utc::now();
        let duration = {
            let mut runic = Runic::new(self.timer);
            runic.timestamp(now.timestamp());
            runic.describe()?
        };
        let config = Config::new().unwrap();

        if self.local {
            return self.run_local_timer(config, duration).await;
        }

        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client
            .timers
            .create(&TimerInput {
                start: now.timestamp_millis() as _,
                duration: duration as u64 * 1_000,
                message: self.timer.to_owned(),
            })
            .await
        {
            Ok(timer) => {
                let body = RunCommand::format_body(duration);

                Notification::new()
                    .summary("timer is now running")
                    .body(&body)
                    .timeout(2_500)
                    .show()
                    .unwrap();

                if !self.silent {
                    if self.json {
                        println!("{}", serde_json::to_string_pretty(&timer).unwrap());
                    } else {
                        self.print_human_report(duration, &body);
                    }
                }

                Ok(())
            }
            Err(e) => {
                if e.is_connect() || e.is_status() {
                    if !self.silent {
                        eprintln!(
                            "could not send timer request to the server, launching local timer..."
                        );
                    }
                    self.run_local_timer(config, duration).await
                } else {
                    Err(e.into())
                }
            }
        }
    }
}
