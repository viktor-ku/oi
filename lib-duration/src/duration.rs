use crate::RawDuration;


#[derive(Debug, PartialEq, Copy, Clone)]
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

    fn from_seconds(total_seconds: i64) -> Self {
        if total_seconds < 0 {
            return Self::default();
        }

        let mut seconds = total_seconds as u64;

        let hours = seconds / 3600;
        seconds -= hours * 3600;

        let minutes = seconds / 60;
        seconds -= minutes * 60;

        Self::new(hours, minutes, seconds)
    }

    pub(crate) fn merge(raw: &[RawDuration]) -> Self {
        let mut total_secs = 0;

        for raw_duration in raw {
            match raw_duration {
                RawDuration::Hours(hours) => {
                    total_secs += (*hours * 3600.0) as i64;
                }
                RawDuration::Minutes(minutes) => {
                    total_secs += (*minutes * 60.0) as i64;
                }
                RawDuration::Seconds(seconds) => {
                    total_secs += *seconds as i64;
                }
            }
        }

        Self::from_seconds(total_secs)
    }

    pub fn format(&self) -> String {
        format!("{}h {}m {}s", self.hours, self.minutes, self.seconds)
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
