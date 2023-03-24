use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, StandardStream};

pub fn print_green(text: &str) {
    // Create a color specification for green
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Green));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_black(text: &str) {
    // Create a color specification for black
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Black));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_blue(text: &str) {
    // Create a color specification for blue
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Blue));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_red(text: &str) {
    // Create a color specification for red
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Red));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_cyan(text: &str) {
    // Create a color specification for cyan
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Cyan));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_magenta(text: &str) {
    // Create a color specification for magenta
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Magenta));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_yellow(text: &str) {
    // Create a color specification for yellow
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Yellow));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}

pub fn print_white(text: &str) {
    // Create a color specification for white
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::White));

    // Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    // Reset the color specification to the default
    stdout.reset().unwrap();
}


