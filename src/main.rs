use std::{
    collections::HashSet,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use clap::Parser;
use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(Parser)]
struct Cli {
    /// Duration in seconds the keys must be held for in order to register.
    /// If a queried key is not held down or released during this time,
    /// then the program will exit with code 1.
    #[clap(long)]
    hold_for: Option<f64>,

    keys: Vec<Keycode>,
}

fn check_keys(state: &DeviceState, to_check: &HashSet<Keycode>) -> bool {
    let keys: HashSet<_> = state.get_keys().into_iter().collect();
    to_check.is_subset(&keys)
}

fn main() {
    let cli = Cli::parse();

    let to_check = cli.keys.into_iter().collect();

    let state = DeviceState::new();

    let query_successful = match cli.hold_for {
        Some(dur) => {
            let deadline = Instant::now() + Duration::from_secs_f64(dur);
            loop {
                if !check_keys(&state, &to_check) {
                    break false;
                }
                if Instant::now() >= deadline {
                    break true;
                }
                // Poll with about 100 Hz
                sleep(Duration::from_millis(10));
            }
        }
        None => check_keys(&state, &to_check),
    };

    let exit_code = if query_successful { 0 } else { 1 };

    exit(exit_code);
}
