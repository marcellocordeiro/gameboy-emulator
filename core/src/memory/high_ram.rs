const HRAM_SIZE: usize = 0x80;

pub struct HighRam {
    data: [u8; HRAM_SIZE],
}

impl Default for HighRam {
    fn default() -> Self {
        Self {
            data: [0; HRAM_SIZE],
        }
    }
}

impl HighRam {
    // 0xFF80 ~ 0xFFFE

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize - 0xFF80]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0xFF80] = value;
    }
}
