use super::info::CartridgeType;

#[derive(Debug)]
pub enum Error {
    InvalidRom,
    InvalidMbcCode { code: u8 },
    UnsupportedMbc { cartridge_type: CartridgeType },
    UnsupportedRomSize { code: u8 },
    UnsupportedRamSize { code: u8 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match *self {
            InvalidRom => write!(f, "Invalid ROM"),

            InvalidMbcCode { code } => {
                write!(f, "Invalid and unsupported MBC (code = {code:#04X}).")
            }

            UnsupportedMbc { cartridge_type } => {
                write!(f, "Unsupported MBC (type = {cartridge_type:?}).")
            }

            UnsupportedRomSize { code } => {
                write!(f, "Unsupported number of ROM banks (code = {code:#04X}).")
            }

            UnsupportedRamSize { code } => {
                write!(f, "Unsupported number of RAM banks (code = {code:#04X}).")
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_format() {
        let err = Error::UnsupportedRamSize { code: 0xDB };

        println!("{err}");
    }
}
