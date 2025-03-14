use std::collections::HashMap;
use std::{env, fs};
use std::error::Error;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use hyprland::data::Monitor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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