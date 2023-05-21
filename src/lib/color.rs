#[allow(dead_code)]
pub enum Pisking {
    Slow,
    Fast,
}

#[allow(dead_code)]
pub enum Format {
    //Defines the formats
    Default,
    Bold,
    Darker,
    Italic,
    Invert,
    Hidden,
    Sublinhed,
    Pisking(Pisking),
    Risk,
}

#[allow(dead_code)]
pub enum Color {
    //Defines all the colors
    BrightBlue,
    Blue,
    BrightRed,
    Red,
    BrightWhite,
    White,
    Purple,
    BrightPurple,
    Black,
    Green,
    BrightGreen,
}

pub fn format<T: std::fmt::Display>(text: T, color: Color, format: Format) -> String {
        let ansi_format = match format {
        //Chooses the correct format
        Format::Default => 0,
        Format::Bold => 1,
        Format::Darker => 2,
        Format::Italic => 3,
        Format::Sublinhed => 4,
        Format::Pisking(Pisking::Slow) => 5,
        Format::Pisking(Pisking::Fast) => 6,
        Format::Invert => 7,
        Format::Hidden => 8,
        Format::Risk => 9,
    };
    let ansi_color = match color {
        //Chooses the correct color
        Color::BrightBlue => 94,
        Color::Blue => 34,
        Color::BrightRed => 91,
        Color::Red => 31,
        Color::BrightWhite => 97,
        Color::White => 97,
        Color::Purple => 35,
        Color::BrightPurple => 95,
        Color::Black => 30,
        Color::Green => 32,
        Color::BrightGreen => 92,
    };
    format!("\x1b[{};{}m{}\x1b[0m", ansi_format, ansi_color, text) //Returns the formated text
}
