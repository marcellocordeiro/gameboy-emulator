use std::sync::Arc;

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
    pub fn insert(&mut self, device_model: DeviceModel, bootrom: Arc<Box<[u8]>>) {
        match device_model {
            DeviceModel::Dmg => assert_eq!(bootrom.len(), DMG_BOOTROM_SIZE),
            DeviceModel::Cgb => assert_eq!(bootrom.len(), CGB_BOOTROM_SIZE),
        };

        self.data = Some(bootrom);
        self.is_active = true;
    }

    pub fn is_loaded(&self) -> bool {
        self.data.is_some()
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
