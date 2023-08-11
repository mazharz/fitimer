use crate::fs::Fs;
use crate::{color::Color, env};
use notify_rust::Notification;
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
        return match Fs::read_file_str(Config::init_static().config_file) {
            Ok(config_str) => match serde_json::from_str(&config_str) {
                Ok(config) => config,
                Err(_) => {
                    Notification::new()
                        .summary("Fitimer")
                        .body("There was a problem loading your configuration file, falling back on default.")
                        .show()
                        .unwrap();
                    Config::get_default_config()
                }
            },
            Err(_) => Config::get_default_config(),
        };
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
            date_format: String::from("%Y-%m-%d"),
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

    fn get_default_config() -> Config {
        return Config {
            color: ConfigColor {
                black: String::from("#282828"),
                white: String::from("#fbf1c7"),
                gray: String::from("#928374"),
                yellow: String::from("#d79921"),
                green: String::from("#98971a"),
                red: String::from("#cc241d"),
            },
            tick_rate: 1000,
            durations: ConfigDurations { work: 25, rest: 5 },
        };
    }
}
