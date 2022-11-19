use anyhow::Result;
use lib_api::Client;
use lib_config::Config;
use notify_rust::Notification;
use uuid::Uuid;

pub struct RmCommand {
    pub timer_id: Uuid,
}

impl RmCommand {
    pub async fn exec(self) -> Result<()> {
        let config = Config::new()?;
        let bind = format!("http://localhost:{}", config.port);
        let client = Client::new(&bind);

        match client.timers.delete_by_uuid(&self.timer_id).await {
            Ok(_) => {
                Notification::new()
                    .summary("timer was deleted")
                    .body(&self.timer_id.to_string())
                    .timeout(2_500)
                    .show()
                    .unwrap();
                Ok(())
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}
