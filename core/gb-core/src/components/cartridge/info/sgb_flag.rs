use super::header::Header;

pub const SGB_FLAG_ADDRESS: usize = 0x0146;

#[must_use]
pub fn from_header(header: &Header) -> bool {
    header[SGB_FLAG_ADDRESS] == 0x03
}
