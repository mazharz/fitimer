use crate::color::Color;
use crate::env;
use tui::style::Color as TColor;

pub struct Config {
    pub color: ConfigColor,
}

pub struct ConfigColor {
    pub border: TColor,
}

impl Config {
    pub fn read() -> Config {
        let border = env::var("COLOR_GRAY".to_string()).unwrap_or(String::from("#928374"));
        let border = Color::new(border);

        Config {
            color: ConfigColor { border },
        }
    }
}
