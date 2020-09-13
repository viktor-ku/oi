use super::Rule;
use chrono::{DateTime, Datelike, Duration, Local, Timelike, Utc};
use pest::iterators::Pairs;

const DAY_IN_SECS: i64 = 24 * 60 * 60;

#[derive(Debug)]
enum Part {
    Pm,
    Am,
    None,
}

#[derive(Debug)]
pub struct AtTime {
    hours: u32,
    minutes: u32,
}

impl AtTime {
    pub fn new(props: Pairs<Rule>) -> Self {
        let mut hours = 0;
        let mut part = Part::None;
        let mut minutes = 0;

        for prop in props {
            match prop.as_rule() {
                Rule::Pm => part = Part::Pm,
                Rule::Am => part = Part::Am,
                Rule::AtHours => {
                    hours = prop.as_str().parse().unwrap();
                }
                Rule::AtMinutes => {
                    minutes = prop.as_str().parse().unwrap();
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
                    _ => hours,
                }
            }
            Part::Am => {
                hours = match hours {
                    0 | 12 => 24,
                    _ => hours,
                }
            }
            Part::None => {}
        };

        Self { hours, minutes }
    }

    pub fn datetime(&self, based_on: &DateTime<Local>) -> DateTime<Local> {
        let mut dt = based_on.clone();

        if self.hours == 24 {
            dt = dt.checked_add_signed(Duration::days(1)).unwrap();
            dt = dt.with_hour(0).unwrap();
        } else {
            dt = dt.with_hour(self.hours).unwrap();
        }

        dt = dt.with_minute(self.minutes).unwrap();
        dt = dt.with_second(0).unwrap();

        dt
    }

    pub fn diff(&self, another: &DateTime<Local>) -> u64 {
        let one = self.datetime(another);
        let diff = one.timestamp() - another.timestamp();

        if diff >= 0 {
            diff as u64
        } else {
            // not black magic:
            // `diff` is negative at this point, meaning that this operation
            // is effectively `24h - "less than 24h time span (diff)"`
            (diff + DAY_IN_SECS) as u64
        }
    }
}
