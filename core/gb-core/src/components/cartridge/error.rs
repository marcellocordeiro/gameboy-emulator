use thiserror::Error;

use super::info::mbc_type::MbcType;

#[derive(Debug, Error)]
pub enum CartridgeError {
    #[error("Invalid ROM.")]
    InvalidRom,

    #[error("Invalid and unsupported MBC (code = {code:#04X}).")]
    InvalidMbcCode { code: u8 },

    #[error("Unsupported MBC (type = {cartridge_type:?}).")]
    UnsupportedMbc { cartridge_type: MbcType },

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
        let err = CartridgeError::UnsupportedRamSize { code: 0xDB };

        println!("{err}");
    }
}
