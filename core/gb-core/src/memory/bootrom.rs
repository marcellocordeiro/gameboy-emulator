#[cfg(all(feature = "bootrom", not(feature = "cgb")))]
/// DMG
///
/// Size: 256 (0x100)
const BOOTROM_SIZE: usize = 0x100;

#[cfg(all(feature = "bootrom", feature = "cgb"))]
/// CGB
///
/// Size: 256 + 256 + 1792  = 2304 (0x900)
///
/// Note: the extra 256 bytes in the middle is mapped to the cartridge header until 0x200.
const BOOTROM_SIZE: usize = 0x100 + 0x100 + 0x700;

#[cfg(feature = "bootrom")]
pub struct Bootrom {
    data: [u8; BOOTROM_SIZE],
    is_active: bool,
}

#[cfg(feature = "bootrom")]
impl Default for Bootrom {
    fn default() -> Self {
        #[cfg(not(feature = "cgb"))]
        let data = include_bytes!("../../../../roms/dmg_boot.bin");

        #[cfg(feature = "cgb")]
        let data = include_bytes!("../../../../roms/cgb_boot.bin");

        Self {
            data: *data,
            is_active: true,
        }
    }
}

#[cfg(feature = "bootrom")]
impl Bootrom {
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn disable(&mut self) {
        self.is_active = false;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_status(&self) -> u8 {
        0b1111_1110 | (!self.is_active as u8)
    }

    pub fn write_status(&mut self, value: u8) {
        if !self.is_active {
            // Locked.
            return;
        }

        self.is_active = (value & 0b1) == 0;
    }
}

// Stubs for when bootrom support is disabled.

#[cfg(not(feature = "bootrom"))]
#[derive(Debug, Default)]
/// Bootrom stubs
pub struct Bootrom {}

#[cfg(not(feature = "bootrom"))]
impl Bootrom {
    pub fn is_active(&self) -> bool {
        false
    }

    pub fn disable(&self) {}

    pub fn read(&self, _address: u16) -> u8 {
        0xFF
    }

    pub fn read_status(&self) -> u8 {
        0xFF
    }

    pub fn write_status(&self, _value: u8) {}
}
