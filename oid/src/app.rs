use super::Cli;
use crate::apidoc::ApiDoc;
use crate::message::OidMessage;
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
    tx: web::Data<Mutex<mpsc::Sender<OidMessage>>>,
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
        .send(OidMessage::StartTimer(timer.clone()))
        .await
        .unwrap();

    HttpResponse::Ok().json(timer)
}

async fn run_timer(timer: Timer, latency: u64) {
    let duration = timer.remaining(None).checked_sub(latency).unwrap_or(0);

    time::sleep(Duration::from_millis(duration)).await;

    spawn_blocking(move || {
        let config = Config::new();

        Notification::new()
            .summary("oi")
            .body(&timer.message)
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
    let (tx, mut rx) = mpsc::channel::<OidMessage>(32);
    let latency = cli.latency as u64;

    let mut state = State::new(Config::config_dir())
        .await
        .expect("could not init state");

    spawn(async move {
        while let Some(msg) = rx.recv().await {
            spawn(async move {
                match msg {
                    OidMessage::StartTimer(timer) => {
                        run_timer(timer, latency).await;
                    }
                    OidMessage::Save => {
                        todo!()
                    }
                };
            });
        }
    });

    let timers_store = TimersStore::new(&mut state);
    let active_timers = timers_store.find_active(None).unwrap();
    println!("found {} sleeping timers", active_timers.len());
    {
        for timer in active_timers {
            tx.send(OidMessage::StartTimer(timer)).await.unwrap();
        }
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
