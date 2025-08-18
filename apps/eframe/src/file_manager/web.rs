use eframe::Storage;
use gb_core::GameBoy;

use crate::file_manager::FileInfo;

pub fn load_battery(gb: &mut GameBoy, storage: &dyn Storage, file_info: &FileInfo) {
    let key = &file_info.name;
    let value = storage.get_string(key);

    if let Some(value) = value {
        let battery = value.as_bytes().to_vec();
        gb.load_battery(battery);
    }
}

pub fn save_battery(gb: &GameBoy, storage: &mut dyn Storage, file_info: &FileInfo) {
    if let Some(battery) = gb.get_battery() {
        let key = file_info.name.clone();
        let value = String::from_utf8_lossy(battery).to_string();

        storage.set_string(&key, value);
    }
}
