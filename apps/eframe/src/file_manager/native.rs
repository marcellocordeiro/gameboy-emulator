use std::{io::Result, path::Path};

use eframe::Storage;
use gb_core::{GameBoy, constants::BATTERY_EXTENSIONS};
use tracing::info;

use crate::file_manager::{BatteryStorage, FileInfo, FileManager};

impl BatteryStorage for FileManager {
    fn load_battery(
        _storage: Option<&dyn Storage>,
        file_info: &FileInfo,
    ) -> Result<Option<Vec<u8>>> {
        let rom_path = &file_info.path;

        let [extension] = BATTERY_EXTENSIONS;
        let path = Path::new(rom_path).with_extension(extension);

        if !path.try_exists()? {
            info!("No battery file was found.");
            return Ok(None);
        }

        info!("Loading battery file from {}", path.display());
        let file = std::fs::read(path)?;

        Ok(Some(file))
    }

    fn save_battery(
        gb: &GameBoy,
        _storage: Option<&mut dyn Storage>,
        file_info: Option<&FileInfo>,
    ) -> Result<()> {
        let Some(file_info) = &file_info else {
            return Ok(());
        };

        let Some(battery) = gb.get_battery() else {
            return Ok(());
        };

        info!("Saving battery file...");

        let rom_path = &file_info.path;
        let [extension] = BATTERY_EXTENSIONS;
        let path = Path::new(rom_path).with_extension(extension);

        std::fs::write(path, battery)
    }
}
