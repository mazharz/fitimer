use std::time::Duration as STDDuration;

use crate::formatter::Formatter;

pub struct Duration {
    value: STDDuration,
}

impl Duration {
    pub fn new(minutes: u64) -> Duration {
        Duration {
            value: STDDuration::new(minutes * 60, 0),
        }
    }

    pub fn format(&self) -> String {
        let duration_seconds = self.value.as_secs();
        let mins = (duration_seconds / 60) % 60;
        let mins = Formatter::add_trailing_zero(mins);
        let secs = duration_seconds % 60;
        let secs = Formatter::add_trailing_zero(secs);

        format!("{}:{}", mins, secs)
    }
}
