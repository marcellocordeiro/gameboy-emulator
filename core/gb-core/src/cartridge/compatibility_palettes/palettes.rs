// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

use super::palette::palette_from_id;
use crate::cartridge::compatibility_palettes::CompatibilityPalettes;

/// **Third step**
///
/// From the palette id, get each palette for BG0, OBJ1 and OBJ2.
pub const fn get_palettes_from_id(id: usize) -> CompatibilityPalettes {
    let (obj0, obj1, bg0) = match PALETTE_COMBINATIONS[id] {
        PaletteLookupKind::Normal(obj0, obj1, bg0) => (obj0 * 4, obj1 * 4, bg0 * 4),
        PaletteLookupKind::Raw(obj0, obj1, bg0) => (obj0, obj1, bg0),
    };

    CompatibilityPalettes {
        bg0: palette_from_id(bg0),
        obj0: palette_from_id(obj0),
        obj1: palette_from_id(obj1),
    }
}

/// (OBJ0, OBJ1, BG0)
#[derive(Debug, Clone, Copy)]
enum PaletteLookupKind {
    Normal(usize, usize, usize),
    Raw(usize, usize, usize),
}

use PaletteLookupKind::{Normal, Raw};

#[allow(clippy::erasing_op)]
const PALETTE_COMBINATIONS: [PaletteLookupKind; 51] = [
    Normal(4, 4, 29),
    Normal(18, 18, 18),
    Normal(20, 20, 20),
    Normal(24, 24, 24),
    Normal(9, 9, 9),
    Normal(0, 0, 0),
    Normal(27, 27, 27),
    Normal(5, 5, 5),
    Normal(12, 12, 12),
    Normal(26, 26, 26),
    Normal(16, 8, 8),
    Normal(4, 28, 28),
    Normal(4, 2, 2),
    Normal(3, 4, 4),
    Normal(4, 29, 29),
    Normal(28, 4, 28),
    Normal(2, 17, 2),
    Normal(16, 16, 8),
    Normal(4, 4, 7),
    Normal(4, 4, 18),
    Normal(4, 4, 20),
    Normal(19, 19, 9),
    Raw(4 * 4 - 1, 4 * 4 - 1, 11 * 4),
    Normal(17, 17, 2),
    Normal(4, 4, 2),
    Normal(4, 4, 3),
    Normal(28, 28, 0),
    Normal(3, 3, 0),
    Normal(0, 0, 1),
    Normal(18, 22, 18),
    Normal(20, 22, 20),
    Normal(24, 22, 24),
    Normal(16, 22, 8),
    Normal(17, 4, 13),
    Raw(28 * 4 - 1, 0 * 4, 14 * 4),
    Raw(28 * 4 - 1, 4 * 4, 15 * 4),
    Raw(19 * 4, 23 * 4 - 1, 9 * 4),
    Normal(16, 28, 10),
    Normal(4, 23, 28),
    Normal(17, 22, 2),
    Normal(4, 0, 2),
    Normal(4, 28, 3),
    Normal(28, 3, 0),
    Normal(3, 28, 4),
    Normal(21, 28, 4),
    Normal(3, 28, 0),
    Normal(25, 3, 28),
    Normal(0, 28, 8),
    Normal(4, 3, 28),
    Normal(28, 3, 6),
    Normal(4, 28, 29),
];
