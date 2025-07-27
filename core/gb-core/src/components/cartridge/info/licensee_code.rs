use super::header::Header;
use crate::components::cartridge::error::CartridgeError;

const OLD_LICENSEE_CODE_ADDRESS: usize = 0x014B;

const NEW_LICENSEE_CODE_ADDRESS_BEGIN: usize = 0x0144;
const NEW_LICENSEE_CODE_ADDRESS_END: usize = 0x0145;

pub struct LicenseeCode {
    old_code: u8,
    new_code: Option<String>,
}

impl LicenseeCode {
    pub fn from_header(header: &Header) -> Result<Self, CartridgeError> {
        let old_code = header[OLD_LICENSEE_CODE_ADDRESS];

        let new_code = if old_code == 0x33 {
            let new_bytes: [u8; 2] = header
                [NEW_LICENSEE_CODE_ADDRESS_BEGIN..=NEW_LICENSEE_CODE_ADDRESS_END]
                .try_into()
                .map_err(|_err| CartridgeError::InvalidRom)?;

            Some(Self::new_bytes_to_string(new_bytes))
        } else {
            None
        };

        Ok(Self { old_code, new_code })
    }

    #[must_use]
    pub fn old_code(&self) -> u8 {
        self.old_code
    }

    #[must_use]
    pub fn new_code(&self) -> Option<&str> {
        self.new_code.as_deref()
    }

    #[must_use]
    pub fn is_nintendo(&self) -> bool {
        (self.old_code() == 0x01) || (self.old_code() == 0x33 && self.new_code() == Some("01"))
    }

    fn new_bytes_to_string(bytes: [u8; 2]) -> String {
        String::from_utf8_lossy(&bytes)
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '?' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::cartridge::info::header::HEADER_SIZE;

    #[test]
    fn test_old_licensee_code() {
        // 0x01
        let header = generate_header(0x01, [0, 0]);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.old_code(), 0x01);
        assert!(licensee_code.is_nintendo());

        // 0x42 (not Nintendo)
        let header = generate_header(0x42, [0, 0]);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.old_code(), 0x42);
        assert!(!licensee_code.is_nintendo());
    }

    #[test]
    fn test_new_licensee_code() {
        let header = generate_header(0x33, [b'0', b'1']);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), Some("01"));
        assert!(licensee_code.is_nintendo());

        let header = generate_header(0x33, [b'4', b'2']);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), Some("42"));
        assert!(!licensee_code.is_nintendo());

        let header = generate_header(0x33, [b'A', b'B']);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), Some("AB"));
        assert!(!licensee_code.is_nintendo());
    }

    #[test]
    fn test_invalid_new_licensee_code() {
        let header = generate_header(0x42, [1, 2]);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), None);

        let header = generate_header(0x42, [1, b'2']);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), None);
    }

    #[test]
    fn test_weird_new_licensee_code() {
        let header = generate_header(0x33, [1, 2]);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), Some("??"));

        let header = generate_header(0x33, [1, b'2']);
        let licensee_code = LicenseeCode::from_header(&header).unwrap();

        assert_eq!(licensee_code.new_code(), Some("?2"));
    }

    fn generate_header(old: u8, new: [u8; 2]) -> Header {
        let mut header = [0; HEADER_SIZE];

        header[OLD_LICENSEE_CODE_ADDRESS] = old;

        let new_code_slice =
            &mut header[NEW_LICENSEE_CODE_ADDRESS_BEGIN..=NEW_LICENSEE_CODE_ADDRESS_END];
        new_code_slice.copy_from_slice(&new);

        header
    }
}
