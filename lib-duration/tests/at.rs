use chrono::{Local, TimeZone};
use lib_duration::{duration, Duration};

#[macro_use]
mod utils;

mod h12_format {
    use super::*;
    use pretty_assertions::assert_eq;

    mod next_day {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn at_0am() {
            assert_eq!(
                duration("at 0am", &time! {20:30}),
                Ok(Duration::new(3, 30, 0))
            );
        }

        #[test]
        fn at_12am() {
            assert_eq!(
                duration("at 12am", &time! {21:00}),
                Ok(Duration::new(3, 0, 0))
            );
        }

        #[test]
        fn at_1am() {
            assert_eq!(
                duration("at 1am", &time! {01:30}),
                Ok(Duration::new(23, 30, 0))
            );
        }

        #[test]
        fn at_2am() {
            assert_eq!(
                duration("at 2 am", &time! {12:00}),
                Ok(Duration::new(14, 0, 0))
            );
        }

        #[test]
        fn at_3am() {
            assert_eq!(
                duration("at 3am", &time! {20:00}),
                Ok(Duration::new(7, 0, 0))
            );
        }

        #[test]
        fn at_4am() {
            assert_eq!(
                duration("at 4am", &time! {20:00}),
                Ok(Duration::new(8, 0, 0))
            );
        }

        #[test]
        fn at_5am() {
            assert_eq!(
                duration("at 5am", &time! {20:00}),
                Ok(Duration::new(9, 0, 0))
            );
        }

        #[test]
        fn at_6am() {
            assert_eq!(
                duration("at 6am", &time! {20:00}),
                Ok(Duration::new(10, 0, 0))
            );
        }

        #[test]
        fn at_7am() {
            assert_eq!(
                duration("at 7am", &time! {20:00}),
                Ok(Duration::new(11, 0, 0))
            );
        }

        #[test]
        fn at_8am() {
            assert_eq!(
                duration("at 8am", &time! {20:00}),
                Ok(Duration::new(12, 0, 0))
            );
        }

        #[test]
        fn at_9am() {
            assert_eq!(
                duration("at 9am", &time! {20:00}),
                Ok(Duration::new(13, 0, 0))
            );
        }

        #[test]
        fn at_10am() {
            assert_eq!(
                duration("at 10 am", &time! {20:00}),
                Ok(Duration::new(14, 0, 0))
            );
        }

        #[test]
        fn at_11am() {
            assert_eq!(
                duration("at 11am", &time! {20:00}),
                Ok(Duration::new(15, 0, 0))
            );
        }

        #[test]
        fn at_12pm() {
            assert_eq!(
                duration("at 12 pm", &time! {20:00}),
                Ok(Duration::new(16, 0, 0))
            );
        }

        #[test]
        fn at_0pm() {
            assert_eq!(
                duration("at 0pm", &time! {20:00}),
                Ok(Duration::new(16, 0, 0))
            );
        }

        #[test]
        fn at_1pm() {
            assert_eq!(
                duration("at 1pm", &time! {20:00}),
                Ok(Duration::new(17, 0, 0))
            );
        }

        #[test]
        fn at_2pm() {
            assert_eq!(
                duration("at 2pm", &time! {20:00}),
                Ok(Duration::new(18, 0, 0))
            );
        }

        #[test]
        fn at_3pm() {
            assert_eq!(
                duration("at 3pm", &time! {20:00}),
                Ok(Duration::new(19, 0, 0))
            );
        }

        #[test]
        fn at_4pm() {
            assert_eq!(
                duration("at 4pm", &time! {20:00}),
                Ok(Duration::new(20, 0, 0))
            );
        }

        #[test]
        fn at_5pm() {
            assert_eq!(
                duration("at 5pm", &time! {20:00}),
                Ok(Duration::new(21, 0, 0))
            );
        }

        #[test]
        fn at_6pm() {
            assert_eq!(
                duration("at 6pm", &time! {20:00}),
                Ok(Duration::new(22, 0, 0))
            );
        }

        #[test]
        fn at_7pm() {
            assert_eq!(
                duration("at 7pm", &time! {20:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_8pm() {
            assert_eq!(
                duration("at 8pm", &time! {21:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_9pm() {
            assert_eq!(
                duration("at 9pm", &time! {22:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_10pm() {
            assert_eq!(
                duration("at 10pm", &time! {23:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_11pm() {
            assert_eq!(
                duration("at 11pm", &time! {23:20}),
                Ok(Duration::new(23, 40, 0))
            );
        }
    }

    #[test]
    fn long_variant_should_work_too() {
        let input = "at 1 am";
        let now = Local.ymd(2020, 9, 30).and_hms(0, 39, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 21, 0)));
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
}

mod h24_format {
    use super::*;
    use pretty_assertions::assert_eq;

    mod next_day {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn at_0() {
            assert_eq!(
                duration("at 0", &time! {20:30}),
                Ok(Duration::new(3, 30, 0))
            );
        }

        #[test]
        fn at_24() {
            assert_eq!(
                duration("at 24", &time! {20:30}),
                Ok(Duration::new(3, 30, 0))
            );
        }

        #[test]
        fn at_1() {
            assert_eq!(
                duration("at 1", &time! {01:30}),
                Ok(Duration::new(23, 30, 0))
            );
        }

        #[test]
        fn at_2() {
            assert_eq!(
                duration("at 2", &time! {12:00}),
                Ok(Duration::new(14, 0, 0))
            );
        }

        #[test]
        fn at_3() {
            assert_eq!(duration("at 3", &time! {20:00}), Ok(Duration::new(7, 0, 0)));
        }

        #[test]
        fn at_4() {
            assert_eq!(duration("at 4", &time! {20:00}), Ok(Duration::new(8, 0, 0)));
        }

        #[test]
        fn at_5() {
            assert_eq!(duration("at 5", &time! {20:00}), Ok(Duration::new(9, 0, 0)));
        }

        #[test]
        fn at_6() {
            assert_eq!(
                duration("at 6", &time! {20:00}),
                Ok(Duration::new(10, 0, 0))
            );
        }

        #[test]
        fn at_7() {
            assert_eq!(
                duration("at 7", &time! {20:00}),
                Ok(Duration::new(11, 0, 0))
            );
        }

        #[test]
        fn at_8() {
            assert_eq!(
                duration("at 8", &time! {20:00}),
                Ok(Duration::new(12, 0, 0))
            );
        }

        #[test]
        fn at_9() {
            assert_eq!(
                duration("at 9", &time! {20:00}),
                Ok(Duration::new(13, 0, 0))
            );
        }

        #[test]
        fn at_10() {
            assert_eq!(
                duration("at 10", &time! {20:00}),
                Ok(Duration::new(14, 0, 0))
            );
        }

        #[test]
        fn at_11() {
            assert_eq!(
                duration("at 11", &time! {20:00}),
                Ok(Duration::new(15, 0, 0))
            );
        }

        #[test]
        fn at_12() {
            assert_eq!(
                duration("at 12", &time! {20:00}),
                Ok(Duration::new(16, 0, 0))
            );
        }

        #[test]
        fn at_13() {
            assert_eq!(
                duration("at 13", &time! {20:00}),
                Ok(Duration::new(17, 0, 0))
            );
        }

        #[test]
        fn at_14() {
            assert_eq!(
                duration("at 14", &time! {20:00}),
                Ok(Duration::new(18, 0, 0))
            );
        }

        #[test]
        fn at_15() {
            assert_eq!(
                duration("at 15", &time! {20:00}),
                Ok(Duration::new(19, 0, 0))
            );
        }

        #[test]
        fn at_16() {
            assert_eq!(
                duration("at 16", &time! {20:00}),
                Ok(Duration::new(20, 0, 0))
            );
        }

        #[test]
        fn at_17() {
            assert_eq!(
                duration("at 17", &time! {20:00}),
                Ok(Duration::new(21, 0, 0))
            );
        }

        #[test]
        fn at_18() {
            assert_eq!(
                duration("at 18", &time! {20:00}),
                Ok(Duration::new(22, 0, 0))
            );
        }

        #[test]
        fn at_19() {
            assert_eq!(
                duration("at 19", &time! {20:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_20() {
            assert_eq!(
                duration("at 20", &time! {21:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_21() {
            assert_eq!(
                duration("at 21", &time! {22:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_22() {
            assert_eq!(
                duration("at 22", &time! {23:00}),
                Ok(Duration::new(23, 0, 0))
            );
        }

        #[test]
        fn at_23() {
            assert_eq!(
                duration("at 23", &time! {23:20}),
                Ok(Duration::new(23, 40, 0))
            );
        }
    }

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
