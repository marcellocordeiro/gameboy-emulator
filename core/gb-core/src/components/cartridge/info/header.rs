use crate::components::cartridge::error::CartridgeError;

/// Not the actual start, we're starting at 0 for convenience, as 0..0x100 is the bootrom area.
/// The actual start is at 0x100.
const HEADER_START: usize = 0;
const HEADER_END: usize = 0x014F;

pub const HEADER_SIZE: usize = HEADER_START + HEADER_END + 1;

pub type Header = [u8; HEADER_SIZE];

pub fn from_rom(rom: &[u8]) -> Result<&Header, CartridgeError> {
    rom.get(HEADER_START..=HEADER_END)
        .and_then(|slice| slice.try_into().ok())
        .ok_or(CartridgeError::InvalidRom)
}
