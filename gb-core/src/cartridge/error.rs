#[derive(Debug)]
pub enum Error {
    InvalidRom,
    UnsupportedMbc { code: u8 },
    UnsupportedRomSize { code: u8 },
    UnsupportedRamSize { code: u8 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::InvalidRom => write!(f, "Invalid ROM"),
            Error::UnsupportedMbc { code } => {
                write!(f, "Unsupported MBC (code = {code:#04X}).")
            }

            Error::UnsupportedRomSize { code } => {
                write!(f, "Unsupported number of ROM banks (code = {code:#04X}).")
            }

            Error::UnsupportedRamSize { code } => {
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
