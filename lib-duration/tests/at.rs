use chrono::{Local, TimeZone};
use lib_duration::{duration, Duration};

mod h12_format {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn long_variant_should_work_too() {
        let input = "at 1 am";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 39, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 21, 0)));
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

mod h24_format {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn at_1() {
        let input = "at 1";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 15, 0)));
    }

    #[test]
    fn at_2() {
        let input = "at 2";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(1, 15, 0)));
    }

    #[test]
    fn at_3() {
        let input = "at 3";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(2, 15, 0)));
    }

    #[test]
    fn at_4() {
        let input = "at 4";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(3, 15, 0)));
    }

    #[test]
    fn at_5() {
        let input = "at 5";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(4, 15, 0)));
    }

    #[test]
    fn at_6() {
        let input = "at 6";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(5, 15, 0)));
    }

    #[test]
    fn at_7() {
        let input = "at 7";
        let now = Local.ymd(2020, 9, 30).and_hms(5, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(1, 15, 0)));
    }

    #[test]
    fn at_8() {
        let input = "at 8";
        let now = Local.ymd(2020, 9, 30).and_hms(5, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(2, 15, 0)));
    }

    #[test]
    fn at_9() {
        let input = "at 9";
        let now = Local.ymd(2020, 9, 30).and_hms(5, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(3, 15, 0)));
    }

    #[test]
    fn at_10() {
        let input = "at 10";
        let now = Local.ymd(2020, 9, 30).and_hms(5, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(4, 15, 0)));
    }

    #[test]
    fn at_11() {
        let input = "at 11";
        let now = Local.ymd(2020, 9, 30).and_hms(5, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(5, 15, 0)));
    }

    #[test]
    fn at_12() {
        let input = "at 12";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(2, 15, 0)));
    }

    #[test]
    fn at_13() {
        let input = "at 13";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(3, 15, 0)));
    }

    #[test]
    fn at_14() {
        let input = "at 14";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(4, 15, 0)));
    }

    #[test]
    fn at_15() {
        let input = "at 15";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(5, 15, 0)));
    }

    #[test]
    fn at_16() {
        let input = "at 16";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(6, 15, 0)));
    }

    #[test]
    fn at_17() {
        let input = "at 17";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(7, 15, 0)));
    }

    #[test]
    fn at_18() {
        let input = "at 18";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(8, 15, 0)));
    }

    #[test]
    fn at_19() {
        let input = "at 19";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(9, 15, 0)));
    }

    #[test]
    fn at_20() {
        let input = "at 20";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(10, 15, 0)));
    }

    #[test]
    fn at_21() {
        let input = "at 21";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(11, 15, 0)));
    }

    #[test]
    fn at_22() {
        let input = "at 22";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(12, 15, 0)));
    }

    #[test]
    fn at_23() {
        let input = "at 23";
        let now = Local.ymd(2020, 9, 30).and_hms(9, 45, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(13, 15, 0)));
    }
}
