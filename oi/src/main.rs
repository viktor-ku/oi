use notify_rust::{Notification, Urgency};
use std::thread;
use std::time;

use lib_duration::duration;

fn main() {
    let input: String = std::env::args()
        .skip(1)
        .into_iter()
        .collect::<Vec<String>>()
        .join(" ");
    let input = input.trim();

    let duration = duration(input).expect("failed to parse input");
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
        .timeout(5_000)
        .urgency(Urgency::Critical)
        .show()
        .unwrap();
}
