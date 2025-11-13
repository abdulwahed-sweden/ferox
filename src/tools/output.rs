/// Colorized output system for diagnostic tools

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Green,
    Red,
    Yellow,
    Blue,
    Cyan,
    Dimmed,
}

impl Color {
    pub fn as_ansi_code(self) -> &'static str {
        match self {
            Color::Green => "\x1b[32m",
            Color::Red => "\x1b[31m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Cyan => "\x1b[36m",
            Color::Dimmed => "\x1b[90m",
        }
    }
}

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

pub struct ColorizedOutput;

impl ColorizedOutput {
    pub fn success(message: &str) {
        println!("{} {} {message}", Color::Green.as_ansi_code(), BOLD);
        print!("{}", RESET);
    }

    pub fn warning(message: &str) {
        println!("{} {} ⚠️  {message}", Color::Yellow.as_ansi_code(), BOLD);
        print!("{}", RESET);
    }

    pub fn error(message: &str) {
        println!("{} {} ❌ {message}", Color::Red.as_ansi_code(), BOLD);
        print!("{}", RESET);
    }

    pub fn info(message: &str) {
        println!("{} {} ℹ️  {message}", Color::Blue.as_ansi_code(), BOLD);
        print!("{}", RESET);
    }

    pub fn section_header(title: &str) {
        println!();
        println!("{} {} {title}", Color::Cyan.as_ansi_code(), BOLD);
        println!(
            "{}{}",
            Color::Dimmed.as_ansi_code(),
            "─".repeat(title.len())
        );
        print!("{}", RESET);
    }

    pub fn table_row(col1: &str, col2: &str, col3: &str) {
        println!(
            "  {:<40} {:<20} {}",
            col1,
            format!("{}{}{}", Color::Cyan.as_ansi_code(), col2, RESET),
            col3
        );
    }

    pub fn status(success: bool, message: &str) {
        if success {
            Self::success(message);
        } else {
            Self::error(message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_codes() {
        assert_eq!(Color::Green.as_ansi_code(), "\x1b[32m");
        assert_eq!(Color::Red.as_ansi_code(), "\x1b[31m");
        assert_eq!(Color::Blue.as_ansi_code(), "\x1b[34m");
    }
}
