use chrono::{DateTime, TimeZone, Utc};
use lib_duration::{duration, Duration};

mod with_part {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn at_1pm() {
        let input = "at 1pm";
        let now = Utc.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(0, 30, 0)));
    }

    #[test]
    fn at_2pm() {
        let input = "at 2pm";
        let now = Utc.ymd(2020, 9, 1).and_hms(12, 30, 0);
        assert_eq!(duration(input, &now), Ok(Duration::new(1, 30, 0)));
    }
}
