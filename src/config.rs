use crate::color::Color;
use crate::env;
use tui::style::Color as TColor;

pub struct Config {
    pub color: ConfigColor,
    pub app: ConfigApp,
}

pub struct ConfigApp {
    pub tick_rate: u64,
    pub durations: ConfigDurations,
    pub config_dir: String,
    pub file_path: String,
    pub date_format: String,
}

pub struct ConfigDurations {
    pub work: i64,
    pub rest: i64,
}

pub struct ConfigColor {
    pub gray: TColor,
    pub white: TColor,
    pub black: TColor,
    pub yellow: TColor,
    pub green: TColor,
    pub red: TColor,
}

impl Config {
    pub fn read() -> Config {
        let gray = env::var("FITIMER_COLOR_GRAY".to_string()).unwrap_or(String::from("#928374"));
        let gray = Color::new(gray);

        let white = env::var("FITIMER_COLOR_LIGHT".to_string()).unwrap_or(String::from("#fbf1c7"));
        let white = Color::new(white);

        let yellow =
            env::var("FITIMER_COLOR_YELLOW".to_string()).unwrap_or(String::from("#d79921"));
        let yellow = Color::new(yellow);

        let green = env::var("FITIMER_COLOR_GREEN".to_string()).unwrap_or(String::from("#98971a"));
        let green = Color::new(green);

        let red = env::var("FITIMER_COLOR_RED".to_string()).unwrap_or(String::from("#cc241d"));
        let red = Color::new(red);

        let black = env::var("FITIMER_COLOR_DARK".to_string()).unwrap_or(String::from("#282828"));
        let black = Color::new(black);

        let tick_rate = env::var(String::from("FITIMER_TICK_RATE")).unwrap_or(String::from("1000"));
        let tick_rate = tick_rate.parse::<u64>().unwrap_or(1000);

        let duration_work = env::var(String::from("FITIMER_DURATION_WORK"));
        let duration_work = duration_work.unwrap_or(String::from("25"));
        let duration_work = duration_work
            .parse::<i64>()
            .expect("Couldn't convert duration string into number");

        let duration_rest = env::var(String::from("FITIMER_DURATION_REST"));
        let duration_rest = duration_rest.unwrap_or(String::from("5"));
        let duration_rest = duration_rest
            .parse::<i64>()
            .expect("Couldn't convert duration string into number");

        let home_dir = env::var(String::from("HOME")).unwrap_or("/root".to_string());
        let config_dir = env::var(String::from("FITIMER_CONFIG_DIR"));
        let config_dir = config_dir.unwrap_or(String::from(".config/fitimer"));
        let config_dir = format!("{}/{}", home_dir, config_dir);

        let file_path = env::var(String::from("FITIMER_LOG_FILE"));
        let file_path = file_path.unwrap_or(String::from("fitimer.log"));

        let date_format = env::var(String::from("FITIMER_DATE_FORMAT"));
        let date_format = date_format.unwrap_or(String::from("%Y-%m-%d %H:%M:%S %z"));

        Config {
            color: ConfigColor {
                gray,
                white,
                black,
                yellow,
                green,
                red,
            },
            app: ConfigApp {
                tick_rate,
                durations: ConfigDurations {
                    work: duration_work,
                    rest: duration_rest,
                },
                config_dir,
                file_path,
                date_format,
            },
        }
    }
}
