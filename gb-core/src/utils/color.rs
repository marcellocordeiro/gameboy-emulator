#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const DMG_PALETTE: [u8; 4] = [0xFF, 0xAA, 0x55, 0x00];

    pub fn new(red: u8, blue: u8, green: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn to_rgb555(&self) -> u16 {
        ((self.red as u16) << 10) | ((self.green as u16) << 5) | (self.blue as u16)
    }

    pub fn white() -> Self {
        Self::new(0xFF, 0xFF, 0xFF, 0xFF)
    }

    pub fn from_dmg_color_id(color_id: u8) -> Self {
        let color = Self::DMG_PALETTE[(color_id & 0b11) as usize];

        let red = color;
        let green = color;
        let blue = color;

        Self::new(red, blue, green, 0xFF)
    }

    pub fn from_dmg_color_id_with_palette(color_id: u8, dmg_palette: u8) -> Self {
        let resolved_color_id = Self::apply_dmg_palette(color_id, dmg_palette);

        Self::from_dmg_color_id(resolved_color_id)
    }

    pub fn from_rgb555_u16_raw(value: u16) -> Self {
        let red = value & 0b1_1111;
        let green = (value >> 5) & 0b1_1111;
        let blue = (value >> 10) & 0b1_1111;

        Self::new(red as u8, blue as u8, green as u8, 0xFF)
    }

    pub fn from_rgb555_u16_to_rgba8888(value: u16) -> Self {
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
            0xFF,
        )
    }

    #[allow(clippy::identity_op)]
    fn apply_dmg_palette(color_id: u8, palette: u8) -> u8 {
        match color_id & 0b11 {
            0 => (palette >> 0) & 0b11,
            1 => (palette >> 2) & 0b11,
            2 => (palette >> 4) & 0b11,
            3 => (palette >> 6) & 0b11,

            _ => unreachable!(),
        }
    }
}
