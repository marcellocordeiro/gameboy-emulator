use super::error::Error;

/// Not the actual start, we're starting at 0 for convenience, as 0..0x100 is the bootrom area.
/// The actual start is at 0x100.
pub const HEADER_START: usize = 0;
pub const HEADER_END: usize = 0x014F;

pub const HEADER_SIZE: usize = HEADER_START + HEADER_END + 1;

pub type Header = [u8; HEADER_SIZE];

pub fn try_from(rom: &[u8]) -> Result<&Header, Error> {
    rom.get(HEADER_START..=HEADER_END)
        .ok_or(Error::InvalidRom)?
        .try_into()
        .map_err(|_| Error::InvalidRom)
}
