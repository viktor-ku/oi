use chrono::Local;
use notify_rust::{Notification, Urgency};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;
use std::time;

use lib_duration::duration;

fn play(path: &PathBuf) {
    let device = rodio::default_output_device().unwrap();

    let sink = rodio::Sink::new(&device);

    let file = File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.set_volume(0.8);
    sink.append(source);

    sink.sleep_until_end();
}

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

    play(
        &PathBuf::from("/home/viktor/Music/notification.wav")
            .canonicalize()
            .unwrap(),
    );
}
