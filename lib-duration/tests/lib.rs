use lib_duration::{duration, Duration};
use pretty_assertions::assert_eq;

#[test]
fn no_duration_one_word() {
    let input = "yo";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), Duration::default());
}

#[test]
fn no_duration_in_the_sentence() {
    let input = "one two three no duration you can see";
    let actual = duration(input);
    assert_eq!(actual.unwrap(), Duration::default());
}

mod short_duration {
    use super::*;

    mod hours {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn alone() {
            let input = "3h";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
        }

        #[test]
        fn before_text() {
            let input = "12h and it will happen";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(12, 0, 0));
        }

        #[test]
        fn after_text() {
            let input = "just after 7h";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
        }

        #[test]
        fn in_the_middle() {
            let input = "just after 10h it will happen";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(10, 0, 0));
        }
    }

    mod minutes {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn alone() {
            let input = "2m";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 2, 0));
        }

        #[test]
        fn before_text() {
            let input = "4m and it will happen";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 4, 0));
        }

        #[test]
        fn after_text() {
            let input = "just after 8m";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 8, 0));
        }

        #[test]
        fn in_the_middle() {
            let input = "just after 11m it will happen";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 11, 0));
        }
    }

    mod seconds {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn alone() {
            let input = "1s";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
        }

        #[test]
        fn before_text() {
            let input = "3s and it will happen";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 3));
        }

        #[test]
        fn after_text() {
            let input = "just after 55s";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
        }

        #[test]
        fn in_the_middle() {
            let input = "just after 23s it will happen";
            let actual = duration(input);
            assert_eq!(actual.unwrap(), Duration::new(0, 0, 23));
        }
    }
}

mod long_duration {
    use super::*;

    mod hours {
        use super::*;

        mod alone {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn hour() {
                let input = "1 hour";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
            }

            #[test]
            fn hours() {
                let input = "42 hours";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(42, 0, 0));
            }

            #[test]
            fn hr() {
                let input = "2 hr";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(2, 0, 0));
            }

            #[test]
            fn h() {
                let input = "3 h";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
            }
        }

        mod before_text {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn hour() {
                let input = "1 hour before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
            }

            #[test]
            fn hours() {
                let input = "11 hours before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(11, 0, 0));
            }

            #[test]
            fn hr() {
                let input = "9 hr before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(9, 0, 0));
            }

            #[test]
            fn h() {
                let input = "7 h before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(7, 0, 0));
            }
        }

        mod after_text {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn hour() {
                let input = "you shall spend 1 hour";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
            }

            #[test]
            fn hours() {
                let input = "you shall spend 3 hours";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
            }

            #[test]
            fn hr() {
                let input = "you shall spend 2 hr";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(2, 0, 0));
            }

            #[test]
            fn h() {
                let input = "you shall spend 4 h";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(4, 0, 0));
            }
        }

        mod in_the_middle {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn hour() {
                let input = "you shall spend 1 hour on this";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(1, 0, 0));
            }

            #[test]
            fn hours() {
                let input = "you shall spend 3 hours on this";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(3, 0, 0));
            }

            #[test]
            fn hr() {
                let input = "you shall spend 2 hr on this";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(2, 0, 0));
            }

            #[test]
            fn h() {
                let input = "you shall spend 4 h on this";
                let actual = duration(input);
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
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
            }

            #[test]
            fn minutes() {
                let input = "42 minutes";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 42, 0));
            }

            #[test]
            fn m() {
                let input = "3 m";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 3, 0));
            }
        }

        mod before_text {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn minute() {
                let input = "1 minute before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
            }

            #[test]
            fn minutes() {
                let input = "11 minutes before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 11, 0));
            }

            #[test]
            fn m() {
                let input = "7 m before other text";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 7, 0));
            }
        }

        mod after_text {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn minute() {
                let input = "you shall spend 1 minute";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
            }

            #[test]
            fn minutes() {
                let input = "you shall spend 3 minutes";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 3, 0));
            }

            #[test]
            fn m() {
                let input = "you shall spend 4 m";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 4, 0));
            }
        }

        mod in_the_middle {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn minute() {
                let input = "you shall spend 1 minute on this";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 1, 0));
            }

            #[test]
            fn minutes() {
                let input = "you shall spend 3 minutes on this";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 3, 0));
            }

            #[test]
            fn m() {
                let input = "you shall spend 4 m on this";
                let actual = duration(input);
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
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
            }

            #[test]
            fn seconds() {
                let input = "42 seconds";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
            }

            #[test]
            fn secs() {
                let input = "12 secs";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
            }

            #[test]
            fn sec() {
                let input = "9 sec";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
            }

            #[test]
            fn s() {
                let input = "55 s";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
            }
        }

        mod before_text {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn second() {
                let input = "1 second more or less";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
            }

            #[test]
            fn seconds() {
                let input = "42 seconds more or less";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
            }

            #[test]
            fn secs() {
                let input = "12 secs more or less";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
            }

            #[test]
            fn sec() {
                let input = "9 sec more or less";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
            }

            #[test]
            fn s() {
                let input = "55 s more or less";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
            }
        }

        mod after_text {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn second() {
                let input = "more or less 1 second";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
            }

            #[test]
            fn seconds() {
                let input = "more or less 42 seconds";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
            }

            #[test]
            fn secs() {
                let input = "more or less 12 secs";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
            }

            #[test]
            fn sec() {
                let input = "more or less 9 sec";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
            }

            #[test]
            fn s() {
                let input = "more or less 55 s";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
            }
        }

        mod in_the_middle {
            use super::*;
            use pretty_assertions::assert_eq;

            #[test]
            fn second() {
                let input = "more or less 1 second but who really knows";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 1));
            }

            #[test]
            fn seconds() {
                let input = "more or less 42 seconds but who really knows";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 42));
            }

            #[test]
            fn secs() {
                let input = "more or less 12 secs but who really knows";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 12));
            }

            #[test]
            fn sec() {
                let input = "more or less 9 sec but who really knows";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 9));
            }

            #[test]
            fn s() {
                let input = "more or less 55 s but who really knows";
                let actual = duration(input);
                assert_eq!(actual.unwrap(), Duration::new(0, 0, 55));
            }
        }
    }
}
