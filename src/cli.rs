use clap::Parser;
use termcolor::Color;

#[derive(Parser, Debug)]
pub enum Cli {
    /// Paint output black
    Black,
    /// Paint output blue
    Blue,
    /// Paint output green
    Green,
    /// Paint output red
    Red,
    /// Paint output cyan
    Cyan,
    /// Paint output magenta
    Magenta,
    /// Paint output yellow
    Yellow,
    /// Paint output white
    White,
    /// Paint output an ansi256 color
    Ansi { color: u8 },
    /// Paint output true color rgb
    Rgb { red: u8, green: u8, blue: u8 },
}

impl From<Cli> for Color {
    fn from(value: Cli) -> Self {
        match value {
            Cli::Black => Color::Black,
            Cli::Blue => Color::Blue,
            Cli::Green => Color::Green,
            Cli::Red => Color::Red,
            Cli::Cyan => Color::Cyan,
            Cli::Magenta => Color::Magenta,
            Cli::Yellow => Color::Yellow,
            Cli::White => Color::White,
            Cli::Ansi { color } => Color::Ansi256(color),
            Cli::Rgb { red, green, blue } => Color::Rgb(red, blue, green),
        }
    }
}
