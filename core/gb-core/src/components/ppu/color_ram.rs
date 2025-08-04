const CRAM_SIZE: usize = 64;

pub struct ColorRam {
    data: [u8; CRAM_SIZE],
}

impl Default for ColorRam {
    fn default() -> Self {
        Self {
            data: [0; CRAM_SIZE],
        }
    }
}

impl ColorRam {
    pub fn read(&self, address: u8) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u8, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn get_color_rgb555(&self, palette_number: u8, color_id: u8) -> u16 {
        let palette_address = palette_number * 8;
        let color_index = color_id * 2;
        let base_address = palette_address + color_index;

        let lo = self.read(base_address) as u16;
        let hi = self.read(base_address + 1) as u16;

        (hi << 8) | lo
    }

    pub fn set_palette(&mut self, palette_number: u8, colors: [u16; 4]) {
        let palette_address = palette_number * 8;

        for (color_index, color) in colors.into_iter().enumerate() {
            if color > 0x7FFF {
                log::error!(
                    "Color value is higher than 0x7FFF: [{palette_number}][{color_index}] = {color:#06X}."
                );
            }

            let lo = (color & 0xFF) as u8;
            let hi = (color >> 8) as u8;

            let offset = (color_index * 2) as u8;
            let base_address = palette_address + offset;

            self.write(base_address, lo);
            self.write(base_address + 1, hi);
        }
    }
}
