// TODO: better failable returns

use crate::constants::ONE_KIB;

pub const ROM_BANK_SIZE: usize = 16 * ONE_KIB; // 0x4000
pub const RAM_BANK_SIZE: usize = 8 * ONE_KIB; // 0x2000

pub const ROM_BANKS_CODE_ADDRESS: usize = 0x0148;
pub const RAM_BANKS_CODE_ADDRESS: usize = 0x0149;

pub fn get_rom_banks(code: u8) -> Result<usize, super::Error> {
    let result = match code {
        0x00 => 2,   // 32 KiB
        0x01 => 4,   // 64 KiB
        0x02 => 8,   // 128 KiB
        0x03 => 16,  // 256 KiB
        0x04 => 32,  // 512 KiB
        0x05 => 64,  // 1 MiB
        0x06 => 128, // 2 MiB
        0x07 => 256, // 4 MiB
        0x08 => 512, // 8 MiB

        _ => return Err(super::Error::UnsupportedRomSize { code }),
    };

    Ok(result)
}

pub fn get_ram_banks(code: u8) -> Result<usize, super::Error> {
    let result = match code {
        0x00 => 0,
        0x01 => 1,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,

        _ => return Err(super::Error::UnsupportedRamSize { code }),
    };

    Ok(result)
}