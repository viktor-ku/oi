use super::Rule;
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use pest::iterators::Pairs;

#[derive(Debug)]
enum Part {
    Pm,
    Am,
    None,
}

#[derive(Debug)]
pub struct AtTime {
    /// value from `0` to `23`
    hours: u32,

    /// values from `0` to `59`
    minutes: u32,
}

impl AtTime {
    pub fn new(props: Pairs<Rule>) -> Self {
        let mut hours = 0;
        let mut minutes = 0;
        let mut part = Part::None;

        for prop in props {
            match prop.as_rule() {
                Rule::Pm => part = Part::Pm,
                Rule::Am => part = Part::Am,
                Rule::TimeValue => {
                    hours = prop.as_str().parse().unwrap();
                }
                _ => {}
            }
        }

        match &part {
            Part::Pm => {
                hours = match hours {
                    0 => 12,
                    1 => 13,
                    2 => 14,
                    3 => 15,
                    4 => 16,
                    5 => 17,
                    6 => 18,
                    7 => 19,
                    8 => 20,
                    9 => 21,
                    10 => 22,
                    11 => 23,
                    12 => 12,
                    _ => hours,
                }
            }
            Part::Am => {
                hours = match hours {
                    12 => 24,
                    0 => 24,
                    _ => hours,
                }
            }
            Part::None => {}
        };

        Self { hours, minutes }
    }

    pub fn datetime(&self, based_on: &DateTime<Utc>) -> DateTime<Utc> {
        let mut dt = based_on.clone();

        if self.hours == 24 {
            dt = dt.checked_add_signed(Duration::days(1)).unwrap();
            dt = dt.with_hour(0).unwrap();
        } else {
            dt = dt.with_hour(self.hours).unwrap();
        }

        dt = dt.with_minute(self.minutes).unwrap();

        dt
    }

    pub fn diff(&self, another: &DateTime<Utc>) -> i64 {
        let one = self.datetime(another);
        one.timestamp() - another.timestamp()
    }
}
