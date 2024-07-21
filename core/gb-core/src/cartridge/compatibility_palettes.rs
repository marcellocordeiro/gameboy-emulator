// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

use palette_id::get_palette_id;
use palette_id_index::get_palette_id_index;
use palettes::get_palettes_from_id;

use crate::cartridge::{licensee_code::LicenseeCode, title::Title};

pub struct CompatibilityPalettes {
    pub bg0: [u16; 4],
    pub obj0: [u16; 4],
    pub obj1: [u16; 4],
}

impl CompatibilityPalettes {
    pub const DEFAULT: Self = get_palettes_from_id(0);

    pub fn from_header_info(licensee_code: &LicenseeCode, title: &Title) -> Self {
        if !licensee_code.is_nintendo() {
            return Self::DEFAULT;
        }

        let Some(palette_id_index) = get_palette_id_index(title) else {
            return Self::DEFAULT;
        };

        let palette_id = get_palette_id(palette_id_index);

        get_palettes_from_id(palette_id)
    }
}

mod palette;
mod palette_id;
mod palette_id_index;
mod palettes;
