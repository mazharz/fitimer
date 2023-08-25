use colors_transform::{Color as CTColor, Rgb};
use tui::style::Color as TColor;

pub struct Color;

impl Color {
    pub fn new(hex: String) -> TColor {
        let rgb = Rgb::from_hex_str(&hex);
        let rgb = match rgb {
            Ok(color) => TColor::Rgb(
                color.get_red() as u8,
                color.get_green() as u8,
                color.get_blue() as u8,
            ),
            Err(_) => TColor::Rgb(255, 255, 255),
        };
        return rgb;
    }
}
