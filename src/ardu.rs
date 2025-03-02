use std::io::{stdout, Write};
use std::{process::Command, thread::sleep, time::Duration};

use super::CHARGE_UPPER_LIMIT;

use super::battery_health::{get_battery_percentage, get_battery_state, BatteryState};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArduSketch {
    DoNothing,
    Disconnect,
    Connect,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandState {
    ToExecute,
    Executing,
    Stopped,
}

const DO_NOTHING_PATH: &str = "/home/giulio/arduino_embedded/do_nothing";
const CONNECT_PATH: &str = "/home/giulio/arduino_embedded/connect_charger";
const DISCONNECT_PATH: &str = "/home/giulio/arduino_embedded/disconnect_charger";

#[derive(Debug, Clone, Copy)]
pub struct ArduCommand {
    pub command_type: ArduSketch,
    pub state: CommandState,
}
impl ArduCommand {
    pub fn execute(&self) {
        match self.command_type {
            ArduSketch::DoNothing => {
                Command::new("avrdude")
                    .args([
                        "-c",
                        "arduino",
                        "-p",
                        "m328p",
                        "-P",
                        "/dev/ttyACM0",
                        "-U",
                        "flash:w:target/avr-atmega328p/release/do_nothing.elf",
                    ])
                    .current_dir(DO_NOTHING_PATH)
                    .status()
                    .unwrap();
                println!("DoNothing is being executed!\n");
                
                'do_nothing: loop {
                    let batt_perc = get_battery_percentage().expect("Failed getting batt percentage");
                    if CHARGE_UPPER_LIMIT - 0.1 < batt_perc || batt_perc < CHARGE_UPPER_LIMIT + 0.1{
                        print!("batt_charg:{}  \r", batt_perc);
                        stdout().flush().unwrap();
                        sleep(Duration::from_secs(1));
                        continue;
                    }else {
                        break 'do_nothing;
                    }
                    }
                },
            ArduSketch::Disconnect => {
                Command::new("avrdude")
                    .args([
                        "-c",
                        "arduino",
                        "-p",
                        "m328p",
                        "-P",
                        "/dev/ttyACM0",
                        "-U",
                        "flash:w:target/avr-atmega328p/release/disconnect_charger.elf",
                    ])
                    .current_dir(DISCONNECT_PATH)
                    .status()
                    .unwrap();
                println!("Disconnect is being executed!");
                'disconnecting: loop {
                    //let batt_perc = get_battery_percentage().expect("Failed getting batt percentage");
                    let batt_state = BatteryState::match_string(&get_battery_state().unwrap());
                    match batt_state {
                        BatteryState::Discharging => {
                            sleep(Duration::from_secs(13));
                            break 'disconnecting;
                        }
                        BatteryState::Charging | BatteryState::Full => {
                            sleep(Duration::from_secs(1))
                        }
                    }
                }
            }
            ArduSketch::Connect => {
                Command::new("avrdude")
                    .args([
                        "-c",
                        "arduino",
                        "-p",
                        "m328p",
                        "-P",
                        "/dev/ttyACM0",
                        "-U",
                        "flash:w:target/avr-atmega328p/release/connect_charger.elf",
                    ])
                    .current_dir(CONNECT_PATH)
                    .status()
                    .unwrap();
                println!("Connect is being executed!");
                'connecting: loop {
                    //let batt_perc = get_battery_percentage().expect("Failed getting batt percentage");
                    let batt_state = BatteryState::match_string(&get_battery_state().unwrap());
                    match batt_state {
                        BatteryState::Charging => {
                            sleep(Duration::from_secs(19));
                            break 'connecting;
                        }
                        BatteryState::Discharging | BatteryState::Full => {
                            sleep(Duration::from_secs(1))
                        }
                    }
                }
            }
        }
    }
}
