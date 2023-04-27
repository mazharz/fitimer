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
}

pub struct ConfigDurations {
    pub work: i64,
    pub rest: i64,
    pub extend: i64,
}

pub struct ConfigColor {
    pub gray: TColor,
    pub white: TColor,
    pub black: TColor,
}

impl Config {
    pub fn read() -> Config {
        let gray = env::var("COLOR_GRAY".to_string()).unwrap_or(String::from("#928374"));
        let gray = Color::new(gray);

        let white = env::var("COLOR_LIGHT".to_string()).unwrap_or(String::from("#fbf1c7"));
        let white = Color::new(white);

        let black = env::var("COLOR_DARK".to_string()).unwrap_or(String::from("#282828"));
        let black = Color::new(black);

        let tick_rate = env::var(String::from("TICK_RATE")).unwrap_or(String::from("250"));
        let tick_rate = tick_rate.parse::<u64>().unwrap_or(250);

        let duration_work = env::var(String::from("DURATION_WORK"));
        let duration_work = duration_work.unwrap_or(String::from("25"));
        let duration_work = duration_work
            .parse::<i64>()
            .expect("Couldn't convert duration string into number");

        let duration_rest = env::var(String::from("DURATION_REST"));
        let duration_rest = duration_rest.unwrap_or(String::from("5"));
        let duration_rest = duration_rest
            .parse::<i64>()
            .expect("Couldn't convert duration string into number");

        let duration_extend = env::var(String::from("DURATION_EXTEND"));
        let duration_extend = duration_extend.unwrap_or(String::from("5"));
        let duration_extend = duration_extend
            .parse::<i64>()
            .expect("Couldn't convert duration string into number");

        Config {
            color: ConfigColor { gray, white, black },
            app: ConfigApp {
                tick_rate,
                durations: ConfigDurations {
                    work: duration_work,
                    rest: duration_rest,
                    extend: duration_extend,
                },
            },
        }
    }
}
