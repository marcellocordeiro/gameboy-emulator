const VRAM_SIZE: usize = 0x2000;

pub struct VideoRam {
    data: [u8; VRAM_SIZE],
}

impl Default for VideoRam {
    fn default() -> Self {
        Self {
            data: [0; VRAM_SIZE], // can't default this :(
        }
    }
}

impl VideoRam {
    // 0x8000 ~ 0x9FFF

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize - 0x8000]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0x8000] = value;
    }
}
