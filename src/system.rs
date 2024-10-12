use nix::unistd::Uid;
use std::fs;
use std::path::Path;
use std::process;

pub fn is_root() -> bool {
    Uid::effective().is_root()
}

pub fn check_driver() -> bool {
    let scaling_driver_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver";
    match fs::read_to_string(scaling_driver_path) {
        Ok(content) => content.trim() == "amd-pstate-epp",
        Err(_) => false,
    }
}

pub fn is_charging() -> bool {
    let power_supply_path = Path::new("/sys/class/power_supply");
    let mut power_supplies: Vec<_> = fs::read_dir(power_supply_path)
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    power_supplies.sort_by_key(|dir| dir.file_name());

    for supply in power_supplies {
        let supply_path = supply.path();
        let type_path = supply_path.join("type");
        let type_content = fs::read_to_string(type_path).unwrap_or_default();

        match type_content.trim() {
            "Mains" => {
                let online_path = supply_path.join("online");
                if let Ok(online) = fs::read_to_string(online_path) {
                    if online.trim() == "1" {
                        return true;
                    }
                }
            }
            "Battery" => {
                let status_path = supply_path.join("status");
                if let Ok(status) = fs::read_to_string(status_path) {
                    if status.trim() == "Discharging" {
                        return false;
                    }
                }
            }
            _ => continue,
        }
    }

    true
}

pub fn set_governor() {
    let governor_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
    let current_governor = fs::read_to_string(governor_path).unwrap_or_default().trim().to_string();

    if current_governor != "powersave" {
        println!("Current governor {} is not powersafe. Setting governor to powersafe", current_governor);

        let cpu_count = num_cpus::get();
        for cpu in 0..cpu_count {
            let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor", cpu);
            if let Err(e) = fs::write(path, "powersafe") {
                eprint!("Failed to set governor for CPU: {}: {}", cpu, e);
                process::exit(1);
            }
        }
    }
}

pub fn set_epp(epp_value: &str) {
    let cpu_count = num_cpus::get();
    for cpu in 0..cpu_count {
        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/energy_performance_preference", cpu);
        if let Err(e) = fs::write(path, epp_value) {
            eprintln!("Failed to set EPP for CPU {}: {}", cpu, e);
            process::exit(1);
        }
    }
}