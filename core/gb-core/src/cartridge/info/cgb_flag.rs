pub const CGB_FLAG_ADDRESS: usize = 0x0143;

#[derive(Debug, PartialEq, Eq)]
pub enum CgbFlag {
    DmgMode,
    CgbEnhanced,
    CgbOnly,
}

impl From<u8> for CgbFlag {
    fn from(code: u8) -> Self {
        match code {
            0x80 => Self::CgbEnhanced,
            0xC0 => Self::CgbOnly,

            _ => Self::DmgMode,
        }
    }
}

impl CgbFlag {
    pub fn has_cgb_support(&self) -> bool {
        *self != Self::DmgMode
    }
}

impl std::fmt::Display for CgbFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::DmgMode => "DMG mode",
            Self::CgbEnhanced => "CGB enhanced",
            Self::CgbOnly => "CGB only",
        };

        write!(f, "{str}")
    }
}
