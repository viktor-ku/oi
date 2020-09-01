use crate::RawDuration;

#[derive(Debug, PartialEq)]
pub struct Duration {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Duration {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    pub(crate) fn merge(raw: &[RawDuration]) -> Self {
        let mut it = Self::default();

        for raw_duration in raw {
            println!("{:#?}", raw_duration);
            match raw_duration {
                RawDuration::Minutes(minutes) => {
                    it.minutes += *minutes as u32;
                }
                RawDuration::Hours(hours) => {
                    it.hours += *hours as u32;
                }
                RawDuration::Seconds(seconds) => {
                    it.seconds += *seconds as u32;
                }
                _ => {}
            }
        }

        it
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
