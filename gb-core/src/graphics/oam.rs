use super::{SpriteFlags, SpriteObject};

const OAM_SIZE: usize = 0xA0;

// Sprite Attribute Table (OAM: Object Attribute Memory).
pub struct Oam {
    data: [u8; OAM_SIZE],

    pub(super) sprite_buffer: Vec<SpriteObject>,
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            data: [0; OAM_SIZE], // can't default this :(
            sprite_buffer: Vec::default(),
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

    pub(super) fn get_sprites_in_line(&mut self, ly: u8, obj_height: u8) -> &[SpriteObject] {
        self.sprite_buffer.clear();

        for i in 0..40_usize {
            let base_address = i * 4;

            let y = self.data[base_address].wrapping_sub(16);
            let x = self.data[base_address + 1].wrapping_sub(8);
            let tile_index = self.data[base_address + 2];
            let flags = SpriteFlags::from_bits_truncate(self.data[base_address + 3]);

            if ly.wrapping_sub(y) < obj_height {
                self.sprite_buffer.push(SpriteObject {
                    y,
                    x,
                    tile_index,
                    flags,
                });
            }

            if self.sprite_buffer.len() == 10 {
                break;
            }
        }

        // Increasing priority.
        self.sprite_buffer.sort_by(|&a, &b| a.x.cmp(&b.x));

        &self.sprite_buffer
    }
}
