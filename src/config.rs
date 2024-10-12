use configparser::ini::Ini;
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = "/etc/auto-epp.conf";
const DEFAULT_CONFIG: &str = r#"# see available epp state by running: cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_available_preferences
[Settings]
epp_state_for_AC=balance_performance
epp_state_for_BAT=power
"#;

pub struct Config {
    pub epp_state_for_ac: String,
    pub epp_state_for_bat: String,
}

pub fn read_config() -> Config {
    if !Path::new(CONFIG_FILE).exists() {
        fs::write(CONFIG_FILE, DEFAULT_CONFIG).expect("Failed to write default config");
    }

    let mut config = Ini::new();
    config
        .load(CONFIG_FILE)
        .expect("Failed to load config file");

    Config {
        epp_state_for_ac: config
            .get("Settings", "epp_state_for_AC")
            .unwrap_or_else(|| "balance_performance".to_string()),
        epp_state_for_bat: config
            .get("Settings", "epp_state_for_BAT")
            .unwrap_or_else(|| "power".to_string()),
    }
}
