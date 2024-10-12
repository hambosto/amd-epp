
# AMD-EPP

  

Auto-EPP is a Rust program that automatically manages the Energy Performance Preference (EPP) settings for AMD CPUs using the amd-pstate-epp driver. It adjusts the EPP based on whether the system is running on battery or AC power.

  

## Features

  

- Automatically switches between performance and power-saving EPP states
- Configurable EPP states for AC and battery power
- Continuously monitors power source changes
- Ensures the CPU governor is set to "powersave"

  

## Requirements
- Rust programming language (latest stable version)
- Linux system with an AMD CPU supporting the amd-pstate-epp driver
- Root privileges for execution

  

## Installation
1. Clone the repository:
2. 
```
git clone https://github.com/hambosto/amd-epp.git

cd amd-epp
```
2. Build the project:
```
cargo build --release
```
3. Copy the binary to a system directory:
```
sudo cp target/release/amd-epp /usr/local/bin/
```
## Configuration
Edit the configuration file at `/etc/amd-epp.conf` to set your preferred EPP states:
```ini
[Settings]
epp_state_for_AC=balance_performance
epp_state_for_BAT=power
```
Available EPP states can be found by running:
```
cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences
```

## Usage
Run the program with root privileges:
```
sudo amd-epp
```

It's recommended to set up auto-epp as a systemd service to run automatically at boot.


## Creating a systemd Service
1. Create a systemd service file:

```
sudo nano /etc/systemd/system/amd-epp.service
```

  

2. Add the following content:

```
[Unit]
Description=amd-epp - Automatic EPP Changer for amd-pstate-epp
After=network.target network-online.target

[Service]
Type=simple
User=root
ExecStart=/usr/bin/amd-epp
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

  

3. Enable and start the service:

```
sudo systemctl enable amd-epp.service
sudo systemctl start amd-epp.service
```

  

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

  

## License
This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

  

## Acknowledgments
- Original Python script by [jothi-prasath](https://github.com/jothi-prasath/auto-epp)
- All contributors to the Rust crates used in this project