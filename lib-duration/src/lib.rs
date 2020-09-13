#[macro_use]
extern crate pest_derive;

use chrono::{DateTime, Utc, Local};
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

mod duration;
pub use duration::Duration;

pub mod error;
use error::Error;

mod raw_duration;
use raw_duration::RawDuration;

mod at_time;
use at_time::AtTime;

pub fn duration(input: &str, now: &DateTime<Local>) -> Result<Duration, Error> {
    let raw_durations: Vec<RawDuration> = match InputParser::parse(Rule::Input, input) {
        Ok(parsed) => {
            let mut v = Vec::with_capacity(32);

            for expr in parsed {
                match expr.as_rule() {
                    Rule::AtTime => {
                        let at = AtTime::new(expr.into_inner());
                        let diff = at.diff(now);
                        if diff > 0 {
                            v.push(RawDuration::Seconds(diff as f64));
                        }
                    }
                    Rule::DurationExpr => {
                        let mut needle: f64 = 0.0;

                        for prop in expr.into_inner() {
                            match prop.as_rule() {
                                Rule::Duration => {
                                    needle = prop.as_str().parse().unwrap();
                                }
                                Rule::Minutes => {
                                    v.push(RawDuration::Minutes(needle));
                                }
                                Rule::Hours => v.push(RawDuration::Hours(needle)),
                                Rule::Seconds => v.push(RawDuration::Seconds(needle)),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }

            v
        }
        Err(e) => {
            println!("err {}", e);
            return Err(Error {});
        }
    };

    Ok(Duration::merge(&raw_durations))
}
