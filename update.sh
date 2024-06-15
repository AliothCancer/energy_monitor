cargo build --release;
systemctl --user stop energy_monitor.service;
cp /home/giulio/Documenti/project/energy_monitor/target/release/energy_monitor /home/giulio/Programmi/energy_monitor/
systemctl --user start energy_monitor.service
