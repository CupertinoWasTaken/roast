use owo_colors::{OwoColorize, Style};
use std::fmt::Write;

pub fn parse_colors(input: &str) -> String {
    let mut output = String::new();
    let mut current_style = Style::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '§' {
            if let Some(code) = chars.next() {
                match code {
                    '0' => current_style = Style::new().black(),
                    '1' => current_style = current_style.blue(),
                    '2' => current_style = current_style.green(),
                    '3' => current_style = current_style.cyan(),
                    '4' => current_style = current_style.red(),
                    '5' => current_style = current_style.purple(),
                    '6' => current_style = current_style.yellow(),
                    '7' => current_style = current_style.white(),
                    '8' => current_style = current_style.bright_black(),
                    '9' => current_style = current_style.bright_blue(),
                    'a' => current_style = current_style.bright_green(),
                    'b' => current_style = current_style.bright_cyan(),
                    'c' => current_style = current_style.bright_red(),
                    'd' => current_style = current_style.bright_purple(),
                    'e' => current_style = current_style.bright_yellow(),
                    'f' => current_style = current_style.bright_white(),
                    'l' => current_style = current_style.bold(),
                    'o' => current_style = current_style.italic(),
                    'u' => current_style = current_style.underline(),
                    'r' => current_style = Style::new(),
                    _ => {}
                }
            }
        } else {
            let _ = write!(output, "{}", current_style.style(c));
        }
    }

    output
}
