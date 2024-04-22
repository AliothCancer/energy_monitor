use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    battery_notifier: bool,
    health_stats: bool,
    write_every: u64,
}

#[derive(Clone, Copy, Debug)]
enum ConfigOption {
    BatteryNotifier(bool),
    HealthStats(bool),
    WriteEvery(u64),
}

impl Config {
    fn validate_field(field: &str){
        match field {
            "battery_notifier" 
            | "health_stats"
            | "write_every" => (),
            _ => {
                log::error!("Provided field:'{field}' is not a valid config field");
                panic!()
            }
        }
    }
    pub fn get(path: &str) -> Self {
        let config = read_file_as_string(Path::new(path)).unwrap();
        println!("config read");

        let config = config
            .split('\n')
            .take(3)
            .map(|line| {
                let mut splitted_line = line.split('=');

                
                let option_name = splitted_line.next().unwrap().trim();
                
                Config::validate_field(option_name);

                let option_value = match splitted_line.next(){
                    Some(val) => val.trim(),
                    None => {
                        log::error!("Missing field for '{option_name}'");
                        panic!()
                    }
                };

                match option_name{
                    "battery_notifier" => match option_value.parse::<bool>() {
                        Ok(val) => ConfigOption::BatteryNotifier(val),
                        Err(err) => panic!("Config file error: '{option_value}' is not parsable to bool type:{err}")
                    },
                    "health_stats" => match option_value.parse::<bool>() {
                        Ok(val) => ConfigOption::HealthStats(val),
                        Err(err) => panic!("Config file error: '{option_value}' is not parsable to bool type:{err}")
                    },
                    "write_every" => match option_value.parse::<u64>() {
                        Ok(val) => ConfigOption::WriteEvery(val),
                        Err(err) => panic!("Config file error: '{option_value}' is not parsable to u64 type:{err}")
                    }
                    _ => panic!("{option_name} is not a valid config option")
                }})
            .collect::<Vec<_>>();

        if let (
            ConfigOption::BatteryNotifier(battery_notifier),
            ConfigOption::HealthStats(health_stats),
            ConfigOption::WriteEvery(write_every),
        ) = (config[0], config[1], config[2])
        {
            Config {
                battery_notifier,
                health_stats,
                write_every,
            }
        } else {
            panic!("Impossible to parse config")
        }
    }

    pub fn battery_notifier(&self) -> bool {
        self.battery_notifier
    }

    pub fn health_stats(&self) -> bool {
        self.health_stats
    }

    pub fn write_every(&self) -> u64 {
        self.write_every
    }
}

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
/*
fn notifier_function_example() -> Result<(), MyError> {
    // Simulate an error in the notifier function
    Err(MyError::NotifierError(String::from(
        "Failed to send notification",
    )))
}
*/
