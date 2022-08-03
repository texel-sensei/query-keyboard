use std::{collections::HashSet, process::exit};

use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let state = DeviceState::new();
    let keys: HashSet<_> = state.get_keys().into_iter().collect();

    let to_check: Result<HashSet<Keycode>, _> =
        std::env::args().skip(1).map(|a| a.parse()).collect();

    let exit_code = match to_check {
        Ok(to_check) => {
            if to_check.is_subset(&keys) {
                0
            } else {
                1
            }
        }
        Err(e) => {
            eprintln!("Invalid arguments: {e}");
            5
        }
    };

    exit(exit_code);
}
