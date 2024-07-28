// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

use crate::components::cartridge::info::title::Title;

/// **First step**
///
/// Used to obtain the index of the palette id.
pub fn get_palette_id_index(title: &Title) -> Option<usize> {
    let title_checksum = title.checksum();

    let checksum_index = TITLE_CHECKSUM
        .into_iter()
        .position(|x| x == title_checksum)?;

    if checksum_index <= 64 {
        Some(checksum_index)
    } else {
        let fourth_byte = title.as_bytes()[3];
        let row = find_title_row(checksum_index - 65, fourth_byte);
        let index = checksum_index + (14 * row);

        Some(index)
    }
}

/// The index will be the index that corresponds to the value of the title checksum in this table.
const TITLE_CHECKSUM: [u8; 79] = [
    0x00, 0x88, 0x16, 0x36, 0xD1, 0xDB, 0xF2, 0x3C, 0x8C, 0x92, 0x3D, 0x5C, 0x58, 0xC9, 0x3E, 0x70,
    0x1D, 0x59, 0x69, 0x19, 0x35, 0xA8, 0x14, 0xAA, 0x75, 0x95, 0x99, 0x34, 0x6F, 0x15, 0xFF, 0x97,
    0x4B, 0x90, 0x17, 0x10, 0x39, 0xF7, 0xF6, 0xA2, 0x49, 0x4E, 0x43, 0x68, 0xE0, 0x8B, 0xF0, 0xCE,
    0x0C, 0x29, 0xE8, 0xB7, 0x86, 0x9A, 0x52, 0x01, 0x9D, 0x71, 0x9C, 0xBD, 0x5D, 0x6D, 0x67, 0x3F,
    0x6B, // <-- 64
    // Ambiguous. Refer to the fourth byte in the title if the index is >64.
    0xB3, 0x46, 0x28, 0xA5, 0xC6, 0xD3, 0x27, 0x61, 0x18, 0x66, 0x6A, 0xBF, 0x0D, 0xF4,
];

/// If the index of the checksum is <= 64, don't use this.
/// If > 64, this will return the index of the palette id lookup table.
fn find_title_row(column: usize, fourth_byte: u8) -> usize {
    const TITLE_ROW_LOOKUP_TABLE: [&str; 3] = ["BEFAARBEKEK R-", "URAR INAILICE ", "R"];

    if let Some(ch) = TITLE_ROW_LOOKUP_TABLE[0].as_bytes().get(column) {
        if *ch == fourth_byte {
            return 0;
        }
    }

    if let Some(ch) = TITLE_ROW_LOOKUP_TABLE[1].as_bytes().get(column) {
        if *ch == fourth_byte {
            return 1;
        }
    }

    if let Some(ch) = TITLE_ROW_LOOKUP_TABLE[2].as_bytes().get(column) {
        if *ch == fourth_byte {
            return 2;
        }
    }

    0
}
