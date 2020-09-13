use chrono::Local;
use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn a01() {
    let input = "almost .5h";
    let actual = duration(input, &Local::now());
    assert_eq!(actual.unwrap(), Duration::new(0, 30, 0));
}

#[test]
fn a02() {
    let input = "really it is a sum .5h .75h .75h and 90m but .900 secs are ignored";
    let actual = duration(input, &Local::now());
    assert_eq!(actual.unwrap(), Duration::new(3, 30, 0));
}
