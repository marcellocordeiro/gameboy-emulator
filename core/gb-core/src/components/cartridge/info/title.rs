use super::header::Header;
use crate::components::cartridge::error::CartridgeError;

pub const TITLE_ADDRESS_BEGIN: usize = 0x0134;
pub const TITLE_ADDRESS_END: usize = 0x0143;

pub const TITLE_SIZE: usize = TITLE_ADDRESS_END - TITLE_ADDRESS_BEGIN + 1;

pub struct Title {
    bytes: [u8; TITLE_SIZE],
    string: String,
}

impl Title {
    pub fn from_header(header: &Header) -> Result<Self, CartridgeError> {
        let bytes: [u8; TITLE_SIZE] = header[TITLE_ADDRESS_BEGIN..=TITLE_ADDRESS_END]
            .try_into()
            .map_err(|_| CartridgeError::InvalidRom)?;

        let string = String::from_utf8_lossy(&bytes)
            .trim()
            .chars()
            .filter(char::is_ascii)
            .collect();

        Ok(Self { bytes, string })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    #[must_use]
    pub fn as_string(&self) -> &String {
        &self.string
    }

    #[must_use]
    pub fn checksum(&self) -> u8 {
        self.as_bytes()
            .iter()
            .fold(0, |acc, x| acc.wrapping_add(*x))
    }
}
