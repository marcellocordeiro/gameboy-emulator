use eframe::Storage;
pub use file_info::FileInfo;
pub use file_picker_async::{FileType, file_picker_async};
use gb_core::GameBoy;

#[derive(Default)]
pub struct FileManager {
    pub bootrom: Option<FileInfo>,
    pub rom: Option<FileInfo>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileManager {
    pub fn load_battery(gb: &mut GameBoy, _storage: Option<&dyn Storage>, file_info: &FileInfo) {
        native::load_battery(gb, file_info).unwrap();
    }

    pub fn save_battery(
        gb: &GameBoy,
        _storage: Option<&mut dyn Storage>,
        file_info: Option<&FileInfo>,
    ) {
        if let Some(file_info) = &file_info {
            native::save_battery(gb, file_info).unwrap();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FileManager {
    pub fn load_battery(gb: &mut GameBoy, storage: Option<&dyn Storage>, file_info: &FileInfo) {
        if let Some(storage) = storage {
            web::load_battery(gb, storage, file_info);
        }
    }

    pub fn save_battery(
        gb: &GameBoy,
        storage: Option<&mut dyn Storage>,
        file_info: Option<&FileInfo>,
    ) {
        if let Some(storage) = storage
            && let Some(file_info) = file_info
        {
            web::save_battery(gb, storage, file_info);
        }
    }
}

mod file_info;
mod file_picker_async;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;
