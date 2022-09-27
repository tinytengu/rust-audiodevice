#![allow(dead_code)]

use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

pub struct ConfigDevice {
    pub name: String,
    pub is_active: bool,
}

impl ConfigDevice {
    pub fn new(name: String, is_active: bool) -> ConfigDevice {
        ConfigDevice { name, is_active }
    }

    pub fn from(buf: &str) -> ConfigDevice {
        let is_active = buf.chars().nth(0).unwrap() == '*';
        let name = (&buf[(is_active as usize)..]).to_string();

        ConfigDevice { name, is_active }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;

        if !self.is_active {
            return;
        }

        Command::new("nircmd")
            .args(["setdefaultsounddevice", self.name.as_str()])
            .output()
            .expect("Unable to execute nircmd command");
    }
}

impl fmt::Display for ConfigDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.is_active { "*" } else { "" }, self.name)
    }
}

pub struct Config {
    pub devices: Vec<ConfigDevice>,
}

impl Config {
    pub fn new() -> Config {
        Config { devices: vec![] }
    }

    pub fn from(buf: &str) -> Config {
        let lines: Vec<&str> = buf.split('\n').map(|i| i.trim()).collect();
        Config {
            devices: lines.iter().map(|i| ConfigDevice::from(i)).collect(),
        }
    }

    pub fn read(path: &PathBuf) -> Config {
        let mut file = fs::File::open(path).expect("Unable to open config file");
        let mut buf: String = String::new();
        file.read_to_string(&mut buf).unwrap();
        Config::from(&buf)
    }

    pub fn write(&self, path: &PathBuf) {
        let mut file = fs::File::options()
            .truncate(true)
            .write(true)
            .open(path)
            .unwrap();

        write!(
            file,
            "{}",
            self.devices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
        .unwrap();
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
