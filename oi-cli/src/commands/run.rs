use anyhow::Result;
use chrono::{Duration, Local, Utc};
use lib_api::Client;
use lib_config::Config;
use lib_store::TimerInput;
use notify_rust::Notification;
use runic::Runic;

pub struct RunCommand<'c> {
    pub timer: &'c str,
    pub json: bool,
}

impl<'c> RunCommand<'c> {
    pub async fn exec(self) -> Result<()> {
        let now = Utc::now();
        let input = self.timer;
        let mut runic = Runic::new(input);
        runic.timestamp(now.timestamp());
        let duration = runic.describe()?;

        let body = {
            let (h, m, s) = runic::hms(duration);
            format!("{}h {}m {}s", h, m, s)
        };

        let config = Config::new().unwrap();
        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client
            .timers
            .create(&TimerInput {
                start: now.timestamp_millis() as _,
                duration: duration as u64 * 1_000,
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

                if self.json {
                    println!("{}", serde_json::to_string_pretty(&timer).unwrap());
                } else {
                    let until = Local::now()
                        .checked_add_signed(Duration::seconds(duration as _))
                        .unwrap();
                    println!("timer started for:");
                    println!("\t{}", body);
                    println!("\tuntil: {}", until);
                }

                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
