use crate::{config::Config, expiry::Expiry, fs::Fs};
use chrono::{DateTime, Local};
use notify_rust::Notification;
use std::collections::HashMap;
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

#[derive(Clone, Debug)]
pub struct Stats {
    pub work_data: Vec<(String, f64, f64)>,
    pub rest_data: Vec<(String, f64, f64)>,
    pub min: f64,
    pub max: f64,
    // pub first_day: String,
    // pub last_day: String,
}

impl Stats {
    fn init() -> Self {
        let file_path = Config::read().app.file_path;
        let contents = Fs::read_file(file_path);
        let lines = contents.lines();
        let mut data: HashMap<String, (i32, i32, i64)> = HashMap::new();
        for line in lines {
            let parts: Vec<&str> = line.split(",").collect();
            let (date, timer_type, duration_secs) = (parts[0], parts[1], parts[2]);
            let duration_secs = duration_secs
                .parse::<i32>()
                .expect("Couldn't parse duration in stats file");
            let config = Config::read();
            let date_format = config.app.date_format.as_str();
            let date = DateTime::parse_from_str(date, date_format)
                .expect("Couldn't parse date in stats file");
            let y = date.format("%Y").to_string();
            let now = Local::now();
            let now_y = now.format("%Y").to_string();
            if y != now_y {
                continue;
            }
            let timestamp = date.timestamp();
            let day = date.format("%m/%d").to_string();
            if data.contains_key(&day) {
                let fallback = &mut (0, 0, timestamp);
                let current = data.get_mut(&day).unwrap_or(fallback);
                if timer_type == "Work" {
                    current.0 += duration_secs;
                } else {
                    current.1 += duration_secs;
                }
            } else {
                data.insert(day, (0, 0, timestamp));
            }
        }
        let mut work_data = data
            .iter()
            .map(|d| {
                let key = d.0.clone() as String;
                let value = d.1.clone();
                return (key, value.0 as f64, value.2 as f64);
            })
            .collect::<Vec<(String, f64, f64)>>();
        work_data.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        let mut rest_data = data
            .iter()
            .map(|d| {
                let key = d.0.clone() as String;
                let value = d.1.clone();
                return (key, value.1 as f64, value.2 as f64);
            })
            .collect::<Vec<(String, f64, f64)>>();
        rest_data.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        let first_day = &work_data[0].0;
        let last_day = &work_data[&work_data.len() - 1].0;
        let mut max = 0.0;
        for wd in work_data.iter() {
            if wd.1 > max {
                max = wd.1;
            }
        }
        let min = 0.0;

        return Stats {
            work_data,
            rest_data,
            min,
            max,
            // first_day: first_day.to_string(),
            // last_day: last_day.to_string(),
        };
    }

    pub fn get_work_data(&self) -> Vec<(f64, f64)> {
        let wd = &self.work_data;
        return wd
            .iter()
            .enumerate()
            .map(|(index, item)| (index as f64, item.1))
            .collect();
    }

    pub fn get_rest_data(&self) -> Vec<(f64, f64)> {
        let rd = &self.rest_data;
        return rd
            .iter()
            .enumerate()
            .map(|(index, item)| (index as f64, item.1))
            .collect();
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
        // later read date like this:
        // let d = DateTime::parse_from_str("2023-03-21 12:27:36 +0330", date_format);
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
