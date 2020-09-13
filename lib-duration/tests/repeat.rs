use chrono::Local;
use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn almost_three_hours() {
    let input = "1m 9.0m 100m 69.37900m";
    let actual = duration(input, &Local::now());
    assert_eq!(actual.unwrap(), Duration::new(2, 59, 22));
}

#[test]
fn four_hours_total() {
    let input = "1h 2.5h 20m 5m 2.5m 2.5m";
    let actual = duration(input, &Local::now());
    assert_eq!(actual.unwrap(), Duration::new(4, 0, 0));
}
