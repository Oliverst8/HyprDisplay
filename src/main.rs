mod args;
mod config;
mod utils;
mod monitors;
mod menu;

use std::error::Error;
use clap::Parser;
use crate::args::Args;
use crate::config::{get_config_file, setup, write_config_file, Config};
use crate::menu::show_menu;
use crate::monitors::{apply_monitor_mode, get_and_validate_monitors, set_monitor_mode_by_string};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.setup {
        setup()?;
        return Ok(());
    }

    let mut config = match get_config_file() {
        Ok(config) => config,
        Err(e) => {
            println!("Error getting config, do you have a config file?");
            return Err(e);
        }
    };

    match (args.mode, args.next_mode, args.menu) {
        (Some(mode), false, false) => {
            let (primary_monitor, secondary_monitor) = get_and_validate_monitors(&mut config)?;
            set_monitor_mode_by_string(config, &mode, &primary_monitor, &secondary_monitor)
        }
        (None, true, false) => {
            let (primary_monitor, secondary_monitor) = get_and_validate_monitors(&mut config)?;
            config.current_monitor_mode = (config.current_monitor_mode + 1) % 3;
            apply_monitor_mode(config, &primary_monitor, &secondary_monitor);
        }
        (None, false, true) => {
            let (primary_monitor, secondary_monitor) = get_and_validate_monitors(&mut config)?;
            show_menu(&config, &primary_monitor, &secondary_monitor)
        }
        _ => { println!("Please supply either current_mode or mode see --help for usage") }
    }


    Ok(())
}