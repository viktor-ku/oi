use chrono::{DateTime, TimeZone, Utc};
use lib_duration::{duration, Duration};

mod with_part {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pm_from_1_to_11() {
        let pm_values: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        for pm in pm_values {
            let input = &format!("at {}pm", pm);
            let now = Utc.ymd(2020, 9, 1).and_hms(12, 0, 0);
            let actual = duration(input, &now).unwrap();
            let expected = Duration::new(*pm, 0, 0);
            if actual != expected {
                println!("{} failed", input);
            }
            assert_eq!(actual, Duration::new(*pm, 0, 0));
        }
    }
}
