use crate::color::Color;
use crate::env;
use tui::style::Color as TColor;

pub struct Config {
    pub color: ConfigColor,
}

pub struct ConfigColor {
    pub gray: TColor,
    pub white: TColor,
}

impl Config {
    pub fn read() -> Config {
        let gray = env::var("COLOR_GRAY".to_string()).unwrap_or(String::from("#928374"));
        let gray = Color::new(gray);

        let white = env::var("COLOR_LIGHT".to_string()).unwrap_or(String::from("#fbf1c7"));
        let white = Color::new(white);

        Config {
            color: ConfigColor { gray, white },
        }
    }
}
