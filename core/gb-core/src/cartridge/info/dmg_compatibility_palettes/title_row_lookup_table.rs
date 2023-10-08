// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

pub const TITLE_ROW_LOOKUP_TABLE: [&str; 3] = ["BEFAARBEKEK R-", "URAR INAILICE ", "R"];

pub fn find_row(column: usize, byte: u8) -> usize {
    if let Some(ch) = TITLE_ROW_LOOKUP_TABLE[0].as_bytes().get(column) {
        if *ch == byte {
            return 0;
        }
    }

    if let Some(ch) = TITLE_ROW_LOOKUP_TABLE[1].as_bytes().get(column) {
        if *ch == byte {
            return 1;
        }
    }

    if let Some(ch) = TITLE_ROW_LOOKUP_TABLE[2].as_bytes().get(column) {
        if *ch == byte {
            return 2;
        }
    }

    0
}
