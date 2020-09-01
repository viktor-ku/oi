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

    fn fix_overflow(&mut self) {
        let minutes = self.seconds / 60;
        self.minutes += minutes;
        self.seconds -= minutes * 60;

        let hours = self.minutes / 60;
        self.hours += hours;
        self.minutes -= hours * 60;
    }

    pub(crate) fn merge(raw: &[RawDuration]) -> Self {
        let mut it = Self::default();

        for raw_duration in raw {
            match raw_duration {
                RawDuration::Hours(hours) => {
                    it.hours += *hours as u32;
                    it.minutes += (hours.fract() * 60.0) as u32;
                }
                RawDuration::Minutes(minutes) => {
                    it.minutes += *minutes as u32;
                    it.seconds += (minutes.fract() * 60.0) as u32;
                }
                RawDuration::Seconds(seconds) => {
                    it.seconds += *seconds as u32;
                }
            }
        }

        it.fix_overflow();

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
