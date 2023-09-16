use arrayvec::ArrayVec;

use super::{lcd_status::StatusMode, sprite::SpriteObject, Graphics};

const OAM_SIZE: usize = 0xA0;

// Sprite Attribute Table (OAM: Object Attribute Memory).
pub struct Oam {
    data: [u8; OAM_SIZE],

    sprite_buffer: ArrayVec<SpriteObject, 10>,
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            data: [0; OAM_SIZE], // can't default this :(
            sprite_buffer: ArrayVec::default(),
        }
    }
}

impl Oam {
    // 0xFE00 ~ 0xFE9F

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize - 0xFE00]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - 0xFE00] = value;
    }

    pub fn get_sprites_in_line(&mut self, ly: u8, obj_height: u8) -> &[SpriteObject] {
        self.sprite_buffer.clear();

        for chunk in self.data.chunks_exact(4) {
            let y = chunk[0].wrapping_sub(16);
            let x = chunk[1].wrapping_sub(8);
            let tile_index = chunk[2];
            let flags = chunk[3];

            if ly.wrapping_sub(y) < obj_height {
                let element = SpriteObject::from_bytes(y, x, tile_index, flags);

                self.sprite_buffer.push(element);
            }

            if self.sprite_buffer.is_full() {
                break;
            }
        }

        // Increasing priority.
        self.sprite_buffer.sort_by(|a, b| a.x.cmp(&b.x));

        &self.sprite_buffer
    }
}

impl Graphics {
    pub fn read_oam(&self, address: u16) -> u8 {
        if self.lcdc.get_lcd_enable()
            && (self.mode == StatusMode::OamScan || self.mode == StatusMode::Drawing)
        {
            return 0xFF;
        }

        self.oam.read(address)
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        if self.lcdc.get_lcd_enable()
            && (self.mode == StatusMode::OamScan || self.mode == StatusMode::Drawing)
        {
            return;
        }

        self.oam.write(address, value);
    }
}
