use chrono::Utc;
use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn a01() {
    let input = "delivery estimated in 1h 30 mins and 50 secs";
    let actual = duration(input, &Utc::now());
    assert_eq!(actual.unwrap(), Duration::new(1, 30, 50));
}

#[test]
fn a02() {
    let input = "just wait for 12 minutes and 33s";
    let actual = duration(input, &Utc::now());
    assert_eq!(actual.unwrap(), Duration::new(0, 12, 33));
}

#[test]
fn a03() {
    let input = "9h 8 min 7 sec";
    let actual = duration(input, &Utc::now());
    assert_eq!(actual.unwrap(), Duration::new(9, 8, 7));
}

#[test]
fn a04() {
    let input = "morning after 9 hrs and 11 minutes";
    let actual = duration(input, &Utc::now());
    assert_eq!(actual.unwrap(), Duration::new(9, 11, 0));
}
