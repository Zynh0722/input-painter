use std::io;

use clap::Parser;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

mod cli;

use cli::Cli;

fn main() {
    let color: Color = Cli::parse().into();

    let stdin = io::stdin();
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Auto);

    for line in stdin.lines() {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();

        if let Ok(line) = line {
            println!("{line}");
        }
    }

    stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
}
