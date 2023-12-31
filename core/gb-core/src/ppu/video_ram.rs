use std::ops::RangeInclusive;

use super::{lcd_status::StatusMode, Ppu};
use crate::{
    constants::{TileDataFrame, TILES_PER_LINE, TILE_DATA_FRAME_WIDTH},
    utils::{
        color::Color,
        macros::{device_is_cgb, in_cgb_mode},
    },
};

#[cfg(not(feature = "cgb"))]
const VRAM_BANKS: usize = 1;

#[cfg(feature = "cgb")]
const VRAM_BANKS: usize = 2;

const VRAM_BANK_SIZE: usize = 0x2000;
const VRAM_SIZE: usize = VRAM_BANKS * VRAM_BANK_SIZE; // DMG: 8192 (0x2000) / CGB: 16384 (0x4000)

pub struct VideoRam {
    data: [u8; VRAM_SIZE],
    vbk: u8,

    cgb_mode: bool,
}

impl Default for VideoRam {
    fn default() -> Self {
        Self {
            data: [0; VRAM_SIZE], // can't default this :(
            vbk: 0,
            cgb_mode: false,
        }
    }
}

impl VideoRam {
    // 0x8000 ~ 0x9FFF

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    pub fn draw_tile_data_0_into_frame(&self, frame: &mut TileDataFrame) {
        const TILE_DATA_0_START: usize = 0;
        const TILE_DATA_0_END: usize = 0x97FF - 0x8000;

        let range = TILE_DATA_0_START..=TILE_DATA_0_END;

        self.draw_tile_data_range_into_frame(range, frame, 0);
    }

    #[cfg(feature = "cgb")]
    pub fn draw_tile_data_1_into_frame(&self, frame: &mut TileDataFrame) {
        const TILE_DATA_1_START: usize = VRAM_BANK_SIZE;
        const TILE_DATA_1_END: usize = (0x97FF - 0x8000) + VRAM_BANK_SIZE;

        let range = TILE_DATA_1_START..=TILE_DATA_1_END;

        self.draw_tile_data_range_into_frame(range, frame, TILE_DATA_FRAME_WIDTH / 2);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize - 0x8000 + self.bank_offset()]
    }

    pub fn read_bank_0(&self, address: u16) -> u8 {
        self.data[address as usize - 0x8000]
    }

    #[cfg(feature = "cgb")]
    pub fn read_bank_1(&self, address: u16) -> u8 {
        self.data[address as usize - 0x8000 + VRAM_BANK_SIZE]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0x8000 + self.bank_offset()] = value;
    }

    pub fn read_vbk(&self) -> u8 {
        if !device_is_cgb!() {
            return 0xFF;
        }

        0b1111_1110 | self.vbk
    }

    pub fn write_vbk(&mut self, value: u8) {
        if !in_cgb_mode!(self) {
            return;
        }

        self.vbk = value & 0b1;
    }

    fn bank_offset(&self) -> usize {
        if !in_cgb_mode!(self) {
            return 0;
        }

        VRAM_BANK_SIZE * (self.vbk as usize)
    }

    fn draw_tile_data_range_into_frame(
        &self,
        range: RangeInclusive<usize>,
        frame: &mut TileDataFrame,
        frame_column_offset: usize,
    ) {
        const TILE_SIZE: usize = 16;

        let tile_data_chunks = self.data[range].chunks_exact(TILE_SIZE);

        for (tile_index, tile) in tile_data_chunks.into_iter().enumerate() {
            let tile_base_x = (tile_index % TILES_PER_LINE) * 8;
            let tile_base_y = (tile_index / TILES_PER_LINE) * 8;

            let tile_data_lo_hi_chunks = tile.chunks_exact(2);

            for (byte_line, tile_data_lo_hi) in tile_data_lo_hi_chunks.into_iter().enumerate() {
                let data_lo = tile_data_lo_hi[0];
                let data_hi = tile_data_lo_hi[1];

                for bit in 0..=7 {
                    let color_id = {
                        let lo = ((data_lo << bit) >> 7) & 0b1;
                        let hi = ((data_hi << bit) >> 7) & 0b1;

                        (hi << 1) | lo
                    };

                    let pixel = Color::from_dmg_color_id(color_id);

                    let mapped_address = {
                        let mapped_x = tile_base_x + bit;
                        let mapped_y = tile_base_y + byte_line;

                        (mapped_y * TILE_DATA_FRAME_WIDTH) + mapped_x + frame_column_offset
                    };

                    frame[mapped_address * 4] = pixel.red;
                    frame[(mapped_address * 4) + 1] = pixel.green;
                    frame[(mapped_address * 4) + 2] = pixel.blue;
                    frame[(mapped_address * 4) + 3] = 0xFF;
                }
            }
        }
    }
}

impl Ppu {
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

        if cfg!(feature = "cgb") {
            assert_eq!(vram.data.len(), 0x4000);
        } else {
            assert_eq!(vram.data.len(), 0x2000);
        }
    }

    #[test]
    #[cfg(feature = "cgb")]
    fn test_read_banks() {
        let mut vram = VideoRam::default();
        vram.set_cgb_mode(true);

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
    #[cfg(feature = "cgb")]
    fn test_write_banks() {
        let mut vram = VideoRam::default();
        vram.set_cgb_mode(true);

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

    #[cfg(feature = "cgb")]
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
