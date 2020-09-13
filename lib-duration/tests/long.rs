use chrono::Utc;
use lib_duration::{duration, Duration};

mod hours {
    use super::*;

    mod alone {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn hour() {
            let input = "1 hour";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
        }

        #[test]
        fn hours() {
            let input = "42 hours";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(42, 0, 0));
        }

        #[test]
        fn hr() {
            let input = "2 hr";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(2, 0, 0));
        }

        #[test]
        fn hrs() {
            let input = "7 hrs";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }

        #[test]
        fn h() {
            let input = "3 h";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
        }
    }

    mod before_text {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn hour() {
            let input = "1 hour before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
        }

        #[test]
        fn hours() {
            let input = "11 hours before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(11, 0, 0));
        }

        #[test]
        fn hr() {
            let input = "9 hr before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(9, 0, 0));
        }

        #[test]
        fn hrs() {
            let input = "7 hrs before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }

        #[test]
        fn h() {
            let input = "7 h before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }
    }

    mod after_text {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn hour() {
            let input = "you shall spend 1 hour";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
        }

        #[test]
        fn hours() {
            let input = "you shall spend 3 hours";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
        }

        #[test]
        fn hr() {
            let input = "you shall spend 2 hr";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(2, 0, 0));
        }

        #[test]
        fn hrs() {
            let input = "you shall spend 7 hrs";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }

        #[test]
        fn h() {
            let input = "you shall spend 4 h";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(4, 0, 0));
        }
    }

    mod in_the_middle {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn hour() {
            let input = "you shall spend 1 hour on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
        }

        #[test]
        fn hours() {
            let input = "you shall spend 3 hours on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
        }

        #[test]
        fn hr() {
            let input = "you shall spend 2 hr on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(2, 0, 0));
        }

        #[test]
        fn hrs() {
            let input = "you shall spend 7 hrs on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }

        #[test]
        fn h() {
            let input = "you shall spend 4 h on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(4, 0, 0));
        }
    }
}

mod minutes {
    use super::*;

    mod alone {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn minute() {
            let input = "1 minute";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
        }

        #[test]
        fn minutes() {
            let input = "42 minutes";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 42, 0));
        }

        #[test]
        fn mins() {
            let input = "33 mins";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 33, 0));
        }

        #[test]
        fn min() {
            let input = "2 min";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 2, 0));
        }

        #[test]
        fn m() {
            let input = "3 m";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 3, 0));
        }
    }

    mod before_text {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn minute() {
            let input = "1 minute before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
        }

        #[test]
        fn minutes() {
            let input = "11 minutes before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 11, 0));
        }

        #[test]
        fn mins() {
            let input = "33 mins before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 33, 0));
        }

        #[test]
        fn min() {
            let input = "2 min before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 2, 0));
        }

        #[test]
        fn m() {
            let input = "7 m before other text";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 7, 0));
        }
    }

    mod after_text {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn minute() {
            let input = "you shall spend 1 minute";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
        }

        #[test]
        fn minutes() {
            let input = "you shall spend 3 minutes";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 3, 0));
        }

        #[test]
        fn mins() {
            let input = "you shall spend 33 mins";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 33, 0));
        }

        #[test]
        fn min() {
            let input = "you shall spend 2 min";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 2, 0));
        }

        #[test]
        fn m() {
            let input = "you shall spend 4 m";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 4, 0));
        }
    }

    mod in_the_middle {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn minute() {
            let input = "you shall spend 1 minute on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
        }

        #[test]
        fn minutes() {
            let input = "you shall spend 3 minutes on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 3, 0));
        }

        #[test]
        fn mins() {
            let input = "you shall spend 33 mins on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 33, 0));
        }

        #[test]
        fn min() {
            let input = "you shall spend 2 min on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 2, 0));
        }

        #[test]
        fn m() {
            let input = "you shall spend 4 m on this";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 4, 0));
        }
    }
}

mod seconds {
    use super::*;

    mod alone {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn second() {
            let input = "1 second";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
        }

        #[test]
        fn seconds() {
            let input = "42 seconds";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
        }

        #[test]
        fn secs() {
            let input = "12 secs";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
        }

        #[test]
        fn sec() {
            let input = "9 sec";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
        }

        #[test]
        fn s() {
            let input = "55 s";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
        }
    }

    mod before_text {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn second() {
            let input = "1 second more or less";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
        }

        #[test]
        fn seconds() {
            let input = "42 seconds more or less";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
        }

        #[test]
        fn secs() {
            let input = "12 secs more or less";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
        }

        #[test]
        fn sec() {
            let input = "9 sec more or less";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
        }

        #[test]
        fn s() {
            let input = "55 s more or less";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
        }
    }

    mod after_text {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn second() {
            let input = "more or less 1 second";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
        }

        #[test]
        fn seconds() {
            let input = "more or less 42 seconds";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
        }

        #[test]
        fn secs() {
            let input = "more or less 12 secs";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
        }

        #[test]
        fn sec() {
            let input = "more or less 9 sec";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
        }

        #[test]
        fn s() {
            let input = "more or less 55 s";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
        }
    }

    mod in_the_middle {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn second() {
            let input = "more or less 1 second but who really knows";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
        }

        #[test]
        fn seconds() {
            let input = "more or less 42 seconds but who really knows";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
        }

        #[test]
        fn secs() {
            let input = "more or less 12 secs but who really knows";
            let actual = duration(input, &Utc::now());
            println!("{:#?}", actual);
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
        }

        #[test]
        fn sec() {
            let input = "more or less 9 sec but who really knows";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
        }

        #[test]
        fn s() {
            let input = "more or less 55 s but who really knows";
            let actual = duration(input, &Utc::now());
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
        }
    }
}
