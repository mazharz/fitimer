use chrono::{DateTime, Duration, Local};
use std::time::Duration as TDuration;

use crate::formatter::Formatter;

pub struct Expiry {
    pub duration: TDuration,
    beg: DateTime<Local>,
    end: DateTime<Local>,
}

impl Expiry {
    pub fn new(minutes: i64) -> Expiry {
        let beg = Local::now();
        let end = Local::now() + Duration::minutes(minutes);
        Expiry {
            beg,
            end,
            duration: TDuration::from_secs((minutes * 60) as u64),
        }
    }

    pub fn format(&self) -> String {
        let diff = self.get_remaining();

        let mins = (diff / 60) % 60;
        let mins = Formatter::add_trailing_zero(mins);
        let secs = diff % 60;
        let secs = Formatter::add_trailing_zero(secs);

        format!("{}:{}", mins, secs)
    }

    pub fn get_remaining(&self) -> i64 {
        let diff = self.end - Local::now();
        let diff = diff.num_seconds();

        if diff <= 0 {
            return 0;
        }

        return diff;
    }

    pub fn get_elapsed(&self) -> i64 {
        let diff = Local::now() - self.beg;
        let diff = diff.num_seconds();

        if diff <= 0 {
            return 0;
        }

        return diff;
    }

    pub fn get_is_expired(&self) -> bool {
        let value = self.get_remaining();
        if value <= 0 {
            return true;
        }
        return false;
    }
}
