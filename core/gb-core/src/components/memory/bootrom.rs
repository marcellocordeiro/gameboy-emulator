use std::sync::Arc;

use thiserror::Error;

use crate::DeviceModel;

/// DMG
///
/// Size: 256 (0x100)
const DMG_BOOTROM_SIZE: usize = 0x100;

/// CGB
///
/// Size: 256 + 256 + 1792 = 2304 (0x900)
///
/// Note: the extra 256 bytes in the middle is mapped to the cartridge header until 0x200.
const CGB_BOOTROM_SIZE: usize = 0x100 + 0x100 + 0x700;

#[derive(Debug, Error)]
pub enum BootromError {
    #[error("Invalid ROM.")]
    InvalidBootrom,
}

#[derive(Default)]
pub struct Bootrom {
    data: Option<Arc<[u8]>>,
}

impl Bootrom {
    pub fn try_new(device_model: DeviceModel, bootrom: Arc<[u8]>) -> Result<Self, BootromError> {
        let is_valid = match device_model {
            DeviceModel::Dmg => bootrom.len() == DMG_BOOTROM_SIZE,
            DeviceModel::Cgb => bootrom.len() == CGB_BOOTROM_SIZE,
        };

        if !is_valid {
            return Err(BootromError::InvalidBootrom);
        }

        Ok(Self {
            data: Some(bootrom),
        })
    }

    pub fn mapped(&self) -> bool {
        self.data.is_some()
    }

    pub fn unmap(&mut self) {
        self.data = None;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data
            .as_ref()
            .map_or(0xFF, |data| data[address as usize])
    }

    pub fn read_status(&self) -> u8 {
        (!self.mapped() as u8) | 0b1111_1110
    }
}
