use super::{lcd_status::StatusMode, Graphics};

#[cfg(feature = "cgb-mode")]
const VRAM_BANKS: usize = 2;

#[cfg(not(feature = "cgb-mode"))]
const VRAM_BANKS: usize = 1;

const VRAM_BANK_SIZE: usize = 0x2000;
const VRAM_SIZE: usize = VRAM_BANKS * VRAM_BANK_SIZE; // DMG: 8192 (0x2000) / CGB: 16384 (0x4000)

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

impl Graphics {
    pub fn read_vram(&self, address: u16) -> u8 {
        if self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing {
            return 0xFF;
        }

        self.vram.read(address)
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        if self.lcdc.get_lcd_enable() && self.mode == StatusMode::Drawing {
            return;
        }

        self.vram.write(address, value);
    }
}
