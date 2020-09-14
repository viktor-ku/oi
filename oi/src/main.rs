use chrono::Local;
use notify_rust::{Notification, Urgency};
use std::path::PathBuf;
use std::thread;
use std::time;

use lib_duration::duration;

mod player;
use player::Player;

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

    let player = Player::new(0.8);

    player.play(
        &PathBuf::from("/home/viktor/Music/notification.wav")
            .canonicalize()
            .unwrap(),
    );
}
