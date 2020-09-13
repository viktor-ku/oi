use chrono::Local;
use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn one_word() {
    let input = "yo";
    let actual = duration(input, &Local::now());
    assert_eq!(actual.unwrap(), Duration::default());
}

#[test]
fn words() {
    let input = "one two three no durations can you see";
    let actual = duration(input, &Local::now());
    assert_eq!(actual.unwrap(), Duration::default());
}
