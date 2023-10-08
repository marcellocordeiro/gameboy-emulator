// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

use super::{licensee_code::LicenseeCode, title::Title};

use self::{
    palette_combinations::PALETTE_COMBINATIONS, palette_id_lookup_table::PALETTE_ID_LOOKUP_TABLE,
    title_checksum_lookup_table::TITLE_CHECKSUM_LOOKUP_TABLE,
};

pub struct DmgCompatibilityPalettes {
    pub bg0: [u16; 4],
    pub obj0: [u16; 4],
    pub obj1: [u16; 4],
}

impl DmgCompatibilityPalettes {
    pub const DEFAULT: Self = PALETTE_COMBINATIONS[0].into_palettes();

    pub fn with_header_info(licensee_code: &LicenseeCode, title: &Title) -> Self {
        match licensee_code.old() {
            0x33 => {
                if !(licensee_code.new_as_string() == "01"
                    || u16::from_le_bytes(*licensee_code.new_as_bytes()) == 0x01)
                {
                    return Self::DEFAULT;
                }
            }

            0x01 => (),

            _ => return Self::DEFAULT,
        };

        let title_checksum = title.checksum();

        let Some(checksum_index) = TITLE_CHECKSUM_LOOKUP_TABLE
            .into_iter()
            .position(|x| x == title_checksum)
        else {
            return Self::DEFAULT;
        };

        let palette_id = if checksum_index <= 64 {
            PALETTE_ID_LOOKUP_TABLE[checksum_index]
        } else {
            let fourth_byte = title.as_bytes()[3];
            let row = title_row_lookup_table::find_row(checksum_index - 65, fourth_byte);
            let index = checksum_index + (14 * row);

            PALETTE_ID_LOOKUP_TABLE[index]
        };

        PALETTE_COMBINATIONS[palette_id].into_palettes()
    }
}

mod palette_combinations;
mod palette_id_lookup_table;
mod palette_lookup_table;
mod palette_lookup_type;
mod title_checksum_lookup_table;
mod title_row_lookup_table;
