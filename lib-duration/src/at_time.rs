use super::Rule;
use chrono::{DateTime, Timelike, Utc};
use pest::iterators::Pairs;

#[derive(Debug)]
pub enum AtTimePart {
    Pm,
    Am,
}

#[derive(Debug)]
pub struct AtTime<'d> {
    /// value from `0` to `23`
    hours: u32,

    part: Option<AtTimePart>,

    now: &'d DateTime<Utc>,
}

impl<'d> AtTime<'d> {
    pub fn new(now: &'d DateTime<Utc>, props: Pairs<Rule>) -> Self {
        let mut hours = 0;
        let mut part = None;

        println!("at time props {:#?}", props);

        for prop in props {
            match prop.as_rule() {
                Rule::Pm => part = Some(AtTimePart::Pm),
                Rule::TimeValue => {
                    hours = prop.as_str().parse().unwrap();
                }
                _ => {}
            }
        }

        Self { hours, part, now }
    }

    fn normalize_hours(&self) -> u32 {
        match &self.part {
            Some(part) => match part {
                AtTimePart::Pm => match self.hours {
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
                    _ => panic!("illegal hours value"),
                },
                AtTimePart::Am => todo!(),
            },
            None => 0,
        }
    }
}

impl<'d> Into<DateTime<Utc>> for AtTime<'d> {
    fn into(self) -> DateTime<Utc> {
        let mut now = self.now.clone();
        println!("self {:#?}", self);

        let hours = self.normalize_hours();
        println!("hours {}", hours);
        now = now.with_hour(hours).unwrap();

        now
    }
}
