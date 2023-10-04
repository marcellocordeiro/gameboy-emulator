use crate::cartridge::error::Error as CartridgeError;

pub const TITLE_ADDRESS_BEGIN: usize = 0x0134;
pub const TITLE_ADDRESS_END: usize = 0x0143;

pub struct Title {
    bytes: [u8; 16],
    string: String,
}

impl Title {
    pub fn with_rom(rom: &[u8]) -> Result<Self, CartridgeError> {
        let bytes = rom
            .get(TITLE_ADDRESS_BEGIN..=TITLE_ADDRESS_END)
            .ok_or(CartridgeError::InvalidRom)?;

        let string = String::from_utf8_lossy(bytes)
            .trim()
            .chars()
            .filter(char::is_ascii)
            .collect();

        Ok(Self {
            bytes: bytes
                .try_into()
                .map_err(|_err| CartridgeError::InvalidRom)?,
            string,
        })
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    pub fn as_string(&self) -> &String {
        &self.string
    }

    pub fn checksum(&self) -> u8 {
        self.as_bytes()
            .iter()
            .fold(0, |acc, x| acc.wrapping_add(*x))
    }
}
