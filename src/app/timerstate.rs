use crate::stats::Stats;
use crate::{config::Config, expiry::Expiry, fs::Fs};
use chrono::Local;
use notify_rust::Notification;
use std::fmt::Display;

pub enum TimerType {
    Work,
    Rest,
}

impl Display for TimerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerType::Work => write!(f, "Work"),
            TimerType::Rest => write!(f, "Rest"),
        }
    }
}

pub struct TimerState {
    pub enabled: bool,
    pub expiry: Expiry,
    pub timer_type: TimerType,
    pub stat_data: Stats,
}

impl TimerState {
    pub fn new(enabled: bool) -> TimerState {
        let expiry = TimerState::get_work_expiry();
        let stat_data = Stats::init();

        TimerState {
            enabled,
            expiry,
            timer_type: TimerType::Work,
            stat_data,
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
        let now = Local::now();
        let config = Config::read();
        let date_format = config.app.date_format.as_str();
        let now = now.format(date_format);
        let formatted_data = format!("{},{},{}", now, self.timer_type, self.expiry.get_elapsed());
        Fs::append_to_file(stat_file_path, formatted_data);
    }

    pub fn toggle_work_rest(&mut self) {
        match self.timer_type {
            TimerType::Work => self.change_to_rest(true),
            TimerType::Rest => self.change_to_work(true),
        }
    }

    pub fn change_to_work(&mut self, show_notification: bool) {
        self.save_stats();
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
        self.save_stats();
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
