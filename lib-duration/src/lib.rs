#[macro_use]
extern crate pest_derive;

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

pub fn duration(input: &str) -> Result<Duration, Error> {
    let raw_durations: Vec<RawDuration> = match InputParser::parse(Rule::Input, input) {
        Ok(parsed) => {
            let mut v = Vec::with_capacity(32);

            for expr in parsed {
                match expr.as_rule() {
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
        Err(e) => return Err(Error {}),
    };

    Ok(Duration::merge(&raw_durations))
}
