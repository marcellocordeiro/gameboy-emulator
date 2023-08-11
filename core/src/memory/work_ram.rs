const WRAM_SIZE: usize = 0x8000;

pub struct WorkRam {
    data: [u8; WRAM_SIZE],
}

impl Default for WorkRam {
    fn default() -> Self {
        Self {
            data: [0; WRAM_SIZE], // can't default this :(
        }
    }
}

impl WorkRam {
    // 0xC000 ~ 0xDFFF

    // 0xC000 ~ 0xCFFF: bank 0.
    // 0xD000 ~ 0xDFFF: In CGB mode, switchable bank 1~7.
    // 0xE000 ~ 0xFDFF: ECHO RAM (prohibited area, but mirrors 0xC000 ~ 0xDDFF).

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xC000..=0xCFFF => self.data[address as usize - 0xC000],
            0xD000..=0xDFFF => self.data[address as usize - 0xC000], //  TODO: CGB. Only DMG mode for now.

            // ECHO RAM.
            0xE000..=0xEFFF => self.data[address as usize - 0xE000],
            0xF000..=0xFDFF => self.data[address as usize - 0xE000], //  TODO: CGB. Only DMG mode for now.

            _ => unreachable!("[work_ram.rs] Read out of bounds: {:#06x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xC000..=0xCFFF => self.data[address as usize - 0xC000] = value,
            0xD000..=0xDFFF => self.data[address as usize - 0xC000] = value, //  TODO: CGB. Only DMG mode for now.

            // ECHO RAM.
            0xE000..=0xEFFF => self.data[address as usize - 0xE000] = value,
            0xF000..=0xFDFF => self.data[address as usize - 0xE000] = value, //  TODO: CGB. Only DMG mode for now.

            _ => unreachable!(
                "[work_ram.rs] Read out of bounds: ({:#06x}) = {:#04x}",
                address, value
            ),
        }
    }
}
