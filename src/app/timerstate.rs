use chrono::{DateTime, Local};
use notify_rust::Notification;

use crate::{config::Config, expiry::Expiry, fs::Fs};

pub enum TimerType {
    Work,
    Rest,
}

pub struct TimerState {
    pub enabled: bool,
    pub expiry: Expiry,
    pub timer_type: TimerType,
}

impl TimerState {
    pub fn new(enabled: bool) -> TimerState {
        let expiry = TimerState::get_work_expiry();

        TimerState {
            enabled,
            expiry,
            timer_type: TimerType::Work,
        }
    }

    fn reset(&mut self) {
        self.expiry = TimerState::get_work_expiry();
        self.timer_type = TimerType::Work;
    }

    pub fn toggle_enabled(&mut self) {
        self.reset();
        self.enabled = !self.enabled;
    }

    fn save_stats(&self) {
        let stat_file_path = Config::read().app.file_path;
        // TODO: then store this & type & duration in a file
        Fs::append_to_file(stat_file_path, "sdfj".to_string());
        // let now = Local::now();
        // let config = Config::read();
        // let date_format = config.app.date_format.as_str();
        // let now = now.format(date_format);
        // println!("now: {}", now);

        // later read it like this:
        // let d = DateTime::parse_from_str("2023-03-21 12:27:36 +0330", date_format);
    }

    pub fn toggle_work_rest(&mut self) {
        self.save_stats();
        match self.timer_type {
            TimerType::Work => self.change_to_rest(true),
            TimerType::Rest => self.change_to_work(true),
        }
    }

    pub fn change_to_work(&mut self, show_notification: bool) {
        self.timer_type = TimerType::Work;
        self.expiry = TimerState::get_work_expiry();
        if show_notification {
            Notification::new()
                .summary("Fitimer")
                .body("It's time to get back to work!")
                .show()
                .unwrap();
        }
    }

    pub fn change_to_rest(&mut self, show_notification: bool) {
        self.timer_type = TimerType::Rest;
        self.expiry = TimerState::get_rest_expiry();
        if show_notification {
            Notification::new()
                .summary("Fitimer")
                .body("It's time to take a break!")
                .show()
                .unwrap();
        }
    }

    pub fn check(&mut self) {
        let stat_file_path = Config::read().app.file_path;
        // TODO: then store this & type & duration in a file
        Fs::append_to_file(stat_file_path, "sdfj".to_string());
        if !self.enabled {
            return ();
        };
        let is_expired = self.expiry.get_is_expired();
        if is_expired == true {
            self.toggle_work_rest();
        }
    }

    fn get_work_expiry() -> Expiry {
        let work = Config::read().app.durations.work;
        let work = Expiry::new(work);

        return work;
    }

    fn get_rest_expiry() -> Expiry {
        let rest = Config::read().app.durations.rest;
        let rest = Expiry::new(rest);

        return rest;
    }
}
