mod args;
mod config;

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

fn reset_to_default_monitor_settings(monitors: &Vec<&Monitor>) -> hyprland::Result<()> {
    for monitor in monitors {
        Keyword::set("monitor", format!("{name}, {width}x{height}@{refresh_rate}, {x}x{y}, {scale}",
                                        name = monitor.name,
                                        width = monitor.width,
                                        height = monitor.height,
                                        refresh_rate = monitor.refresh_rate,
                                        x = monitor.x,
                                        y = monitor.y,
                                        scale = monitor.scale))?;
        println!(
            "{name}, {width}x{height}@{refresh_rate}, {x}@{y}, {scale}",
            name = monitor.name,
            width = monitor.width,
            height = monitor.height,
            refresh_rate = monitor.refresh_rate,
            x = monitor.x,
            y = monitor.y,
            scale = monitor.scale
        );
    }

    Ok(())
}

fn mirror_monitor(primary_monitor: &Monitor, secondary_monitor: &Monitor) -> hyprland::Result<()> {
    let mirror_monitor_settings = format!("{name}, {width}x{height}@{refresh_rate}, {x}x{y}, {scale}, mirror, {main_monitor_name}",
                                          name = secondary_monitor.name,
                                          width = secondary_monitor.width,
                                          height = secondary_monitor.height,
                                          refresh_rate = secondary_monitor.refresh_rate,
                                          x = secondary_monitor.x,
                                          y = secondary_monitor.y,
                                          scale = secondary_monitor.scale,
                                          main_monitor_name = primary_monitor.name);
    Keyword::set("monitor", mirror_monitor_settings.clone())?;
    println!("{}", mirror_monitor_settings.clone());
    Ok(())
}

fn extend_to_right(primary_monitor: &Monitor, secondary_monitor: &Monitor) -> hyprland::Result<()> {
    let new_primary_monitor_settings = format!("{name}, {width}x{height}@{refresh_rate}, {x}x{y}, {scale}",
                                               name = primary_monitor.name,
                                               width = primary_monitor.width,
                                               height = primary_monitor.height,
                                               refresh_rate = primary_monitor.refresh_rate,
                                               x = 0,
                                               y = primary_monitor.y,
                                               scale = primary_monitor.scale);
    let new_secondary_monitor_settings = format!("{name}, {width}x{height}@{refresh_rate}, {x}x{y}, {scale}",
                                                 name = secondary_monitor.name,
                                                 width = secondary_monitor.width,
                                                 height = secondary_monitor.height,
                                                 refresh_rate = secondary_monitor.refresh_rate,
                                                 x = (primary_monitor.width as f32 / primary_monitor.scale),
                                                 y = secondary_monitor.y,
                                                 scale = secondary_monitor.scale);
    Keyword::set("monitor", new_primary_monitor_settings.clone())?;
    Keyword::set("monitor", new_secondary_monitor_settings.clone())?;
    println!("{}", new_primary_monitor_settings);
    println!("{}", new_secondary_monitor_settings);
    Ok(())
}

fn extend_to_left(primary_monitor: &Monitor, secondary_monitor: &Monitor) -> hyprland::Result<()> {
    extend_to_right(secondary_monitor, primary_monitor)?;
    Ok(())
}

fn send_notification(content: &String) {
    Notification::new()
        .summary("HyprDisplay")
        .body(content)
        .show().expect("Error sending notification");
}

fn set_monitor_mode(monitor_mode: u8, primary_monitor: &Monitor, secondary_monitor: &Monitor) {
    match monitor_mode {
        0 => {
            send_notification(&String::from("Mirroring monitor"));
            match mirror_monitor(primary_monitor, secondary_monitor) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error trying to mirror monitor: {}", e)
                }
            }
        }
        1 => {
            send_notification(&String::from("Extending monitor to left"));
            match extend_to_left(primary_monitor, secondary_monitor) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error trying to extend monitor to the left: {}", e)
                }
            }
        }
        2 => {
            send_notification(&String::from("Extending monitor to right"));
            match extend_to_right(primary_monitor, secondary_monitor) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error trying to extend monitor to the right: {}", e)
                }
            }
        }
        _ => { panic!("Error out of bounds value") }
    }
}

fn get_and_validate_monitors(config: &mut Config) -> Result<(Monitor, Monitor), Box<dyn Error>> {
    let monitors = Monitors::get()?.to_vec();

    if monitors.len() != 2 {
        panic!("HyprDisplay can only handle two monitors at a time");
    }

    for monitor in &monitors {
        if !config.monitors.contains_key(&monitor.description) {
            println!("New monitor found adding to config");
            config.monitors.insert(String::from(&monitor.description), monitor.clone());
        }
    }

    let first_monitor = monitors.get(0).unwrap();
    let second_monitor = monitors.get(1).unwrap();

    let (primary_monitor, secondary_monitor) = if config.default == first_monitor.description {
        (first_monitor, second_monitor)
    } else {
        (second_monitor, first_monitor)
    };

    if primary_monitor.description != config.default {
        println!("No primary monitor found");
        println!("Please set a default monitor\nExiting");
        panic!("No default monitor");
    }

    Ok((primary_monitor.clone(), secondary_monitor.clone()))
}

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