pub struct Formatter;

impl Formatter {
    pub fn add_trailing_zero(number: i64) -> String {
        let formatted = if number > 9 {
            format!("{}", number)
        } else {
            format!("0{}", number)
        };

        return formatted;
    }
}
