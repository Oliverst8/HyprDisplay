mod args;
mod config;
mod utils;
mod monitors;


use std::error::Error;
use clap::Parser;
use crate::args::Args;
use crate::config::{get_config_file, setup, write_config_file, Config};
use crate::monitors::{apply_monitor_mode, get_and_validate_monitors};

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

    match (args.mode, args.next_mode) {
        (None, false) | (Some(_), true) => { println!("Please supply either current_mode or mode see --help for usage") }
        (Some(mode), false) => { println!("TBD") }
        (None, true) => {
            let (primary_monitor, secondary_monitor) = get_and_validate_monitors(&mut config)?;
            config.current_monitor_mode = (config.current_monitor_mode + 1) % 3;
            apply_monitor_mode(config, &primary_monitor, &secondary_monitor);
        }
    }

    Ok(())
}