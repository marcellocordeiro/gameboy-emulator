use crate::cartridge::error::Error as CartridgeError;

pub const CGB_FLAG_ADDRESS: usize = 0x0143;

#[derive(Debug, PartialEq, Eq)]
pub enum CgbFlag {
    DmgMode,
    CgbEnhanced,
    CgbOnly,
}

impl CgbFlag {
    pub fn with_rom(rom: &[u8]) -> Result<Self, CartridgeError> {
        let code = *rom
            .get(CGB_FLAG_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        Ok(Self::with_code(code))
    }

    pub fn with_code(code: u8) -> Self {
        match code {
            0x80 => Self::CgbEnhanced,
            0xC0 => Self::CgbOnly,

            _ => Self::DmgMode,
        }
    }

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
