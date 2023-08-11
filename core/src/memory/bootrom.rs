pub struct Bootrom {
    data: [u8; 0x100],

    is_active: bool,
}

#[cfg(feature = "bootrom")]
impl Default for Bootrom {
    fn default() -> Self {
        let data = include_bytes!("../../../roms/bootrom.gb");

        Self {
            data: *data,
            is_active: true,
        }
    }
}

#[cfg(not(feature = "bootrom"))]
impl Default for Bootrom {
    fn default() -> Self {
        Self {
            data: [0; 0x100], // can't default this :(
            is_active: false,
        }
    }
}

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
        (!self.is_active as u8) | 0xFE
    }

    pub fn write_status(&mut self, value: u8) {
        if !self.is_active {
            // Locked.
            return;
        }

        self.is_active = (value & 0b1) == 0;
    }
}
