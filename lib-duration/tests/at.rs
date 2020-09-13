use lib_duration::{duration, Duration};

#[macro_use]
mod utils;

mod midnight_business {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn at_0_45_am() {
        assert_eq!(
            duration("at 0:45 am", &time! {00:30-50}),
            Ok(Duration::new(0, 14, 10))
        );
    }

    #[test]
    fn at_12_45_am() {
        assert_eq!(
            duration("at 12:45am", &time! {00:30-50}),
            Ok(Duration::new(0, 14, 10))
        );
    }

    #[test]
    fn at_0_45() {
        assert_eq!(
            duration("at 0:45", &time! {00:30-50}),
            Ok(Duration::new(0, 14, 10))
        );
    }

    #[test]
    fn at_24_45() {
        assert_eq!(
            duration("at 24:45", &time! {00:30-50}),
            Ok(Duration::new(0, 14, 10))
        );
    }
}

mod with_input_minutes {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn at_18_30() {
        assert_eq!(
            duration("at 18:30", &time! {18:10-30}),
            Ok(Duration::new(0, 19, 30))
        );
    }

    #[test]
    fn at_4_15_am() {
        assert_eq!(
            duration("at 4:15am", &time! {16:15}),
            Ok(Duration::new(12, 0, 0))
        );
    }

    #[test]
    fn at_6_30_pm_long() {
        assert_eq!(
            duration("at 6:30 pm", &time! {18:10-30}),
            Ok(Duration::new(0, 19, 30))
        );
    }

    #[test]
    fn at_6_30_pm_short() {
        assert_eq!(
            duration("at 6:30pm", &time! {18:10-30}),
            Ok(Duration::new(0, 19, 30))
        );
    }
}

mod with_time_seconds {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn at_1pm() {
        assert_eq!(
            duration("at 1pm", &time! {12:05-30}),
            Ok(Duration::new(0, 54, 30))
        );
    }

    #[test]
    fn at_18() {
        assert_eq!(
            duration("at 18", &time! {17:59-50}),
            Ok(Duration::new(0, 0, 10))
        );
    }
}

mod h12_format {
    use super::*;
    use pretty_assertions::assert_eq;

    mod uppercase_works {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn at_1am_long() {
            assert_eq!(
                duration("at 1 AM", &time! {00:30-10}),
                Ok(Duration::new(0, 29, 50))
            );
        }

        #[test]
        fn at_1am_short() {
            assert_eq!(
                duration("at 1AM", &time! {00:30-10}),
                Ok(Duration::new(0, 29, 50))
            );
        }

        #[test]
        fn at_1pm_long() {
            assert_eq!(
                duration("at 1 PM", &time! {12:20-01}),
                Ok(Duration::new(0, 39, 59))
            );
        }

        #[test]
        fn at_1pm_short() {
            assert_eq!(
                duration("at 1PM", &time! {12:20-01}),
                Ok(Duration::new(0, 39, 59))
            );
        }
    }

    mod weird_cases {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn at_13am() {
            assert_eq!(
                duration("at 13am", &time! {10:00}),
                Ok(Duration::new(3, 0, 0))
            );
        }

        #[test]
        fn at_13pm() {
            assert_eq!(
                duration("at 13pm", &time! {10:00}),
                Ok(Duration::new(3, 0, 0))
            );
        }

        #[test]
        fn at_24am() {
            assert_eq!(
                duration("at 24am", &time! {22:00}),
                Ok(Duration::new(2, 0, 0))
            );
        }

        #[test]
        fn at_24pm() {
            assert_eq!(
                duration("at 24pm", &time! {22:00}),
                Ok(Duration::new(2, 0, 0))
            );
        }
    }

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
    fn at_1am() {
        assert_eq!(
            duration("at 1am", &time! {00:20}),
            Ok(Duration::new(0, 40, 0))
        );
    }

    #[test]
    fn at_1am_long() {
        assert_eq!(
            duration("at 1 am", &time! {00:39}),
            Ok(Duration::new(0, 21, 0))
        );
    }

    #[test]
    fn at_2am() {
        assert_eq!(
            duration("at 2am", &time! {00:20}),
            Ok(Duration::new(1, 40, 0))
        );
    }

    #[test]
    fn at_3am() {
        assert_eq!(
            duration("at 3am", &time! {00:20}),
            Ok(Duration::new(2, 40, 0))
        );
    }

    #[test]
    fn at_4am() {
        assert_eq!(
            duration("at 4am", &time! {00:20}),
            Ok(Duration::new(3, 40, 0))
        );
    }

    #[test]
    fn at_5am() {
        assert_eq!(
            duration("at 5am", &time! {00:20}),
            Ok(Duration::new(4, 40, 0))
        );
    }

    #[test]
    fn at_6am() {
        assert_eq!(
            duration("at 6am", &time! {00:20}),
            Ok(Duration::new(5, 40, 0))
        );
    }

    #[test]
    fn at_7am() {
        assert_eq!(
            duration("at 7am", &time! {00:20}),
            Ok(Duration::new(6, 40, 0))
        );
    }

    #[test]
    fn at_8am() {
        assert_eq!(
            duration("at 8am", &time! {00:20}),
            Ok(Duration::new(7, 40, 0))
        );
    }

    #[test]
    fn at_9am() {
        assert_eq!(
            duration("at 9am", &time! {00:20}),
            Ok(Duration::new(8, 40, 0))
        );
    }

    #[test]
    fn at_10am() {
        assert_eq!(
            duration("at 10am", &time! {00:20}),
            Ok(Duration::new(9, 40, 0))
        );
    }

    #[test]
    fn at_11am() {
        assert_eq!(
            duration("at 11am", &time! {00:20}),
            Ok(Duration::new(10, 40, 0))
        );
    }

    #[test]
    fn at_12pm() {
        assert_eq!(
            duration("at 12pm", &time! {05:45}),
            Ok(Duration::new(6, 15, 0))
        );
    }

    #[test]
    fn at_0pm() {
        assert_eq!(
            duration("at 0pm", &time! {09:50}),
            Ok(Duration::new(2, 10, 0))
        );
    }

    #[test]
    fn at_1pm() {
        assert_eq!(
            duration("at 1pm", &time! {12:30}),
            Ok(Duration::new(0, 30, 0))
        );
    }

    #[test]
    fn at_2pm() {
        assert_eq!(
            duration("at 2pm", &time! {12:30}),
            Ok(Duration::new(1, 30, 0))
        );
    }

    #[test]
    fn at_3pm() {
        assert_eq!(
            duration("at 3pm", &time! {12:30}),
            Ok(Duration::new(2, 30, 0))
        );
    }

    #[test]
    fn at_4pm() {
        assert_eq!(
            duration("at 4pm", &time! {12:30}),
            Ok(Duration::new(3, 30, 0))
        );
    }

    #[test]
    fn at_5pm() {
        assert_eq!(
            duration("at 5pm", &time! {12:30}),
            Ok(Duration::new(4, 30, 0))
        );
    }

    #[test]
    fn at_6pm() {
        assert_eq!(
            duration("at 6pm", &time! {12:30}),
            Ok(Duration::new(5, 30, 0))
        );
    }

    #[test]
    fn at_7pm() {
        assert_eq!(
            duration("at 7pm", &time! {12:30}),
            Ok(Duration::new(6, 30, 0))
        );
    }

    #[test]
    fn at_8pm() {
        assert_eq!(
            duration("at 8pm", &time! {12:30}),
            Ok(Duration::new(7, 30, 0))
        );
    }

    #[test]
    fn at_9pm() {
        assert_eq!(
            duration("at 9pm", &time! {12:30}),
            Ok(Duration::new(8, 30, 0))
        );
    }

    #[test]
    fn at_10pm() {
        assert_eq!(
            duration("at 10pm", &time! {12:30}),
            Ok(Duration::new(9, 30, 0))
        );
    }

    #[test]
    fn at_11pm() {
        assert_eq!(
            duration("at 11pm", &time! {12:30}),
            Ok(Duration::new(10, 30, 0))
        );
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
        assert_eq!(
            duration("at 1", &time! {00:45}),
            Ok(Duration::new(0, 15, 0))
        );
    }

    #[test]
    fn at_2() {
        assert_eq!(
            duration("at 2", &time! {00:45}),
            Ok(Duration::new(1, 15, 0))
        );
    }

    #[test]
    fn at_3() {
        assert_eq!(
            duration("at 3", &time! {00:45}),
            Ok(Duration::new(2, 15, 0))
        );
    }

    #[test]
    fn at_4() {
        assert_eq!(
            duration("at 4", &time! {00:45}),
            Ok(Duration::new(3, 15, 0))
        );
    }

    #[test]
    fn at_5() {
        assert_eq!(
            duration("at 5", &time! {00:45}),
            Ok(Duration::new(4, 15, 0))
        );
    }

    #[test]
    fn at_6() {
        assert_eq!(
            duration("at 6", &time! {00:45}),
            Ok(Duration::new(5, 15, 0))
        );
    }

    #[test]
    fn at_7() {
        assert_eq!(
            duration("at 7", &time! {05:45}),
            Ok(Duration::new(1, 15, 0))
        );
    }

    #[test]
    fn at_8() {
        assert_eq!(
            duration("at 8", &time! {05:45}),
            Ok(Duration::new(2, 15, 0))
        );
    }

    #[test]
    fn at_9() {
        assert_eq!(
            duration("at 9", &time! {05:45}),
            Ok(Duration::new(3, 15, 0))
        );
    }

    #[test]
    fn at_10() {
        assert_eq!(
            duration("at 10", &time! {05:45}),
            Ok(Duration::new(4, 15, 0))
        );
    }

    #[test]
    fn at_11() {
        assert_eq!(
            duration("at 11", &time! {05:45}),
            Ok(Duration::new(5, 15, 0))
        );
    }

    #[test]
    fn at_12() {
        assert_eq!(
            duration("at 12", &time! {09:45}),
            Ok(Duration::new(2, 15, 0))
        );
    }

    #[test]
    fn at_13() {
        assert_eq!(
            duration("at 13", &time! {09:45}),
            Ok(Duration::new(3, 15, 0))
        );
    }

    #[test]
    fn at_14() {
        assert_eq!(
            duration("at 14", &time! {09:45}),
            Ok(Duration::new(4, 15, 0))
        );
    }

    #[test]
    fn at_15() {
        assert_eq!(
            duration("at 15", &time! {09:45}),
            Ok(Duration::new(5, 15, 0))
        );
    }

    #[test]
    fn at_16() {
        assert_eq!(
            duration("at 16", &time! {09:45}),
            Ok(Duration::new(6, 15, 0))
        );
    }

    #[test]
    fn at_17() {
        assert_eq!(
            duration("at 17", &time! {09:45}),
            Ok(Duration::new(7, 15, 0))
        );
    }

    #[test]
    fn at_18() {
        assert_eq!(
            duration("at 18", &time! {09:45}),
            Ok(Duration::new(8, 15, 0))
        );
    }

    #[test]
    fn at_19() {
        assert_eq!(
            duration("at 19", &time! {09:45}),
            Ok(Duration::new(9, 15, 0))
        );
    }

    #[test]
    fn at_20() {
        assert_eq!(
            duration("at 20", &time! {09:45}),
            Ok(Duration::new(10, 15, 0))
        );
    }

    #[test]
    fn at_21() {
        assert_eq!(
            duration("at 21", &time! {09:45}),
            Ok(Duration::new(11, 15, 0))
        );
    }

    #[test]
    fn at_22() {
        assert_eq!(
            duration("at 22", &time! {09:45}),
            Ok(Duration::new(12, 15, 0))
        );
    }

    #[test]
    fn at_23() {
        assert_eq!(
            duration("at 23", &time! {09:45}),
            Ok(Duration::new(13, 15, 0))
        );
    }
}
