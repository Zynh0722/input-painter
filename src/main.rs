use std::io;

use clap::Parser;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

mod cli;

use cli::Cli;

fn main() {
    let color: Color = Cli::parse().into();

    let mut stdin = io::stdin();
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Auto);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();

    io::copy(&mut stdin, &mut stdout).unwrap();

    stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
}
