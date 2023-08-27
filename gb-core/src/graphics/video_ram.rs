use super::{lcd_status::StatusMode, Graphics};

#[cfg(not(feature = "cgb-mode"))]
const VRAM_BANKS: usize = 1;

#[cfg(feature = "cgb-mode")]
const VRAM_BANKS: usize = 2;

const VRAM_BANK_SIZE: usize = 0x2000;
const VRAM_SIZE: usize = VRAM_BANKS * VRAM_BANK_SIZE; // DMG: 8192 (0x2000) / CGB: 16384 (0x4000)

pub struct VideoRam {
    data: [u8; VRAM_SIZE],
    vbk: u8,
}

impl Default for VideoRam {
    fn default() -> Self {
        Self {
            data: [0; VRAM_SIZE], // can't default this :(
            vbk: 0,
        }
    }
}

impl VideoRam {
    // 0x8000 ~ 0x9FFF

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize - 0x8000 + self.bank_offset()]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0x8000 + self.bank_offset()] = value;
    }

    pub fn read_vbk(&self) -> u8 {
        if cfg!(feature = "cgb-mode") {
            0b1111_1110 | self.vbk
        } else {
            0xFF
        }
    }

    pub fn write_vbk(&mut self, value: u8) {
        if cfg!(feature = "cgb-mode") {
            self.vbk = value & 0b1
        }
    }

    fn bank_offset(&self) -> usize {
        VRAM_BANK_SIZE * (self.vbk as usize)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sanity() {
        let vram = VideoRam::default();

        if cfg!(feature = "cgb-mode") {
            assert_eq!(vram.data.len(), 0x4000);
        } else {
            assert_eq!(vram.data.len(), 0x2000);
        }
    }

    #[test]
    #[cfg(feature = "cgb-mode")]
    fn test_read_banks() {
        let mut vram = VideoRam::default();

        let chunks = vram.data.chunks_exact_mut(VRAM_BANK_SIZE);

        assert_eq!(chunks.len(), VRAM_BANKS); // 2 banks

        for (bank, chunk) in chunks.enumerate() {
            let chunk_iter = chunk.iter_mut();

            assert_eq!(chunk_iter.len(), VRAM_BANK_SIZE);

            for element in chunk_iter {
                *element = bank as u8;
            }
        }

        verify_banks(&mut vram);
    }

    #[test]
    #[cfg(feature = "cgb-mode")]
    fn test_write_banks() {
        let mut vram = VideoRam::default();

        // Bank 0
        for address in 0x8000..=0x9FFF {
            vram.write(address, 0);
        }

        // Bank 1
        vram.write_vbk(0b1111_1110 | 0b1);
        assert_eq!(vram.vbk, 0b1, "Should ignore bits 1-7");

        for address in 0x8000..=0x9FFF {
            vram.write(address, 1);
        }

        verify_banks(&mut vram);
    }

    #[cfg(feature = "cgb-mode")]
    fn verify_banks(vram: &mut VideoRam) {
        // Bank 0
        vram.write_vbk(0b1111_1110 | 0b0);
        assert_eq!(vram.vbk, 0b0, "Should ignore bits 1-7");

        for address in 0x8000..=0x9FFF {
            assert_eq!(vram.read(address), 0);
        }

        // Bank 1
        vram.write_vbk(0b1111_1110 | 0b1);
        assert_eq!(vram.vbk, 0b1, "Should ignore bits 1-7");

        for address in 0x8000..=0x9FFF {
            assert_eq!(vram.read(address), 1);
        }
    }
}