use crate::{fs::Fs, STATIC_CONFIG};
use chrono::{Local, NaiveDate};
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug)]
pub struct Stat {
    pub work_data: Vec<(String, f64, NaiveDate)>,
    pub rest_data: Vec<(String, f64, NaiveDate)>,
    pub min: f64,
    pub max: f64,
}

impl Stat {
    pub fn init() -> Self {
        let mut data: HashMap<String, (i32, i32, NaiveDate)> = HashMap::new();
        let lines = Self::get_raw_lines();
        for line in lines {
            let (date, timer_type, duration_secs) = Self::get_parsed_line_elements(&line);
            let should_include = Self::get_should_include(&date);
            if !should_include {
                continue;
            }
            let day = date.format("%m/%d").to_string();
            if data.contains_key(&day) {
                let fallback = &mut (0, 0, date);
                let current = data.get_mut(&day).unwrap_or(fallback);
                if timer_type == "w" {
                    current.0 += duration_secs;
                } else {
                    current.1 += duration_secs;
                }
            } else {
                if timer_type == "w" {
                    data.insert(day, (duration_secs, 0, date));
                } else {
                    data.insert(day, (0, duration_secs, date));
                }
            }
        }
        let work_data = Self::get_formatted_data(&data, |tuple| tuple.0);
        let rest_data = Self::get_formatted_data(&data, |tuple| tuple.1);
        let mut max = 0.0;
        for wd in work_data.iter() {
            if wd.1 > max {
                max = wd.1;
            }
        }
        let min = 0.0;

        return Stat {
            work_data,
            rest_data,
            min,
            max,
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

    fn get_raw_lines() -> Vec<String> {
        let file_path = STATIC_CONFIG.stat_file.clone();
        let lines = Fs::read_file(file_path);
        return lines.unwrap_or_default();
    }

    fn get_parsed_line_elements<'a>(line: &'a String) -> (NaiveDate, &'a str, i32) {
        let parts: Vec<&str> = line.split(",").collect();
        let (date, timer_type, duration_secs) = (parts[0], parts[1], parts[2]);
        let duration_secs = duration_secs.parse::<i32>().unwrap_or_default();
        let date_format = STATIC_CONFIG.date_format.as_str();
        let date = NaiveDate::parse_from_str(date, date_format).unwrap_or_default();

        return (date, timer_type, duration_secs);
    }

    fn get_should_include(date: &NaiveDate) -> bool {
        let stat_year = date.format("%Y").to_string();
        let now = Local::now();
        let current_year = now.format("%Y").to_string();
        return stat_year == current_year;
    }

    fn get_formatted_data(
        data: &HashMap<String, (i32, i32, NaiveDate)>,
        get_relevant_field: impl Fn((i32, i32, NaiveDate)) -> i32,
    ) -> Vec<(String, f64, NaiveDate)> {
        let mut work_data = data
            .iter()
            .map(|d| {
                let key = d.0.clone() as String;
                let value = d.1.clone();
                return (key, get_relevant_field(value) as f64, value.2);
            })
            .collect::<Vec<(String, f64, NaiveDate)>>();
        work_data.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal));
        return work_data;
    }
}
