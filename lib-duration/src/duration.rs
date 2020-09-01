use crate::RawDuration;

#[derive(Debug, PartialEq)]
pub struct Duration {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl Duration {
    pub fn new(hours: u64, minutes: u64, seconds: u64) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    pub fn secs(&self) -> u64 {
        self.seconds + (self.minutes * 60) + (self.hours * 3600)
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
                    it.hours += *hours as u64;
                    it.minutes += (hours.fract() * 60.0) as u64;
                }
                RawDuration::Minutes(minutes) => {
                    it.minutes += *minutes as u64;
                    it.seconds += (minutes.fract() * 60.0) as u64;
                }
                RawDuration::Seconds(seconds) => {
                    it.seconds += *seconds as u64;
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
