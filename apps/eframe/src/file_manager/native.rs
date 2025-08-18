use std::path::Path;

use gb_core::{GameBoy, constants::BATTERY_EXTENSIONS};

use crate::file_manager::FileInfo;

pub fn load_battery(gb: &mut GameBoy, file_info: &FileInfo) -> Result<(), std::io::Error> {
    let rom_path = &file_info.path;

    let [extension] = BATTERY_EXTENSIONS;
    let path = Path::new(rom_path).with_extension(extension);

    if !path.try_exists()? {
        log::info!("No battery file was found.");
        return Ok(());
    }

    log::info!("Loading battery file from {}", path.display());
    let file = std::fs::read(path)?;
    gb.load_battery(file);

    Ok(())
}

pub fn save_battery(gb: &GameBoy, file_info: &FileInfo) -> Result<(), std::io::Error> {
    let Some(battery) = gb.get_battery() else {
        return Ok(());
    };

    log::info!("Saving battery file...");

    let rom_path = &file_info.path;
    let [extension] = BATTERY_EXTENSIONS;
    let path = Path::new(rom_path).with_extension(extension);

    std::fs::write(path, battery)
}
