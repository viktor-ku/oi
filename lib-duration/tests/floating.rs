use chrono::Utc;
use lib_duration::{duration, Duration};

mod seconds {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ignore_short_seconds() {
        let input = "2.35s";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 0, 2));
    }

    #[test]
    fn ignore_long_seconds() {
        let input = "9.0050 secs";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
    }
}

mod minutes {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a01() {
        let input = "1.5m";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 1, 30));
    }

    #[test]
    fn a02() {
        let input = "1.99m";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 1, 59));
    }

    #[test]
    fn a03() {
        let input = "1.84m";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 1, 50));
    }

    #[test]
    fn a04() {
        let input = "101.5 min";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(1, 41, 30));
    }
}

mod hours {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a01() {
        let input = "1.5 hours";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(1, 30, 0));
    }

    #[test]
    fn a02() {
        let input = "0.495101 hours";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 29, 0));
    }
}
