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
    pub fn load_battery(&self, gb: &mut GameBoy) -> Result<(), std::io::Error> {
        if let Some(rom) = &self.rom {
            native::load_battery(gb, &rom.path)
        } else {
            Ok(())
        }
    }

    pub fn save_battery(&self, gb: &GameBoy) -> Result<(), std::io::Error> {
        if let Some(rom) = &self.rom {
            native::save_battery(gb, &rom.path)
        } else {
            Ok(())
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl FileManager {
    #[allow(clippy::unnecessary_wraps)]
    pub fn load_battery(&self, gb: &mut GameBoy) -> Result<(), std::io::Error> {
        if let Some(rom) = &self.rom {
            web::load_battery(gb, &rom.name);
        }

        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn save_battery(&self, gb: &GameBoy) -> Result<(), std::io::Error> {
        if let Some(rom) = &self.rom {
            web::save_battery(gb, &rom.name);
        }

        Ok(())
    }
}

mod file_info;
mod file_picker_async;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;
