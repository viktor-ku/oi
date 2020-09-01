use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn a01() {
    let input = "almost 10.h";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), Duration::new(10, 0, 0));
}

#[test]
fn a02() {
    let input = "try 1.h 3600. seconds and 30. mins";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), Duration::new(2, 30, 0));
}

#[test]
fn a03() {
    let input = ".25h 15.m 10. sec";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), Duration::new(0, 30, 10));
}
