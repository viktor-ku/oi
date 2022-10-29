use super::Cli;
use crate::apidoc::ApiDoc;
use actix_web::{
    delete, get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use lib_config::Config;
use lib_player::Player;
use lib_store::{State, Timer, TimerInput, TimersStore};
use notify_rust::{Notification, Urgency};
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::task::{spawn, spawn_blocking};
use tokio::time::{self, Duration};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug)]
pub struct ChannelMessage {
    /// in milliseconds
    remaining: u64,
    timer: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "List active timers", body = [Timer])
    )
)]
#[get("/timers/active")]
pub async fn find_active_timers(state: web::Data<Mutex<State>>) -> impl Responder {
    let mut state = state.try_lock().unwrap();
    let store = TimersStore::new(&mut state);
    let timers = store.find_active(None).unwrap();
    HttpResponse::Ok().json(timers)
}

#[utoipa::path(
    responses(
        (status = 200, description = "Find timer by id (UUID)", body = Timer)
    )
)]
#[get("/timers/{timer_id}")]
pub async fn find_by_uuid(
    state: web::Data<Mutex<State>>,
    timer_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut state = state.try_lock().unwrap();
    let store = TimersStore::new(&mut state);
    let timer = store.find_by_uuid(&timer_id).unwrap();
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
    state: web::Data<Mutex<State>>,
    timer_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mut state = state.try_lock().unwrap();
    let mut store = TimersStore::new(&mut state);
    store.delete_by_uuid(&timer_id).unwrap();
    HttpResponse::Ok()
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all timers", body = [Timer])
    )
)]
#[get("/timers")]
pub async fn find_all_timers(state: web::Data<Mutex<State>>) -> impl Responder {
    let mut state = state.try_lock().unwrap();
    let store = TimersStore::new(&mut state);
    let timers = store.find_all().unwrap();
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
    state: web::Data<Mutex<State>>,
    payload: web::Json<TimerInput>,
) -> impl Responder {
    let timer: Timer = {
        let mut state = state.try_lock().unwrap();
        let mut store = TimersStore::new(&mut state);
        store.create(payload.0.clone()).unwrap()
    };

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

async fn run_timer(remaining: u64, timer: String, latency: u64) {
    let duration = remaining.checked_sub(latency).unwrap_or(0);

    time::sleep(Duration::from_millis(duration)).await;

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
    println!("starting up...");
    let (tx, mut rx) = mpsc::channel::<ChannelMessage>(32);
    let latency = cli.latency as u64;

    spawn(async move {
        println!("awaiting for incoming timers...");
        while let Some(msg) = rx.recv().await {
            spawn(async move {
                run_timer(msg.remaining, msg.timer, latency).await;
            });
        }
    });

    let mut state = State::new(Config::config_dir())
        .await
        .expect("could not init state");

    let timers_store = TimersStore::new(&mut state);
    let active_timers = timers_store.find_active(None).unwrap();
    println!("found {} sleeping timers", active_timers.len());
    for timer in active_timers {
        tx.clone()
            .send(ChannelMessage {
                remaining: timer.remaining(None),
                timer: timer.message.clone(),
            })
            .await
            .unwrap();
    }

    let openapi = ApiDoc::openapi();
    let tx = web::Data::new(Mutex::new(tx));
    let state = web::Data::new(Mutex::new(state));

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(tx.clone())
            .app_data(state.clone())
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

    {
        if cli.workers > 0 {
            server = server.workers(cli.workers);
        }

        let workers = cli.workers.to_string();
        println!(
            "spinning up workers ({})",
            if cli.workers == 0 { "auto" } else { &workers }
        );
    }

    let bind = format!("localhost:{}", cli.port.unwrap_or(Config::new().port));
    server = server.bind(&bind)?;
    println!("> up and running at: http://{}", bind);
    println!("> also swagger ui is up at: http://{}/swagger-ui/", bind);

    server.run().await
}
