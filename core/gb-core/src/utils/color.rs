#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub alpha: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl Color {
    pub const CGB_SYSTEM_DEFAULT: Self = Self::WHITE_RGB555;
    pub const DMG_GREEN_PALETTE: [Self; 4] = [
        Self::from_u32(0x9A9E3F),
        Self::from_u32(0x496B22),
        Self::from_u32(0x0E450B),
        Self::from_u32(0x1B2A09),
    ];
    pub const DMG_GREY_PALETTE: [Self; 4] = [
        Self::from_u8(0xFF),
        Self::from_u8(0xAA),
        Self::from_u8(0x55),
        Self::from_u8(0x00),
    ];
    pub const DMG_SYSTEM_DEFAULT: Self = Self::DMG_GREEN_PALETTE[0];
    pub const WHITE: Self = Self::new(0xFF, 0xFF, 0xFF);
    pub const WHITE_RGB555: Self = Self::from_rgb555_accurate((0x7F << 10) | (0x7F << 5) | 0x7F);

    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha: 0xFF,
            blue,
            green,
            red,
        }
    }

    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        Self::new(value, value, value)
    }

    #[must_use]
    pub const fn from_u32(value: u32) -> Self {
        let red = (value >> 16) as u8;
        let green = (value >> 8) as u8;
        let blue = value as u8;

        Self::new(red, green, blue)
    }

    #[must_use]
    pub const fn to_rgb555(&self) -> u16 {
        ((self.red as u16) << 10) | ((self.green as u16) << 5) | (self.blue as u16)
    }

    #[must_use]
    pub const fn from_dmg_color_id(color_id: u8) -> Self {
        Self::DMG_GREEN_PALETTE[(color_id & 0b11) as usize]
    }

    #[must_use]
    pub const fn from_dmg_grey_color_id(color_id: u8) -> Self {
        Self::DMG_GREY_PALETTE[(color_id & 0b11) as usize]
    }

    #[must_use]
    pub const fn from_dmg_color_id_with_palette(color_id: u8, dmg_palette: u8) -> Self {
        let resolved_color_id = Self::apply_dmg_palette(color_id, dmg_palette);

        Self::from_dmg_color_id(resolved_color_id)
    }

    #[must_use]
    pub const fn from_rgb555(value: u16) -> Self {
        let red = (value & 0b1_1111) as u8;
        let green = ((value >> 5) & 0b1_1111) as u8;
        let blue = ((value >> 10) & 0b1_1111) as u8;

        Self::new(red, green, blue)
    }

    #[must_use]
    pub const fn from_rgb555_accurate(value: u16) -> Self {
        let raw_red = value & 0b1_1111;
        let raw_green = (value >> 5) & 0b1_1111;
        let raw_blue = (value >> 10) & 0b1_1111;

        let mut adjusted_red = (raw_red * 26) + (raw_green * 4) + (raw_blue * 2);
        let mut adjusted_green = (raw_green * 24) + (raw_blue * 8);
        let mut adjusted_blue = (raw_red * 6) + (raw_green * 4) + (raw_blue * 22);

        adjusted_red >>= 2;
        adjusted_green >>= 2;
        adjusted_blue >>= 2;

        Self::new(
            adjusted_red as u8,
            adjusted_green as u8,
            adjusted_blue as u8,
        )
    }

    #[must_use]
    pub const fn apply_dmg_palette(color_id: u8, palette: u8) -> u8 {
        match color_id & 0b11 {
            0 => palette & 0b11,
            1 => (palette >> 2) & 0b11,
            2 => (palette >> 4) & 0b11,
            3 => (palette >> 6) & 0b11,

            _ => unreachable!(),
        }
    }
}
