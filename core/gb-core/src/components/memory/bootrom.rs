use std::sync::Arc;

use thiserror::Error;

use crate::DeviceModel;

/// DMG
///
/// Size: 256 (0x100)
const DMG_BOOTROM_SIZE: usize = 0x100;

/// CGB
///
/// Size: 256 + 256 + 1792  = 2304 (0x900)
///
/// Note: the extra 256 bytes in the middle is mapped to the cartridge header until 0x200.
const CGB_BOOTROM_SIZE: usize = 0x100 + 0x100 + 0x700;

// #[cfg(feature = "bootrom")]
#[derive(Default)]
pub struct Bootrom {
    data: Option<Arc<Box<[u8]>>>,
    is_active: bool,
}

#[derive(Debug, Error)]
pub enum BootromError {
    #[error("Invalid ROM.")]
    InvalidBootrom,
}

// TODO: optional feature to bundle the bootrom
// #[cfg(feature = "bootrom")]
/* impl Default for Bootrom {
    fn default() -> Self {
        // #[cfg(not(feature = "cgb"))]
        // let data = include_bytes!("../../../../roms/bootrom/dmg_boot.bin");

        // #[cfg(feature = "cgb")]
        // let data = include_bytes!("../../../../roms/bootrom/cgb_boot.bin");

        Self {
            data,
            is_active: true,
        }
    }
} */

impl Bootrom {
    pub fn new(
        device_model: DeviceModel,
        bootrom: Option<Arc<Box<[u8]>>>,
    ) -> Result<Self, BootromError> {
        let Some(bootrom) = bootrom else {
            return Ok(Self {
                data: None,
                is_active: false,
            });
        };

        let is_valid = match device_model {
            DeviceModel::Dmg => bootrom.len() == DMG_BOOTROM_SIZE,
            DeviceModel::Cgb => bootrom.len() == CGB_BOOTROM_SIZE,
        };

        if !is_valid {
            return Err(BootromError::InvalidBootrom);
        }

        Ok(Self {
            data: Some(bootrom),
            is_active: true,
        })
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn disable(&mut self) {
        if self.data.is_none() {
            return;
        }

        self.is_active = false;
    }

    pub fn read(&self, address: u16) -> u8 {
        if let Some(data) = &self.data {
            data[address as usize]
        } else {
            0xFF
        }
    }

    pub fn read_status(&self) -> u8 {
        if self.data.is_none() {
            return 0xFF;
        }

        0b1111_1110 | (!self.is_active as u8)
    }

    pub fn write_status(&mut self, value: u8) {
        if self.data.is_none() {
            return;
        }

        if !self.is_active {
            // Locked.
            return;
        }

        self.is_active = (value & 0b1) == 0;
    }
}
