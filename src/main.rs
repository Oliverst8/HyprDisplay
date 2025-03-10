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

/// A command-line tool for managing monitor configurations with Hyprland.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Monitor mode to switch to, supports mirror extend left/right
    #[arg(short, long)]
    mode: Option<String>,

    /// Switch to the next monitor mode
    #[arg(short, long)]
    next_mode: bool,

    /// Whether to run setup
    #[arg(long)]
    setup: bool,
}
#[derive(Serialize, Deserialize)]
struct Config {
    default: String,
    /// 0 = Mirror, 1 = extend left, 2 = extend right
    current_monitor_mode: u8,
    monitors: HashMap<String, Monitor>,
}

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

fn get_config_dir_path() -> PathBuf {
    let mut config_dir = env::home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("HyprDisplay");
    config_dir
}

fn get_config_file_path() -> PathBuf {
    let mut config_file = get_config_dir_path();
    config_file.push("HyprDisplay.json");
    config_file
}

fn write_to_file<T: Serialize>(path_buf: PathBuf, data: &T) -> Result<(), Box<dyn Error>> {
    let f = BufWriter::new(fs::File::create(path_buf)?);

    //let encoded = serde_json::to_vec(&data).unwrap();
    println!("Writing config file");
    //f.write_all(&encoded)?;
    serde_json::to_writer_pretty(f, data)?;
    println!("Finished Writing config file");
    Ok(())
}

fn write_config_file(config: Config) {
    //home_dir is no longer deprecated and is recommended from rust 1.85
    let config_dir = get_config_dir_path();
    if !Path::exists(&*config_dir) {
        fs::create_dir(config_dir.clone()).expect("Could not create config folder");
    }
    let config_file = get_config_file_path();

    write_to_file(config_file, &config).unwrap()
}

fn get_config_file() -> Result<Config, Box<dyn Error>> {
    let config_file_path = get_config_file_path();
    let r = BufReader::new(fs::File::open(config_file_path).unwrap());
    let config = serde_json::from_reader(r)?;
    Ok(config)
}

fn setup() -> Result<(), Box<dyn Error>> {
    let mut config = Config {
        default: String::from(""),
        monitors: HashMap::new(),
        current_monitor_mode: 0,
    };

    let monitors = Monitors::get()?.to_vec();
    println!("Welcome to HyprDisplay\nPlease select your default monitor:");
    for (i, monitor) in monitors.iter().enumerate() {
        println!("{}", format!("{index}:\n\tInput: {inputName}\n\tDescription: {description}", index = i, inputName = monitor.name, description = monitor.description));
        config.monitors.insert((*monitor.description).to_string(), monitor.clone());
    }

    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {}
        Err(error) => {
            println!("error: {error}");
            panic!();
        }
    }

    let parsed_input = match user_input.trim().parse::<usize>() {
        Ok(number) => if (number < monitors.len()) { number } else {
            println!("Number out of range: {}", user_input);
            panic!("Number out of range");
        }

        Err(error) => {
            println!("Expected number in shown range: {}", user_input);
            panic!("{}", error);
        }
    };
    let default_monitor_description = &monitors[parsed_input].description;
    println!("Setting {} as default monitor", *default_monitor_description);

    config.default = String::from(default_monitor_description);
    write_config_file(config);

    Ok(())
}

fn send_notication(content: &String) {
    Notification::new()
        .summary("HyprDisplay")
        .body(content)
        .show().expect("Error sending notification");
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
            let monitors = Monitors::get()?.to_vec();

            if monitors.len() != 2 {
                panic!("HyprDisplay can only handle two monitors at a time")
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
                panic!("No default monitor")
            }

            config.current_monitor_mode = (config.current_monitor_mode + 1) % 3;
            match config.current_monitor_mode {
                0 => {
                    send_notication(&String::from("Mirroring monitor"));
                    match mirror_monitor(primary_monitor, secondary_monitor) {
                        Ok(_) => {}
                        Err(e) => {
                            panic!("Error trying to mirror monitor: {}", e)
                        }
                    }
                }
                1 => {
                    send_notication(&String::from("Extending monitor to left"));
                    match extend_to_left(primary_monitor, secondary_monitor) {
                        Ok(_) => {}
                        Err(e) => {
                            panic!("Error trying to extend monitor to the left: {}", e)
                        }
                    }
                }
                2 => {
                    send_notication(&String::from("Extending monitor to right"));
                    match extend_to_right(primary_monitor, secondary_monitor) {
                        Ok(_) => {}
                        Err(e) => {
                            panic!("Error trying to extend monitor to the right: {}", e)
                        }
                    }
                }
                _ => { panic!("Error out of bounds value") }
            }
            write_config_file(config)
        }
    }

    Ok(())
}