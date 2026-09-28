use std::io::Result;

use eframe::Storage;
use gb_core::GameBoy;

use crate::file_manager::{BatteryStorage, FileInfo, FileManager};

impl BatteryStorage for FileManager {
    fn load_battery(
        storage: Option<&dyn Storage>,
        file_info: &FileInfo,
    ) -> Result<Option<Vec<u8>>> {
        let Some(storage) = storage else {
            return Ok(None);
        };

        let key = file_info.path.file_name().unwrap().to_str().unwrap();

        let Some(value) = storage.get_string(key) else {
            return Ok(None);
        };

        let battery = value.as_bytes().to_vec();
        Ok(Some(battery))
    }

    fn save_battery(
        gb: &GameBoy,
        storage: Option<&mut dyn Storage>,
        file_info: Option<&FileInfo>,
    ) -> Result<()> {
        let (Some(storage), Some(file_info)) = (storage, file_info) else {
            return Ok(());
        };

        let Some(battery) = gb.get_battery() else {
            return Ok(());
        };

        let key = file_info.path.file_name().unwrap().to_str().unwrap();
        let value = String::from_utf8_lossy(battery).to_string();

        storage.set_string(key, value);

        Ok(())
    }
}
