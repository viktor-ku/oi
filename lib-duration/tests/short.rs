use chrono::Utc;
use lib_duration::{duration, Duration};

mod short_duration {
    use super::*;

    mod hours {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn alone() {
            let input = "3h";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
        }

        #[test]
        fn before_text() {
            let input = "12h and it will happen";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(12, 0, 0));
        }

        #[test]
        fn after_text() {
            let input = "just after 7h";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }

        #[test]
        fn in_the_middle() {
            let input = "just after 10h it will happen";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(10, 0, 0));
        }
    }

    mod minutes {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn alone() {
            let input = "2m";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 2, 0));
        }

        #[test]
        fn before_text() {
            let input = "4m and it will happen";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 4, 0));
        }

        #[test]
        fn after_text() {
            let input = "just after 8m";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 8, 0));
        }

        #[test]
        fn in_the_middle() {
            let input = "just after 11m it will happen";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 11, 0));
        }
    }

    mod seconds {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn alone() {
            let input = "1s";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
        }

        #[test]
        fn before_text() {
            let input = "3s and it will happen";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 3));
        }

        #[test]
        fn after_text() {
            let input = "just after 55s";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
        }

        #[test]
        fn in_the_middle() {
            let input = "just after 23s it will happen";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 23));
        }
    }
}
