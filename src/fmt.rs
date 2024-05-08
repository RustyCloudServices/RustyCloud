use chrono::{DateTime, Local, Timelike};
use colored::{Colorize, CustomColor};

pub enum ConsoleOutputTypes {
    CONSOLE,
    INFO,
    SUCCESS,
    WARN,
    ERROR,
}

pub fn print_console(string: &str, output_type: ConsoleOutputTypes) {
    match output_type {
        ConsoleOutputTypes::CONSOLE => {
            println!("{} CONSOLE: {}", formatted_current_time(), string);
        }
        ConsoleOutputTypes::INFO => {
            println!("{} {} {}", formatted_current_time(), "INFO:".blue(), string);
        }
        ConsoleOutputTypes::SUCCESS => {
            println!(
                "{} {} {}",
                formatted_current_time(),
                "SUCCESS:".green(),
                string
            );
        }
        ConsoleOutputTypes::WARN => {
            println!(
                "{} {} {}",
                formatted_current_time(),
                "WARN:".yellow(),
                string
            );
        }
        ConsoleOutputTypes::ERROR => {
            println!("{} {} {}", formatted_current_time(), "ERROR:".red(), string);
        }
    }
}

pub fn formatted_current_time() -> String {
    let local: DateTime<Local> = Local::now();
    let hour = local.hour();
    let minute = local.minute();
    let second = local.second();
    let formatted_time = format!("[{:02}:{:02}:{:02}]", hour, minute, second);
    formatted_time // [HH:MM:SS]
}

pub fn hex_to_customcolor(hex: &str) -> CustomColor {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    CustomColor { r, g, b }
}