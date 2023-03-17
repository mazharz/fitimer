use colors_transform::{Color as CTColor, Rgb};
use tui::style::Color as TColor;

pub struct Color;

impl Color {
    pub fn new(hex: String) -> TColor {
        let border = Rgb::from_hex_str(&hex).expect("Couldn't convert hex into rgb.");
        let border = TColor::Rgb(
            border.get_red() as u8,
            border.get_green() as u8,
            border.get_blue() as u8,
        );
        border
    }
}
