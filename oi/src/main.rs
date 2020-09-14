use chrono::Local;
use lib_duration::duration;
use notify_rust::{Notification, Urgency};
use std::thread;
use std::time;

mod config;
use config::Config;

mod player;
use player::Player;

pub mod on_timeout;

mod norm_path;
pub use norm_path::norm_path;

fn main() {
    let input: String = std::env::args()
        .skip(1)
        .into_iter()
        .collect::<Vec<String>>()
        .join(" ");
    let input = input.trim();

    let now = Local::now();
    let duration = duration(input, &now).expect("failed to parse input");
    Notification::new()
        .summary("timer is now running")
        .body(&format!("{}", duration))
        .timeout(2_500)
        .show()
        .unwrap();

    thread::sleep(time::Duration::from_secs(duration.secs()));

    Notification::new()
        .summary("oi")
        .body(input)
        .timeout(10_000)
        .urgency(Urgency::Critical)
        .show()
        .unwrap();

    let config = Config::new();

    if let Some(sound_path) = &config.on_timeout.play {
        if sound_path.is_file() {
            let player = Player::new(config.volume());
            player.play(&sound_path);
        }
    }
}
