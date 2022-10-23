use super::Cli;
use actix_web::{delete, get, post, web, App, HttpServer};
use lib_api as api;
use lib_config::Config;
use lib_player::Player;
use lib_store::{self as store, Store};
use notify_rust::{Notification, Urgency};
use std::sync::{Mutex, Arc};
use store::TimerInput;
use tokio::sync::mpsc;
use tokio::task::{spawn, spawn_blocking};
use tokio::time::{self, Duration};

#[derive(Debug)]
struct ChannelMessage {
    remaining: u64,
    timer: String,
}

#[get("/timers/active")]
async fn find_active_timers(
    store: web::Data<Mutex<Store>>,
) -> api::timer::FindActiveTimersResponse {
    let timers = store.lock().unwrap().timers.find_active(None).unwrap();

    api::timer::FindActiveTimersResponse { timers }
}

#[get("/timers/{uuid}")]
async fn find_by_uuid(
    store: web::Data<Mutex<Store>>,
    web::Path(uuid): web::Path<uuid::Uuid>,
) -> api::timer::FindByUuidResponse {
    api::timer::FindByUuidResponse {
        timer: store.lock().unwrap().timers.find_by_uuid(&uuid).unwrap(),
    }
}

#[delete("/timers/{uuid}")]
async fn delete_by_uuid(
    store: web::Data<Mutex<Store>>,
    web::Path(uuid): web::Path<uuid::Uuid>,
) -> api::timer::DeleteByUuidResponse {
    api::timer::DeleteByUuidResponse {
        uuid: match store.lock().unwrap().timers.delete_by_uuid(&uuid).unwrap() {
            true => Some(uuid.clone()),
            false => None,
        },
    }
}

#[get("/timers")]
async fn find_all_timers(store: web::Data<Mutex<Store>>) -> api::timer::FindAllTimersResponse {
    let timers = store.lock().unwrap().timers.find_all().unwrap();
    api::timer::FindAllTimersResponse { timers }
}

#[post("/timer")]
async fn create_timer(
    tx: web::Data<Mutex<mpsc::Sender<ChannelMessage>>>,
    store: web::Data<Mutex<Store>>,
    payload: web::Json<TimerInput>,
) -> api::timer::CreateTimerResponse {
    let uuid = store
        .lock()
        .unwrap()
        .timers
        .create(payload.0.clone())
        .await
        .unwrap();

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
    let store = Arc::new(Store::new(Config::config_dir()).await.unwrap());

    spawn(async move {
        while let Some(msg) = rx.recv().await {
            spawn(async move {
                run_timer(msg.remaining as u64, msg.timer).await;
            });
        }
    });

    let active_timers = store.timers.find_active(None).unwrap();
    for timer in active_timers {
        tx.clone()
            .send(ChannelMessage {
                remaining: timer.remaining(None),
                timer: timer.message.clone(),
            })
            .await
            .unwrap();
    }

    let bind = format!("localhost:{}", cli.port.unwrap_or(Config::new().port));

    let mut server = HttpServer::new(move || {
        App::new()
            .data(Mutex::new(tx.clone()))
            .data(Mutex::new(store.clone()))
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
