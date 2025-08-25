use super::header::Header;

pub const TITLE_ADDRESS_BEGIN: usize = 0x0134;
pub const TITLE_ADDRESS_END: usize = 0x0143;

pub const TITLE_SIZE: usize = TITLE_ADDRESS_END - TITLE_ADDRESS_BEGIN + 1;

pub struct Title {
    bytes: [u8; TITLE_SIZE],
}

impl Title {
    #[must_use]
    pub fn from_header(header: &Header) -> Self {
        let bytes = header[TITLE_ADDRESS_BEGIN..=TITLE_ADDRESS_END]
            .try_into()
            .unwrap();

        Self { bytes }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    #[must_use]
    pub fn checksum(&self) -> u8 {
        self.as_bytes()
            .iter()
            .fold(0, |acc, x| acc.wrapping_add(*x))
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = String::from_utf8_lossy(&self.bytes)
            .trim()
            .chars()
            .filter(char::is_ascii)
            .collect::<String>();

        write!(f, "{value}")
    }
}
