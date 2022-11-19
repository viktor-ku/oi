use anyhow::Result;
use lib_api::Client;
use lib_config::Config;
use notify_rust::Notification;

pub struct CleanCommand;

impl CleanCommand {
    pub async fn exec(self) -> Result<()> {
        let config = Config::new().unwrap();
        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client.timers.delete_active().await {
            Ok(_) => {
                Notification::new()
                    .summary("got them")
                    .body("all active timers have been deleted")
                    .timeout(2_500)
                    .show()
                    .unwrap();

                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
