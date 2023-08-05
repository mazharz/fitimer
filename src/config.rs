use crate::fs::Fs;
use crate::{color::Color, env};
use serde::{Deserialize, Serialize};
use tui::style::Color as TColor;

#[derive(Debug)]
pub struct StaticConfig {
    pub config_dir: String,
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub color: ConfigColor,
    pub tick_rate: u64,
    pub durations: ConfigDurations,
    pub date_format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDurations {
    pub work: i64,
    pub rest: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigColor {
    pub gray: String,
    pub white: String,
    pub black: String,
    pub yellow: String,
    pub green: String,
    pub red: String,
}

pub struct ConfigTuiColor {
    pub gray: TColor,
    pub white: TColor,
    pub black: TColor,
    pub yellow: TColor,
    pub green: TColor,
    pub red: TColor,
}

impl Config {
    pub fn init() -> Config {
        let config_str = Fs::read_file_str(String::from("config.json"));
        // TODO: think about this following error message, try it out for different circumstances
        let config_json: Config = serde_json::from_str(&config_str)
            .expect("Either config file is not valid json, or it lacks certain fields.");
        return Config {
            color: config_json.color,
            tick_rate: config_json.tick_rate,
            durations: config_json.durations,
            date_format: config_json.date_format,
        };
    }

    pub fn init_static() -> StaticConfig {
        let home_dir = env::var(String::from("HOME")).unwrap_or("/root".to_string());
        let config_dir = String::from(".config/fitimer");
        let config_dir = format!("{}/{}", home_dir, config_dir);

        return StaticConfig {
            // TODO: rename to log_file or sth better
            file_path: String::from("fitimer.log"),
            config_dir,
        };
    }

    pub fn get_colors(&self) -> ConfigTuiColor {
        let gray = Color::new(self.color.gray.clone());
        let white = Color::new(self.color.white.clone());
        let yellow = Color::new(self.color.yellow.clone());
        let green = Color::new(self.color.green.clone());
        let red = Color::new(self.color.red.clone());
        let black = Color::new(self.color.black.clone());

        return ConfigTuiColor {
            gray,
            white,
            yellow,
            green,
            red,
            black,
        };
    }
}
