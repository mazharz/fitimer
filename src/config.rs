use crate::fs::Fs;
use crate::{color::Color, env};
use serde::{Deserialize, Serialize};
use tui::style::Color as TColor;

#[derive(Debug)]
pub struct StaticConfig {
    pub stat_file: String,
    pub config_file: String,
    pub date_format: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub color: ConfigColor,
    pub tick_rate: u64,
    pub durations: ConfigDurations,
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
        let config_str = Fs::read_file_str(Config::init_static().config_file);
        // TODO: think about this following error message, try it out for different circumstances
        let config_json: Config = serde_json::from_str(&config_str)
            .expect("Either config file is not valid json, or it lacks certain fields.");
        return config_json;
    }

    pub fn init_static() -> StaticConfig {
        let home_dir = env::var(String::from("HOME")).unwrap_or("/root".to_string());
        let config_file = format!(
            "{}/{}",
            home_dir,
            String::from(".config/fitimer/config.json")
        );
        let stat_file = format!(
            "{}/{}",
            home_dir,
            String::from(".cache/fitimer/fitimer.log")
        );

        return StaticConfig {
            stat_file,
            config_file,
            date_format: String::from("%Y-%m-%d %H:%M:%S %z"),
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
