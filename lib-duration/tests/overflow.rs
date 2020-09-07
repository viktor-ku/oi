use chrono::Utc;
use lib_duration::{duration, Duration};

mod seconds_into_minutes {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn with_remainder() {
        let input = "100 secs";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 1, 40));
    }

    #[test]
    fn without_remainer() {
        let input = "300 secs";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(0, 5, 0));
    }
}

mod minutes_into_hours {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn with_remainder() {
        let input = "75 minutes";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(1, 15, 0));
    }

    #[test]
    fn without_remainer() {
        let input = "180 mins";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
    }
}

mod seconds_into_minutes_into_hours {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn with_remainder() {
        let input = "75 m 100 sec";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(1, 16, 40));
    }

    #[test]
    fn without_remainer() {
        let input = "180 minutes 300 seconds";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(3, 5, 0));
    }

    #[test]
    fn a01() {
        let input = "59 mins 60 secs";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
    }

    #[test]
    fn a02() {
        let input = "1h 119 mins 60 secs";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
    }

    #[test]
    fn a03() {
        let input = "1400 sec 400 min";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(7, 3, 20));
    }

    #[test]
    fn a04() {
        let input = "4100 sec";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(1, 8, 20));
    }

    #[test]
    fn a05() {
        let input = "4243 sec 11 hr";
        let actual = duration(input, &Utc::now());
        assert_eq!(actual.unwrap(), Duration::new(12, 10, 43));
    }
}
