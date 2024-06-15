pub mod battery_health;
pub mod secret_info;
pub mod utils;

use battery_health::*;
use log::LevelFilter;
use secret_info::{CONFIG_FILE_PATH, DATA_FILE_PATH}; // config file and the csv file in which to store data
use std::io::Write;
use std::{error::Error, fs::OpenOptions, path::Path, thread, time::Duration};
use utils::notify_percentage;
use utils::Config;



const CHARGE_UPPER_LIMIT: f32 = 80.0;
const DISCHARGE_LOWER_LIMIT: f32 = 20.0;
//const WRITE_BATTERY_HEALTH_STATS_EVERY: u64 = 5; // minutes
const BATTERY_CHECK_TIME: u64 = 20;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info) // Set log level
        .init();

    let mut has_been_notified_80 = false;
    let mut has_been_notified_20 = false;

    let config = Config::get(CONFIG_FILE_PATH);

    println!("{config:?}");
    let battery_notifier = config.battery_notifier();
    let write_health_stats = config.health_stats();
    let write_every = config.write_every();

    //println!("{battery_notifier}{h_stats}");
    let handle1 = thread::spawn(move || {
        if battery_notifier {
            thread::sleep(Duration::from_secs(1));
            notifier(&mut has_been_notified_80, &mut has_been_notified_20);
            //println!("notify")
        }
    });

    let handle2 = thread::spawn(move || {
        if write_health_stats {
            std::thread::sleep(Duration::from_secs(4));
            match health_stats(write_every) {
                Ok(_) => (),
                Err(err) => {
                    println!("{err}")
                }
            };
            //println!("write stats")
        };
    });

    handle1.join().expect("Thread 1 panicked");
    handle2.join().expect("Thread 2 panicked");
    Ok(())
}

//const FACTORY_VALUE: u32 = 3620000;
//const FIRST_REC_VALUE: u32 = 3189000; // 24/03/2024
/*
cat /sys/class/power_supply/BAT1/charge_full_design                          03/24/24 12:11:48 PM
3620000
~> cat /sys/class/power_supply/BAT1/charge_full                                 03/24/24 12:13:55 PM
3189000
~> cat /sys/class/power_supply/BAT1/charge_now                                  03/24/24 12:14:02 PM
2893000
*/

#[allow(unreachable_code)]
pub fn health_stats(write_timer: u64) -> Result<(), Box<dyn Error>> {
    notify_percentage("N/A", "health_stats is running");
    /*
    Write to a csv file with columns today's date, charge_full, charge_full_design, battery health
    this last one is calculated as charge_full/charge_full_design
     */
    loop {
        let battery_stats = BatteryStatistics::new();
        let charge_full: f32 = battery_stats.charge_full.parse()?;
        let charge_full_design: f32 = battery_stats.charge_full_design.parse()?;

        // Calculate battery health
        let battery_health = charge_full / charge_full_design * 100.0;
        let battery_percentage = get_battery_percentage().expect("Failed getting batt percentage");
        let battery_status = get_battery_state().expect("Failed getting battery status");

        // Get today's date
        let today = chrono::Local::now();

        let now_date = format!("{}", today.format("%d/%m/%Y"));
        let now_hour = format!("{}", today.format("%H:%M"));
        let now_hour_as_float = {
            let hours : f32 = format!("{}", today.format("%H")).parse().unwrap();
            let minutes : f32 = format!("{}", today.format("%M")).parse().unwrap();

            let minutes_in_hours = minutes / 60.0;

            hours + minutes_in_hours
        };

        // Open or create the CSV file
        let file_path = Path::new(DATA_FILE_PATH);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        // Write headers if the file is newly created
        if file.metadata()?.len() == 0 {
            writeln!(
                file,
                "Date,Hour_str,Hour_f32,Charge_Full,Charge_Full_Design,Battery_Health,Battery_Percentage,Battery_Status"
            )?;
        }

        // Write data to the CSV file
        writeln!(
            file,
            "{now_date},{now_hour},{now_hour_as_float},{charge_full},{charge_full_design},{battery_health},{battery_percentage},{battery_status}"
        )?;

        println!("Battery health stats written to battery_stats.csv");
        thread::sleep(Duration::from_secs(write_timer * 60));
    }
    Ok(())
}

fn notifier(has_been_notified_80: &mut bool, has_been_notified_20: &mut bool) {
    
    notify_percentage("N/A", "notifier is running");
    loop {
        let battery_state = BatteryState::match_string(&get_battery_state().unwrap());
        let batt_percentage = get_battery_percentage().unwrap();
        let to_notify_80 = batt_percentage >= CHARGE_UPPER_LIMIT
            && !*has_been_notified_80
            && battery_state == BatteryState::Charging;
        let to_notify_20 = batt_percentage <= DISCHARGE_LOWER_LIMIT
            && !*has_been_notified_20
            && battery_state == BatteryState::Discharging;

        if to_notify_80 {
            notify_percentage("80%", "Sconnetti il caricatore!!");
            *has_been_notified_80 = true;
            *has_been_notified_20 = false;
        } else if to_notify_20 {
            notify_percentage("20%", "Connetti il caricatore!!");
            *has_been_notified_20 = true;
            *has_been_notified_80 = false;
        }
        thread::sleep(Duration::from_secs(BATTERY_CHECK_TIME))
    }
}
