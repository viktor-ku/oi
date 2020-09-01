use notify_rust::{error::Error, Notification, NotificationHandle};
use std::thread;
use std::time;

use lib_duration::duration;

#[inline]
fn notify(title: &str, message: &str) -> Result<NotificationHandle, Error> {
    Ok(Notification::new()
        .summary(title)
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

    let duration = duration(input).expect("failed to parse input");
    notify("timer is up", &format!("{}", duration)).unwrap();

    thread::sleep(time::Duration::from_secs(duration.secs()));
    notify("", input).unwrap();
}
