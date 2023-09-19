#[derive(Debug, Default)]
pub struct SpriteObjectFlags {
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub obp1_selected: bool, // DMG and DMG compat only.
    pub in_bank_1: bool,     // CGB only.
    pub palette_number: u8,  // CGB only.
}

impl SpriteObjectFlags {
    pub fn from_byte(flags: u8) -> Self {
        let priority = (flags & 0b1000_0000) != 0;
        let y_flip = (flags & 0b0100_0000) != 0;
        let x_flip = (flags & 0b0010_0000) != 0;
        let obp1_selected = (flags & 0b0001_0000) != 0;
        let in_bank_1 = (flags & 0b0000_1000) != 0;
        let palette_number = flags & 0b0000_0111;

        Self {
            priority,
            y_flip,
            x_flip,
            obp1_selected,
            in_bank_1,
            palette_number,
        }
    }
}

#[derive(Debug, Default)]
pub struct SpriteObject {
    pub y: u8, // Vertical position + 16 (already adjusted).
    pub x: u8, // Horizontal position + 8 (already adjusted).
    pub tile_index: u8,

    pub flags: SpriteObjectFlags,
}

impl SpriteObject {
    pub fn from_bytes(y: u8, x: u8, tile_index: u8, flags: u8) -> Self {
        let flags = SpriteObjectFlags::from_byte(flags);

        Self {
            y,
            x,
            tile_index,
            flags,
        }
    }
}
