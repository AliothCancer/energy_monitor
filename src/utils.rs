use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn notify_percentage(level: &str, message: &str) {
    let title = format!("Batteria {}!", level);
    std::process::Command::new("notify-send")
        .arg(title)
        .arg(message)
        .output()
        .expect("Something went wrong in getting output from command");
}

pub fn read_file_as_string(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content.trim().to_owned())
}

// Define an enum for errors
#[derive(Debug)]
pub enum MyError {
    NotifierError(String),
    HealthStatsError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotifierError(msg) => write!(f, "Notifier error: {}", msg),
            MyError::HealthStatsError(msg) => write!(f, "Health stats error: {}", msg),
        }
    }
}

impl Error for MyError {}

// Function that may return a NotifierError
fn notifier_function_example() -> Result<(), MyError> {
    // Simulate an error in the notifier function
    Err(MyError::NotifierError(String::from(
        "Failed to send notification",
    )))
}
