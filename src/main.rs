use std::process;
use std::thread;
use std::time::Duration;

mod config;
mod system;

fn main() {
    if !system::is_root() {
        eprintln!("amd-epp must be run with root priviledges.");
        process::exit(1);
    }

    if !system::check_driver() {
        eprintln!("The system is not running amd-pstate-epp");
        process::exit(1);
    }

    let config = config::read_config();

    loop {
        system::set_governor();
        if system::is_charging() {
            system::set_epp(&config.epp_state_for_ac);
        } else {
            system::set_epp(&config.epp_state_for_bat);
        }
        thread::sleep(Duration::from_secs(2));
    }
}
