use actix_web::{post, web, App, HttpServer};
use notify_rust::{Notification, Urgency};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::{spawn, spawn_blocking};
use tokio::time::{self, Duration};

mod player;
use player::Player;

mod config;
use config::Config;

mod norm_path;
pub use norm_path::norm_path;

mod on_timeout;

#[derive(Debug)]
struct ChannelMessage {
    duration: u64,
    timer: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Timer {
    start: i64,
    duration: u64,
    message: String,
}

#[post("/timer")]
async fn create_timer(
    tx: web::Data<Mutex<mpsc::Sender<ChannelMessage>>>,
    payload: web::Json<Timer>,
) -> String {
    tx.lock()
        .unwrap()
        .send(ChannelMessage {
            duration: payload.duration,
            timer: payload.message.clone(),
        })
        .await
        .unwrap();
    String::from("yo")
}

async fn run_timer(duration: u64, timer: String) {
    println!("gotta sleep for {} secs now, bye...", duration);
    time::delay_for(Duration::from_secs(duration)).await;
    println!("opacha! time to wake up!");

    spawn_blocking(move || {
        let config = Config::new();

        Notification::new()
            .summary("oi")
            .body(&timer)
            .timeout(10_000)
            .urgency(Urgency::Critical)
            .show()
            .unwrap();

        if let Some(sound_path) = &config.on_timeout.play {
            if sound_path.is_file() {
                let player = Player::new(config.volume());
                player.play(&sound_path);
            }
        }
    })
    .await
    .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(32);

    spawn(async move {
        println!("Listening for new sleeping requests :)");

        while let Some(msg) = rx.recv().await {
            spawn(async move {
                run_timer(msg.duration, msg.timer).await;
            });
        }
    });

    println!("Server is up and running, ready for accepting stuff!");

    HttpServer::new(move || {
        App::new()
            .data(Mutex::new(tx.clone()))
            .service(create_timer)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
