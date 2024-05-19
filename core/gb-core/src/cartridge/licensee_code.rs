use super::{error::Error, header::Header};

pub const OLD_LICENSEE_CODE_ADDRESS: usize = 0x014B;

pub const NEW_LICENSEE_CODE_ADDRESS_BEGIN: usize = 0x0144;
pub const NEW_LICENSEE_CODE_ADDRESS_END: usize = 0x0145;

pub struct LicenseeCode {
    old: u8,
    new_bytes: [u8; 2],
}

impl LicenseeCode {
    pub fn from_header(header: &Header) -> Result<Self, Error> {
        let old_licensee_code = header[OLD_LICENSEE_CODE_ADDRESS];
        let new_licensee_code_bytes =
            &header[NEW_LICENSEE_CODE_ADDRESS_BEGIN..=NEW_LICENSEE_CODE_ADDRESS_END];

        Ok(Self {
            old: old_licensee_code,
            new_bytes: new_licensee_code_bytes
                .try_into()
                .map_err(|_err| Error::InvalidRom)?,
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
