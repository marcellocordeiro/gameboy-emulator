use crate::{cartridge::error::Error as CartridgeError, constants::ONE_KIB};

pub const RAM_BANK_SIZE: usize = 8 * ONE_KIB; // 0x2000
pub const RAM_BANKS_CODE_ADDRESS: usize = 0x0149;

/// | Code |    Size | Number of banks |
/// | ---- | ------: | --------------: |
/// | $00  |       0 |          No RAM |
/// | $01  |       - |          Unused |
/// | $02  |   8 KiB |               1 |
/// | $03  |  32 KiB |               4 |
/// | $04  | 128 KiB |              16 |
/// | $05  |  64 KiB |               8 |
///
/// Note: each bank is 8 KiB.
pub fn get_ram_banks(code: u8) -> Result<usize, CartridgeError> {
    let result = match code {
        0x00 => 0,
        0x01 => 1, // Unofficial.
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,

        _ => return Err(CartridgeError::UnsupportedRamSize { code }),
    };

    Ok(result)
}
