extern crate pest;
#[macro_use]
extern crate pest_derive;

use notify_rust::{error::Error, Notification, NotificationHandle};
use pest::Parser;
use std::thread;
use std::time;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

fn notify(message: &str) -> Result<NotificationHandle, Error> {
    Ok(Notification::new()
        .summary("timer")
        .body(message)
        .timeout(3_000)
        .show()?)
}

fn main() {
    let input: String = std::env::args()
        .skip(1)
        .into_iter()
        .collect::<Vec<String>>()
        .join(" ");
    let input = input.trim();

    let parsed = InputParser::parse(Rule::Input, input);
    if parsed.is_err() {
        let e = parsed.err().unwrap();
        println!("{}", &e);
        println!("{:#?}", &e);
        return;
    }
    println!("{:#?}", parsed);
}
