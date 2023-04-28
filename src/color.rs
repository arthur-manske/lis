pub enum Color { //Defines all the colors
    BrightBlue,
    Blue,
    BrightRed,
    Red,
    BrightWhite,
    White, 
    Purple,
    BrightPurple,
    Black,
    Pink,
    BrightPink,
    Green,
    BrightGreen,
}

pub fn colorize(text: &str, color: Color) -> String {
    let ansi_color = match color { //Choose the correct color
        Color::BrightBlue => "\x1b[0;94m",
        Color::Blue => "\x1b[0;34m",
        Color::BrightRed => "\x1b[1;91m",
        Color::Red => "\x1b[0;31m",
        Color::BrightWhite => "\x1b[0;97m",
        Color::White => "\x1b[0;37m",
        Color::Purple => "\x1b[0;35m",
        Color::BrightPurple => "\x1b[1;95m",
        Color::Black => "\x1b[0;30m",
        Color::Pink => "\x1b[0;95m",
        Color::BrightPink => "\x1b[1;95m",
        Color::Green => "\x1b[0;32m",
        Color::BrightGreen => "\x1b[0;92m",
    };
    format!("{}{}\x1b[0m", ansi_color, text) //Returns the colorized text
}
