#![allow(dead_code)]

use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
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
        let is_active = buf.chars().nth(0).unwrap_or(' ') == '*';

        ConfigDevice {
            name: (&buf[(is_active as usize)..]).to_string(),
            is_active,
        }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;

        if self.is_active {
            Command::new("nircmd")
                .args(["setdefaultsounddevice", self.name.as_str()])
                .output()
                .unwrap();
        }
    }
}

impl fmt::Display for ConfigDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            devices: lines.iter().map(|l| ConfigDevice::from(l)).collect(),
        }
    }

    pub fn read(path: &Path) -> io::Result<Config> {
        let mut file = fs::File::open(path)?;
        let mut buf: String = String::new();
        file.read_to_string(&mut buf)?;
        Ok(Config::from(&buf))
    }

    pub fn write(&self, path: &PathBuf) -> io::Result<()> {
        let mut file = fs::File::options().truncate(true).write(true).open(path)?;

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

        Ok(())
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
