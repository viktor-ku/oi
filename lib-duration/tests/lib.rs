use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn no_duration_one_word() {
    let input = "yo";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), None);
}

#[test]
fn no_duration_many_word() {
    let input = "one two three no duration you can see";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), None);
}
mod short_duration {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn just_one_duration() {
        let input = "1m";
        let actual = duration(input);
        assert_eq!(actual.unwrap(), Some(Duration::new(0, 1, 0)));
    }
}
