// Taken from
// 1. https://gbdev.io/pandocs/Power_Up_Sequence.html#compatibility-palettes
// 2. https://github.com/LIJI32/SameBoy/blob/master/BootROMs/cgb_boot.asm

use super::{palette_lookup_table::from_index_start, DmgCompatibilityPalettes};

/// (OBJ0, OBJ1, BG0)
#[derive(Debug, Clone, Copy)]
pub enum PaletteLookupKind {
    Normal(usize, usize, usize),
    Raw(usize, usize, usize),
}

impl PaletteLookupKind {
    pub const fn into_palettes(self) -> DmgCompatibilityPalettes {
        let (obj0, obj1, bg0) = match self {
            Self::Normal(obj0, obj1, bg0) => (obj0 * 4, obj1 * 4, bg0 * 4),
            Self::Raw(obj0, obj1, bg0) => (obj0, obj1, bg0),
        };

        DmgCompatibilityPalettes {
            bg0: from_index_start(bg0),
            obj0: from_index_start(obj0),
            obj1: from_index_start(obj1),
        }
    }
}
