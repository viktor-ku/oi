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

pub fn duration(input: &str) -> Result<Option<Duration>, Error> {
    match InputParser::parse(Rule::Input, input) {
        Ok(parsed) => Ok(None),
        Err(e) => Err(Error {}),
    }
}
