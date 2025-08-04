use arrayvec::ArrayVec;

use super::{Ppu, lcd_status::StatusMode, sprite::SpriteObject};

const OAM_SIZE: usize = 0xA0;

// Sprite Attribute Table (OAM: Object Attribute Memory).
pub struct Oam {
    data: [u8; OAM_SIZE],

    sprite_buffer: ArrayVec<SpriteObject, 10>,
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            data: [0; OAM_SIZE],
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

    /// Returns the sprite buffer sorted by the X coordinate.
    ///
    /// This is the default behavior in the DMG.
    pub fn get_sprites_in_line_by_coordinate(&mut self, ly: u8, obj_height: u8) -> &[SpriteObject] {
        self.update_sprite_buffer(ly, obj_height);

        // Increasing priority by the x coordinate.
        self.sprite_buffer.sort_by(|a, b| a.x.cmp(&b.x));

        &self.sprite_buffer
    }

    /// Returns the sprite buffer sorted by the OAM order.
    ///
    /// This is the default behavior in the CGB, but the bootrom can change this
    /// by modifying the OPRI (0xFF6C) register.
    ///
    /// Warning: CGB model only.
    pub fn get_sprites_in_line_by_oam(&mut self, ly: u8, obj_height: u8) -> &[SpriteObject] {
        self.update_sprite_buffer(ly, obj_height);

        &self.sprite_buffer
    }

    fn update_sprite_buffer(&mut self, ly: u8, obj_height: u8) {
        self.sprite_buffer = self
            .data
            .chunks_exact(4)
            .filter_map(|chunk| {
                let y = chunk[0].wrapping_sub(16);
                let x = chunk[1].wrapping_sub(8);
                let tile_index = chunk[2];
                let flags = chunk[3];

                if ly.wrapping_sub(y) < obj_height {
                    Some(SpriteObject::from_bytes(y, x, tile_index, flags))
                } else {
                    None
                }
            })
            .take(10)
            .collect();
    }
}

impl Ppu {
    #[must_use]
    pub fn read_oam(&self, address: u16) -> u8 {
        if self.oam_dma.is_active() {
            return 0xFF;
        }

        if self.mode == StatusMode::OamScan || self.mode == StatusMode::Drawing {
            return 0xFF;
        }

        self.oam.read(address)
    }

    pub fn write_oam(&mut self, address: u16, value: u8) {
        if self.oam_dma.is_active() {
            return;
        }

        if self.mode == StatusMode::OamScan || self.mode == StatusMode::Drawing {
            return;
        }

        self.oam.write(address, value);
    }
}
