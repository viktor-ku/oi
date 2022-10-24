use super::Cli;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use lib_config::Config;
use lib_player::Player;
use lib_store::{Store, TimerInput};
use notify_rust::{Notification, Urgency};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::{spawn, spawn_blocking};
use tokio::time::{self, Duration};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::apidoc::ApiDoc;

#[derive(Debug)]
pub struct ChannelMessage {
    remaining: u64,
    timer: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "List active timers", body = [Timer])
    )
)]
#[get("/timers/active")]
pub async fn find_active_timers(store: web::Data<Mutex<Store>>) -> impl Responder {
    let timers = store.lock().unwrap().timers.find_active(None).unwrap();
    HttpResponse::Ok().json(timers)
}

#[utoipa::path(
    responses(
        (status = 200, description = "Find timer by id (UUID)", body = Timer)
    )
)]
#[get("/timers/{timer_id}")]
pub async fn find_by_uuid(
    store: web::Data<Mutex<Store>>,
    timer_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let timer = store
        .lock()
        .unwrap()
        .timers
        .find_by_uuid(&timer_id)
        .unwrap();
    HttpResponse::Ok().json(timer)
}

#[utoipa::path(
    responses(
        (status = 200, description = "Timer deleted successfully"),
    ),
    params(
        ("timer_id", description = "timer UUID")
    ),
)]
#[delete("/timers/{timer_id}")]
pub async fn delete_by_uuid(
    store: web::Data<Mutex<Store>>,
    timer_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    store
        .lock()
        .unwrap()
        .timers
        .delete_by_uuid(&timer_id)
        .unwrap();
    HttpResponse::Ok()
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all timers", body = [Timer])
    )
)]
#[get("/timers")]
pub async fn find_all_timers(store: web::Data<Mutex<Store>>) -> impl Responder {
    let timers = store.lock().unwrap().timers.find_all().unwrap();
    HttpResponse::Ok().json(timers)
}

#[get("/")]
async fn home() -> impl Responder {
    "hi"
}

#[utoipa::path(
    request_body = TimerInput,
    responses(
        (status = 201, description = "Timer created successfully", body = Timer),
    )
)]
#[post("/timer")]
pub async fn create_timer(
    tx: web::Data<Mutex<mpsc::Sender<ChannelMessage>>>,
    store: web::Data<Mutex<Store>>,
    payload: web::Json<TimerInput>,
) -> impl Responder {
    let timer = store
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

    HttpResponse::Ok().json(timer)
}

async fn run_timer(remaining: u64, timer: String) {
    time::sleep(Duration::from_millis(remaining)).await;

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
    let store = Store::new(Config::config_dir()).await.unwrap();

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

    let openapi = ApiDoc::openapi();
    let store = Arc::new(store);

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Mutex::new(tx.clone()))
            .app_data(Mutex::new(Arc::clone(&store)))
            .service(create_timer)
            .service(home)
            .service(find_all_timers)
            .service(delete_by_uuid)
            .service(find_active_timers)
            .service(find_by_uuid)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    });

    if let Some(workers) = cli.workers {
        server = server.workers(workers);
    }

    println!("> up and running at: http://{}", bind);
    println!("> also swagger ui is up at: http://{}/swagger-ui/", bind);

    server.bind(&bind)?.run().await
}
