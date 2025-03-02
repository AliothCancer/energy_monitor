use super::utils::*;
use std::{io, path::Path};

const BATTERY_FILES_PATH: &str = "/sys/class/power_supply/BAT1";
const BATTERY_FILES: [&str; 21] = [
    "charge_full",        // 0
    "charge_full_design", // 1
    "alarm",
    "device",
    "subsystem",
    "capacity",
    "hwmon2",
    "technology",
    "capacity_level",
    "manufacturer",
    "type",
    "model_name",
    "power",
    "voltage_min_design",
    "charge_now",
    "present",
    "voltage_now",
    "current_now",
    "serial_number",
    "cycle_count",
    "status",
];

pub struct BatteryStatistics {
    pub charge_full: String,
    pub charge_full_design: String,
}

impl BatteryStatistics {
    pub fn new() -> Self {
        let charge_full_path = Path::new(BATTERY_FILES_PATH).join(BATTERY_FILES[0]);
        let charge_full = read_file_as_string(&charge_full_path).unwrap();

        let charge_full_design_path = Path::new(BATTERY_FILES_PATH).join(BATTERY_FILES[1]);
        let charge_full_design = read_file_as_string(&charge_full_design_path).unwrap();

        BatteryStatistics {
            charge_full,
            charge_full_design,
        }
    }
}

impl Default for BatteryStatistics {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_battery_state() -> Result<String, io::Error> {
    let battery_path = Path::new(BATTERY_FILES_PATH);

    let status = battery_path.join("status");

    read_file_as_string(&status)
}

pub fn get_battery_percentage() -> io::Result<f32> {
    let battery_path = Path::new(BATTERY_FILES_PATH);

    let charge_full = battery_path.join("charge_full");
    let charge_now = battery_path.join("charge_now");

    let charge_now_content = read_file_as_string(&charge_now)?;
    let charge_full_content = read_file_as_string(&charge_full)?;

    let battery_level = (charge_now_content.parse::<u32>().unwrap() as f32
        / charge_full_content.parse::<u32>().unwrap() as f32)
        * 100.0;

    //println!("Battery level: {}%", battery_level);

    Ok(battery_level)
}

#[derive(Debug, PartialEq, Eq)]
pub enum BatteryState {
    Discharging,
    Charging,
    Full
}
impl BatteryState {
    pub fn match_string(str_state: &str) -> Self {
        match str_state {
            "Discharging" => Self::Discharging,
            "Charging" => Self::Charging,
            "Full" => Self::Full,
            _ => panic!("Invalid str_state"),
        }
    }
}
