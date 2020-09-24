use lib_duration::duration;
use pretty_assertions::assert_eq;

mod utils;

#[test]
fn one_second() {
    assert_eq!(duration("1 second", now!()).unwrap().total(), 1_000);
}

#[test]
fn one_minute() {
    assert_eq!(duration("1 minute", now!()).unwrap().total(), 1_000 * 60);
}

#[test]
fn one_hour() {
    assert_eq!(duration("1 hour", now!()).unwrap().total(), 1_000 * 60 * 60);
}

#[test]
fn one_hour_one_minute_one_second() {
    assert_eq!(
        duration("1 hour 1 minute 1 second", now!())
            .unwrap()
            .total(),
        1_000 + (1_000 * 60) + (1_000 * 60 * 60)
    );
}
