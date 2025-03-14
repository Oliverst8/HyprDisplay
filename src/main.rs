mod args;
mod config;
mod utils;
mod monitors;

use std::collections::HashMap;
use std::error::Error;
use std::{env, fs};
use hyprland::keyword::Keyword;
use hyprland::data::*;
use hyprland::prelude::*;
use notify_rust::{Notification};
use std::io::{BufReader, BufWriter};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};
use clap::Parser;
use crate::args::Args;
use crate::config::{get_config_file, setup, write_config_file, Config};
use crate::monitors::{get_and_validate_monitors, set_monitor_mode};

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
            set_monitor_mode(config.current_monitor_mode, &primary_monitor, &secondary_monitor);
            write_config_file(config)
        }
    }

    Ok(())
}