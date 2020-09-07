use chrono::Utc;
use lib_duration::duration;
use pretty_assertions::assert_eq;

#[test]
fn a01() {
    let input = "1 sec";
    let d = duration(input, &Utc::now()).unwrap();
    assert_eq!(d.secs(), 1);
}

#[test]
fn a02() {
    let input = "1 min";
    let d = duration(input, &Utc::now()).unwrap();
    assert_eq!(d.secs(), 60);
}

#[test]
fn a03() {
    let input = "1 hour";
    let d = duration(input, &Utc::now()).unwrap();
    assert_eq!(d.secs(), 3600);
}

#[test]
fn a04() {
    let input = "1h 1m 1s";
    let d = duration(input, &Utc::now()).unwrap();
    assert_eq!(d.secs(), 3661);
}

#[test]
fn a05() {
    let input = "25m 330 seconds";
    let d = duration(input, &Utc::now()).unwrap();
    assert_eq!(d.secs(), 1830);
}
