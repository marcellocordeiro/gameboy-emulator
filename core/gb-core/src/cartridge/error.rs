use thiserror::Error;

use super::info::CartridgeType;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid ROM")]
    InvalidRom,

    #[error("Invalid and unsupported MBC (code = {code:#04X}).")]
    InvalidMbcCode { code: u8 },

    #[error("Unsupported MBC (type = {cartridge_type:?}).")]
    UnsupportedMbc { cartridge_type: CartridgeType },

    #[error("Unsupported number of ROM banks (code = {code:#04X}).")]
    UnsupportedRomSize { code: u8 },

    #[error("Unsupported number of RAM banks (code = {code:#04X}).")]
    UnsupportedRamSize { code: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "manual only"]
    fn test_format() {
        let err = Error::UnsupportedRamSize { code: 0xDB };

        println!("{err}");
    }
}
