pub const CGB_FLAG_ADDRESS: usize = 0x0143;

#[derive(Debug, PartialEq, Eq)]
pub enum CgbFlag {
    DmgMode,
    CgbEnhanced,
    CgbOnly,
}

impl From<u8> for CgbFlag {
    fn from(code: u8) -> Self {
        use CgbFlag::*;

        match code {
            0x80 => CgbEnhanced,
            0xC0 => CgbOnly,

            _ => DmgMode,
        }
    }
}

impl std::fmt::Display for CgbFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CgbFlag::*;

        let str = match self {
            DmgMode => "DMG mode.",
            CgbEnhanced => "CGB enhanced.",
            CgbOnly => "CGB only.",
        };

        write!(f, "{str}")
    }
}
