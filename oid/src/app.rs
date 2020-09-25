use super::Cli;
use actix_web::{get, post, web, App, HttpServer};
use lib_api as api;
use lib_config::Config;
use lib_player::Player;
use lib_store::Store;
use notify_rust::{Notification, Urgency};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::{spawn, spawn_blocking};
use tokio::time::{self, Duration};

#[derive(Debug)]
struct ChannelMessage {
    duration: i64,
    timer: String,
}

#[get("/timers/active")]
async fn find_active_timers(store: web::Data<Store>) -> api::timer::FindActiveTimersResponse {
    let records = store.records.find_active().await;
    api::timer::FindActiveTimersResponse {
        timers: records
            .iter()
            .map(|val| api::timer::Timer {
                start: val.start,
                duration: val.remaining as u64,
                message: val.message.clone(),
            })
            .collect(),
    }
}

#[get("/timers")]
async fn find_all_timers(store: web::Data<Store>) -> api::timer::FindAllTimersResponse {
    let records = store.records.find_all().await;
    api::timer::FindAllTimersResponse {
        timers: records
            .iter()
            .map(|val| api::timer::Timer {
                start: val.start,
                duration: val.remaining as u64,
                message: val.message.clone(),
            })
            .collect(),
    }
}

#[post("/timer")]
async fn create_timer(
    tx: web::Data<Mutex<mpsc::Sender<ChannelMessage>>>,
    store: web::Data<Store>,
    payload: web::Json<api::timer::CreateTimer>,
) -> api::timer::CreateTimerResponse {
    let id = store
        .records
        .create(lib_store::CreateRecord {
            start: payload.start,
            message: payload.message.clone(),
            duration: payload.duration,
        })
        .await;

    tx.lock()
        .unwrap()
        .send(ChannelMessage {
            duration: payload.duration as i64,
            timer: payload.message.clone(),
        })
        .await
        .unwrap();

    api::timer::CreateTimerResponse { uuid: id }
}

async fn run_timer(duration: u64, timer: String) {
    println!("gotta sleep for {} secs now, bye...", duration / 1_000);
    time::delay_for(Duration::from_millis(duration)).await;
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
                let player = Player::new(config.volume);
                player.play(&sound_path);
            }
        }
    })
    .await
    .unwrap();
}

pub async fn app(cli: Cli) -> std::io::Result<()> {
    println!("Sandbox in use: #{}", cli.sandbox);

    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(32);
    let store = Store::new(Config::config_dir().unwrap(), cli.sandbox).await;

    spawn(async move {
        println!("Listening for new sleeping requests :)");

        while let Some(msg) = rx.recv().await {
            spawn(async move {
                run_timer(msg.duration as u64, msg.timer).await;
            });
        }
    });

    let active_records = store.records.find_active().await;
    for record in active_records {
        tx.clone()
            .send(ChannelMessage {
                duration: record.remaining,
                timer: record.message.clone(),
            })
            .await
            .unwrap();
    }

    let bind = format!("localhost:{}", cli.port);
    println!("Server is up and running at: http://{}", bind);

    HttpServer::new(move || {
        App::new()
            .data(Mutex::new(tx.clone()))
            .data(store.clone())
            .service(create_timer)
            .service(find_all_timers)
            .service(find_active_timers)
    })
    .bind(&bind)?
    .run()
    .await
}
