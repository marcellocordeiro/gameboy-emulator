use super::{color_ram::ColorRam, Ppu};

impl Ppu {
    pub fn get_bg_cram(&self) -> &ColorRam {
        &self.bg_cram
    }

    pub fn get_obj_cram(&self) -> &ColorRam {
        &self.obj_cram
    }
}
