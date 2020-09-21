use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;
mod utils;

#[test]
fn text() {
    assert_eq!(
        duration(
            r###"
                One    Two     Three
                My     Timer   Is     at 
                3
                pm
            "###,
            &time! {14:00}
        ),
        Ok(Duration::new(1, 0, 0))
    );
}

mod words {
    use super::*;

    mod before {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("    someting here", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("\n\nsomething here", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("  \n  \n  someting here", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }
    }

    mod after {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("someting here    ", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("something here\n\n", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("someting here  \n  \n  ", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }
    }

    mod between {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("one       two", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(duration("one\n\ntwo", now!()), Ok(Duration::new(0, 0, 0)));
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("one  \n  \n  two", now!()),
                Ok(Duration::new(0, 0, 0))
            );
        }
    }
}

mod timeout {
    use super::*;

    mod before {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(duration("  10s", now!()), Ok(Duration::new(0, 0, 10)));
        }

        #[test]
        fn new_line() {
            assert_eq!(duration("\n10s", now!()), Ok(Duration::new(0, 0, 10)));
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("  \n  \n  10s", now!()),
                Ok(Duration::new(0, 0, 10))
            );
        }
    }

    mod after {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(duration("10s   ", now!()), Ok(Duration::new(0, 0, 10)));
        }

        #[test]
        fn new_line() {
            assert_eq!(duration("10s\n", now!()), Ok(Duration::new(0, 0, 10)));
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("10s  \n   \n   ", now!()),
                Ok(Duration::new(0, 0, 10))
            );
        }
    }

    mod between {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("10      seconds", now!()),
                Ok(Duration::new(0, 0, 10))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("10\n\nseconds", now!()),
                Ok(Duration::new(0, 0, 10))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("10  \n  \n  seconds", now!()),
                Ok(Duration::new(0, 0, 10))
            );
        }
    }

    mod many {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("   10    s     10     s      ", now!()),
                Ok(Duration::new(0, 0, 20))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("\n10\ns\n10\ns\n", now!()),
                Ok(Duration::new(0, 0, 20))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("  \n  10  \n  s  \n  10  \n  s  \n  ", now!()),
                Ok(Duration::new(0, 0, 20))
            );
        }
    }
}

mod exact_time {
    use super::*;

    mod before {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("     at 10am", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("\n\nat 10am", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("  \n  \n  at 10am", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }
    }

    mod after {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("at 10am     ", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("at 10am\n\n", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("at 10am  \n  \n  ", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }
    }

    mod between_at_and_time {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("at    10am", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("at\n\n10am", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("at  \n  \n  10am", &time! {09:00}),
                Ok(Duration::new(1, 0, 0))
            );
        }
    }

    mod between_time_and_pm {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("at 10       pm", &time! {10:00}),
                Ok(Duration::new(12, 0, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("at 10\n\npm", &time! {10:00}),
                Ok(Duration::new(12, 0, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("at 10  \n  \n  pm", &time! {10:00}),
                Ok(Duration::new(12, 0, 0))
            );
        }
    }

    mod between_time_and_pm_with_mins {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("at 10:30       pm", &time! {10:00}),
                Ok(Duration::new(12, 30, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration("at 10:30\n\npm", &time! {10:00}),
                Ok(Duration::new(12, 30, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration("at 10:30  \n  \n  pm", &time! {10:00}),
                Ok(Duration::new(12, 30, 0))
            );
        }
    }

    mod between_timeouts_and_times {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn space() {
            assert_eq!(
                duration("  10  min  at  11  am  10  min  ", &time! {10:00}),
                Ok(Duration::new(1, 20, 0))
            );
        }

        #[test]
        fn new_line() {
            assert_eq!(
                duration(
                    "\n\n10\n\nmin\n\nat\n\n11\n\nam\n\n10\n\nmin\n\n",
                    &time! {10:00}
                ),
                Ok(Duration::new(1, 20, 0))
            );
        }

        #[test]
        fn mixed() {
            assert_eq!(
                duration(
                    r###"

                    10

                    min

                    at

                    11

                    am

                    10

                    min

                    "###,
                    &time! {10:00}
                ),
                Ok(Duration::new(1, 20, 0))
            );
        }
    }
}
