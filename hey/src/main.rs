use notify_rust::{error::Error, Notification, NotificationHandle};
use std::thread;
use std::time;

use lib_duration::duration;

fn notify(message: &str) -> Result<NotificationHandle, Error> {
    Ok(Notification::new()
        .summary("timer")
        .body(message)
        .timeout(3_000)
        .show()?)
}

fn main() {
    let input: String = std::env::args()
        .skip(1)
        .into_iter()
        .collect::<Vec<String>>()
        .join(" ");
    let input = input.trim();

    let parsed = duration(input).expect("failed to parse input");
    println!("{:#?}", parsed);
}
