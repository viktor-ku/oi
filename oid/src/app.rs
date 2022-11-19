use super::Cli;
use crate::apidoc::ApiDoc;
use crate::message::OidMessage;
use actix_web::{
    delete, get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use anyhow::Ok;
use lib_config::Config;
use lib_player::Player;
use lib_store::{State, Timer, TimerInput, TimersStore};
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::join;
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
        (status = 200, description = "Timers deleted successfully"),
    ),
)]
#[delete("/timers/active")]
pub async fn delete_active(
    state: web::Data<Mutex<State>>,
    tx: web::Data<Mutex<mpsc::Sender<OidMessage>>>,
) -> impl Responder {
    let saved = {
        let mut state = state.try_lock().unwrap();
        let mut store = TimersStore::new(&mut state);
        store.delete_active().unwrap();
        state.0.save_incremental()
    };

    {
        let tx = tx.try_lock().unwrap();
        tx.send(OidMessage::Save(saved)).await.unwrap();
    };

    HttpResponse::Ok()
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
    tx: web::Data<Mutex<mpsc::Sender<OidMessage>>>,
) -> impl Responder {
    let saved = {
        let mut state = state.try_lock().unwrap();
        let mut store = TimersStore::new(&mut state);
        store.delete_by_uuid(&timer_id).unwrap();
        state.0.save_incremental()
    };

    {
        let tx = tx.try_lock().unwrap();
        tx.send(OidMessage::Save(saved)).await.unwrap();
    };

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
    let (timer, saved): (Timer, Vec<u8>) = {
        let mut state = state.try_lock().unwrap();
        let mut store = TimersStore::new(&mut state);
        let timer = store.create(payload.0.clone()).unwrap();
        let saved = state.0.save_incremental();
        (timer, saved)
    };

    {
        let tx = tx.try_lock().unwrap();
        let res = join!(
            tx.send(OidMessage::Save(saved)),
            tx.send(OidMessage::StartTimer(timer.clone())),
        );
        res.0.unwrap();
        res.1.unwrap();
    };

    HttpResponse::Ok().json(timer)
}

pub struct OiTask {
    pub timer: Timer,
    pub latency: u64,
    pub play_sound: Option<PathBuf>,
}

impl OiTask {
    pub async fn run_timer(self) {
        let duration = self
            .timer
            .remaining(None)
            .checked_sub(self.latency)
            .unwrap_or(0);

        time::sleep(Duration::from_millis(duration)).await;

        Notification::new()
            .summary("oi")
            .body(&self.timer.message)
            .timeout(10_000)
            .urgency(Urgency::Critical)
            .show()
            .unwrap();

        if let Some(play_sound) = self.play_sound {
            spawn_blocking(move || {
                let player = Player::new(1.0);
                player.play(play_sound.to_path_buf());
            })
            .await
            .unwrap();
        }
    }
}

async fn save(root: Option<PathBuf>, changes: &[u8]) -> anyhow::Result<()> {
    if let Some(ref root) = root {
        let mut last = State::persist(root.to_path_buf()).await?;
        last.0.load_incremental(changes).unwrap();
        last.save(root.to_path_buf()).await.unwrap();
    }

    Ok(())
}

pub async fn app(cli: Cli) -> std::io::Result<()> {
    println!("starting up...");
    let latency = cli.latency as u64;
    let config = Config::new().unwrap();
    let base_dir = Config::config_dir();
    let (tx, mut rx) = mpsc::channel::<OidMessage>(32);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    println!(
        "using this config:\n{}",
        serde_yaml::to_string(&config).unwrap()
    );

    spawn(async move {
        while let Some(msg) = rx.recv().await {
            spawn(async move {
                match msg {
                    OidMessage::StartTimer(timer) => {
                        OiTask {
                            timer,
                            latency,
                            play_sound: Config::new().unwrap().on_timeout.play,
                        }
                        .run_timer()
                        .await;
                    }
                    OidMessage::Save(changes) => {
                        save(Config::config_dir(), &changes).await.unwrap()
                    }
                };
            });
        }
    });

    let mut state = State::new(base_dir.clone())
        .await
        .expect("could not init state");

    {
        let timers_store = TimersStore::new(&mut state);
        let active_timers = timers_store.find_active(None).unwrap();
        println!("found {} sleeping timers", active_timers.len());
        for timer in active_timers {
            tx.send(OidMessage::StartTimer(timer)).await.unwrap();
        }
    };

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
            .service(delete_active)
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

    let bind = format!("localhost:{}", cli.port.unwrap_or(config.port));
    server = server.bind(&bind)?;
    println!("> up and running at: http://{}", bind);
    println!("> also swagger ui is up at: http://{}/swagger-ui/", bind);

    server.run().await
}
