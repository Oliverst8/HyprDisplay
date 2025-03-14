use std::collections::HashMap;
use std::{env, fs, io};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use hyprland::data::{Monitor, Monitors};
use hyprland::prelude::{HyprData, HyprDataVec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) default: String,
    /// 0 = Mirror, 1 = extend left, 2 = extend right
    pub(crate) current_monitor_mode: u8,
    pub(crate) monitors: HashMap<String, Monitor>,
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

pub(crate) fn write_config_file(config: Config) {
    //home_dir is no longer deprecated and is recommended from rust 1.85
    let config_dir = get_config_dir_path();
    if !Path::exists(&*config_dir) {
        fs::create_dir(config_dir.clone()).expect("Could not create config folder");
    }
    let config_file = get_config_file_path();

    write_to_file(config_file, &config).unwrap()
}

pub(crate) fn get_config_file() -> Result<Config, Box<dyn Error>> {
    let config_file_path = get_config_file_path();
    let r = BufReader::new(fs::File::open(config_file_path).unwrap());
    let config = serde_json::from_reader(r)?;
    Ok(config)
}

pub(crate) fn setup() -> Result<(), Box<dyn Error>> {
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