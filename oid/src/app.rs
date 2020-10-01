use super::Cli;
use actix_web::{delete, get, post, web, App, HttpServer};
use lib_api as api;
use lib_config::Config;
use lib_player::Player;
use lib_store::{self as store, Store};
use notify_rust::{Notification, Urgency};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::{spawn, spawn_blocking};
use tokio::time::{self, Duration};

#[derive(Debug)]
struct ChannelMessage {
    remaining: u64,
    timer: String,
}

#[get("/timers/active")]
async fn find_active_timers(store: web::Data<Store>) -> api::timer::FindActiveTimersResponse {
    let timers = store.timers.find_active().await;
    api::timer::FindActiveTimersResponse {
        timers: timers
            .iter()
            .map(|timer| api::timer::Timer {
                uuid: timer.uuid,
                start: timer.start,
                message: timer.message.clone(),
                duration: timer.duration,
                remaining: timer.remaining(),
            })
            .collect(),
    }
}

#[get("/timers/{uuid}")]
async fn find_by_uuid(
    store: web::Data<Store>,
    web::Path(uuid): web::Path<uuid::Uuid>,
) -> api::timer::FindByUuidResponse {
    api::timer::FindByUuidResponse {
        timer: store
            .timers
            .find_by_uuid(&uuid)
            .await
            .map(|timer| api::timer::Timer {
                uuid: timer.uuid,
                start: timer.start,
                message: timer.message.clone(),
                duration: timer.duration,
                remaining: timer.remaining(),
            }),
    }
}

#[delete("/timers/{uuid}")]
async fn delete_by_uuid(
    store: web::Data<Store>,
    web::Path(uuid): web::Path<uuid::Uuid>,
) -> api::timer::DeleteByUuidResponse {
    api::timer::DeleteByUuidResponse {
        uuid: store.timers.delete_by_uuid(&uuid).await,
    }
}

#[get("/timers")]
async fn find_all_timers(store: web::Data<Store>) -> api::timer::FindAllTimersResponse {
    let timers = store.timers.find_all().await;
    api::timer::FindAllTimersResponse {
        timers: timers
            .iter()
            .map(|timer| api::timer::Timer {
                uuid: timer.uuid,
                start: timer.start,
                message: timer.message.clone(),
                duration: timer.duration,
                remaining: timer.remaining(),
            })
            .collect(),
    }
}

#[post("/timer")]
async fn create_timer(
    tx: web::Data<Mutex<mpsc::Sender<ChannelMessage>>>,
    store: web::Data<Store>,
    payload: web::Json<api::timer::CreateTimerInput>,
) -> api::timer::CreateTimerResponse {
    let uuid = store
        .timers
        .create(store::TimerInput {
            start: payload.start,
            message: payload.message.clone(),
            duration: payload.duration,
        })
        .await;

    tx.lock()
        .unwrap()
        .send(ChannelMessage {
            remaining: payload.duration,
            timer: payload.message.clone(),
        })
        .await
        .unwrap();

    api::timer::CreateTimerResponse { uuid }
}

async fn run_timer(remaining: u64, timer: String) {
    time::delay_for(Duration::from_millis(remaining)).await;

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
    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(32);
    let store = Store::new(Config::config_dir().unwrap(), cli.sandbox).await;

    spawn(async move {
        while let Some(msg) = rx.recv().await {
            spawn(async move {
                run_timer(msg.remaining as u64, msg.timer).await;
            });
        }
    });

    let active_timers = store.timers.find_active().await;
    for timer in active_timers {
        tx.clone()
            .send(ChannelMessage {
                remaining: timer.remaining(),
                timer: timer.message.clone(),
            })
            .await
            .unwrap();
    }

    let bind = format!("localhost:{}", cli.port.unwrap_or(Config::new().port));

    let mut server = HttpServer::new(move || {
        App::new()
            .data(Mutex::new(tx.clone()))
            .data(store.clone())
            .service(create_timer)
            .service(find_all_timers)
            .service(find_active_timers)
            .service(find_by_uuid)
            .service(delete_by_uuid)
    });

    if let Some(workers) = cli.workers {
        server = server.workers(workers);
    }

    server.bind(&bind)?.run().await
}
