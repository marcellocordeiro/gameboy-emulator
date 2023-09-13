use super::Graphics;

impl Graphics {
    pub fn get_bg_palette_ram(&self) -> &[u8; 64] {
        &self.bg_palette_ram
    }

    pub fn get_obj_palette_ram(&self) -> &[u8; 64] {
        &self.obj_palette_ram
    }
}
