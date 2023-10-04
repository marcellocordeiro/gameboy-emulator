use crate::cartridge::error::Error as CartridgeError;

pub const OLD_LICENSEE_CODE_ADDRESS: usize = 0x014B;

pub const NEW_LICENSEE_CODE_ADDRESS_BEGIN: usize = 0x0144;
pub const NEW_LICENSEE_CODE_ADDRESS_END: usize = 0x0145;

pub struct LicenseeCode {
    old: u8,
    new_bytes: [u8; 2],
}

impl LicenseeCode {
    pub fn with_rom(rom: &[u8]) -> Result<Self, CartridgeError> {
        let old_licensee_code = *rom
            .get(OLD_LICENSEE_CODE_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let new_licensee_code_bytes = rom
            .get(NEW_LICENSEE_CODE_ADDRESS_BEGIN..=NEW_LICENSEE_CODE_ADDRESS_END)
            .ok_or(CartridgeError::InvalidRom)?;

        Ok(Self {
            old: old_licensee_code,
            new_bytes: new_licensee_code_bytes
                .try_into()
                .map_err(|_err| CartridgeError::InvalidRom)?,
        })
    }

    pub fn old(&self) -> u8 {
        self.old
    }

    pub fn new_as_bytes(&self) -> &[u8; 2] {
        &self.new_bytes
    }

    pub fn new_as_string(&self) -> String {
        String::from_utf8_lossy(&self.new_bytes)
            .trim()
            .chars()
            .filter(char::is_ascii)
            .collect()
    }
}
