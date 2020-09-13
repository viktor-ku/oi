use chrono::{Local, TimeZone};
use lib_duration::{duration, Duration};

mod with_part {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn long_variant_should_work_too() {
        let input = "at 0 am";
        let now = Local.ymd(2020, 9, 30).and_hms(23, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 15, 0)));
    }

    #[test]
    fn at_0am() {
        let input = "at 0am";
        let now = Local.ymd(2020, 9, 30).and_hms(23, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 15, 0)));
    }

    #[test]
    fn at_1am() {
        let input = "at 1am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 40, 0)));
    }

    #[test]
    fn at_2am() {
        let input = "at 2am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(1, 40, 0)));
    }

    #[test]
    fn at_3am() {
        let input = "at 3am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(2, 40, 0)));
    }

    #[test]
    fn at_4am() {
        let input = "at 4am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(3, 40, 0)));
    }

    #[test]
    fn at_5am() {
        let input = "at 5am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(4, 40, 0)));
    }

    #[test]
    fn at_6am() {
        let input = "at 6am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(5, 40, 0)));
    }

    #[test]
    fn at_7am() {
        let input = "at 7am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(6, 40, 0)));
    }

    #[test]
    fn at_8am() {
        let input = "at 8am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(7, 40, 0)));
    }

    #[test]
    fn at_9am() {
        let input = "at 9am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(8, 40, 0)));
    }

    #[test]
    fn at_10am() {
        let input = "at 10am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(9, 40, 0)));
    }

    #[test]
    fn at_11am() {
        let input = "at 11am";
        let now = Local.ymd(2020, 9, 1).and_hms(0, 20, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(10, 40, 0)));
    }

    #[test]
    fn at_12pm() {
        let input = "at 12pm";
        let now = Local.ymd(2020, 9, 1).and_hms(5, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(6, 15, 0)));
    }

    #[test]
    fn at_0pm() {
        let input = "at 0pm";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 50, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(2, 10, 0)));
    }

    #[test]
    fn at_1pm() {
        let input = "at 1pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 30, 0)));
    }

    #[test]
    fn at_2pm() {
        let input = "at 2pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(1, 30, 0)));
    }

    #[test]
    fn at_3pm() {
        let input = "at 3pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(2, 30, 0)));
    }

    #[test]
    fn at_4pm() {
        let input = "at 4pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(3, 30, 0)));
    }

    #[test]
    fn at_5pm() {
        let input = "at 5pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(4, 30, 0)));
    }

    #[test]
    fn at_6pm() {
        let input = "at 6pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(5, 30, 0)));
    }

    #[test]
    fn at_7pm() {
        let input = "at 7pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(6, 30, 0)));
    }

    #[test]
    fn at_8pm() {
        let input = "at 8pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(7, 30, 0)));
    }

    #[test]
    fn at_9pm() {
        let input = "at 9pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(8, 30, 0)));
    }

    #[test]
    fn at_10pm() {
        let input = "at 10pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(9, 30, 0)));
    }

    #[test]
    fn at_11pm() {
        let input = "at 11pm";
        let now = Local.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(10, 30, 0)));
    }

    #[test]
    fn at_12am() {
        let input = "at 12am";
        let now = Local.ymd(2020, 9, 30).and_hms(23, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 15, 0)));
    }
}
