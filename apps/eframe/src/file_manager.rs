use std::io::Result;

use eframe::Storage;
pub use file_info::FileInfo;
pub use file_picker_async::{FileType, file_picker_async};
use gb_core::GameBoy;

#[derive(Default)]
pub struct FileManager {
    pub bootrom: Option<FileInfo>,
    pub rom: Option<FileInfo>,
}

pub trait BatteryStorage {
    fn load_battery(storage: Option<&dyn Storage>, file_info: &FileInfo)
    -> Result<Option<Vec<u8>>>;

    fn save_battery(
        gb: &GameBoy,
        _storage: Option<&mut dyn Storage>,
        file_info: Option<&FileInfo>,
    ) -> Result<()>;
}

mod file_info;
mod file_picker_async;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;
