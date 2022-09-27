#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::path::PathBuf;

mod cfg;
use cfg::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create files & folders
    let appdata = PathBuf::from(env::var("APPDATA")?);
    let mut dir_path = appdata.clone();
    dir_path.push("Audiodevice");

    if !dir_path.exists() {
        fs::create_dir(&dir_path)?;
        println!("Created folder {}", dir_path.display());
    }

    let mut config_path = dir_path.clone();
    config_path.push("config.cfg");

    if !config_path.exists() {
        fs::File::create(&config_path)?;
        println!("Created file {}", config_path.display());
        println!("Fill it out with sound devices");
        return Ok(());
    }

    // Change config
    let mut config = Config::read(&config_path);

    for device in &mut config.devices {
        println!(
            "{}: {} -> {}",
            device.name, device.is_active, !device.is_active
        );
        device.toggle();
    }

    config.write(&config_path);
    Ok(())
}
