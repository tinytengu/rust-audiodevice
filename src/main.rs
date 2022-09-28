#![windows_subsystem = "windows"]

use std::env::{self, VarError};
use std::fs;
use std::path::PathBuf;

mod cfg;
use cfg::Config;
use notify_rust::Notification;

fn get_appdata_path() -> Result<PathBuf, VarError> {
    Ok(PathBuf::from(env::var("APPDATA")?))
}

fn get_app_path(parent: &PathBuf) -> PathBuf {
    let mut result = parent.clone();
    result.push("Audiodevice");
    result
}

fn get_config_path(app_dir: &PathBuf) -> PathBuf {
    let mut result = app_dir.clone();
    result.push("config.cfg");
    result
}

fn main() {
    // Files & folders
    let appdata = get_appdata_path().unwrap();
    let dir_path = get_app_path(&appdata);
    let config_path = get_config_path(&dir_path);

    if !dir_path.exists() {
        fs::create_dir(&dir_path).unwrap();
    }

    if !config_path.exists() {
        fs::File::create(&config_path).unwrap();
        Notification::new()
            .summary("Audiodevice")
            .body("Created config file. Fill it out with sound devices")
            .show()
            .unwrap();

        return;
    }

    // Change config
    let mut config = Config::read(&config_path).unwrap();

    for device in &mut config.devices {
        println!(
            "{}: {} -> {}",
            device.name, device.is_active, !device.is_active
        );
        device.toggle();
    }

    config.write(&config_path).unwrap();
}
