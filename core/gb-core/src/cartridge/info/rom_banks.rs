use super::header::Header;
use crate::{cartridge::error::Error, constants::ONE_KIB};

pub const ROM_BANK_SIZE: usize = 16 * ONE_KIB; // 0x4000
pub const ROM_BANKS_CODE_ADDRESS: usize = 0x0148;

/// | Code |    Size | Number of banks |
/// | ---- | ------: | --------------: |
/// | $00  |  32 KiB |  2 (no banking) |
/// | $01  |  64 KiB |               4 |
/// | $02  | 128 KiB |               8 |
/// | $03  | 256 KiB |              16 |
/// | $04  | 512 KiB |              32 |
/// | $05  |   1 MiB |              64 |
/// | $06  |   2 MiB |             128 |
/// | $07  |   4 MiB |             256 |
/// | $08  |   8 MiB |             512 |
///
/// Note: each bank is 16 KiB.
pub fn from_header(header: &Header) -> Result<usize, Error> {
    let code = header[ROM_BANKS_CODE_ADDRESS];

    from_code(code)
}

fn from_code(code: u8) -> Result<usize, Error> {
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

        _ => return Err(Error::UnsupportedRomSize { code }),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::constants::ONE_MIB_TO_KIB;

    #[test]
    fn test_sizes() {
        let mut mapping = HashMap::new();

        mapping.insert(0x00, 32);
        mapping.insert(0x01, 64);
        mapping.insert(0x02, 128);
        mapping.insert(0x03, 256);
        mapping.insert(0x04, 512);
        mapping.insert(0x05, ONE_MIB_TO_KIB);
        mapping.insert(0x06, 2 * ONE_MIB_TO_KIB);
        mapping.insert(0x07, 4 * ONE_MIB_TO_KIB);
        mapping.insert(0x08, 8 * ONE_MIB_TO_KIB);

        for (code, size) in mapping {
            let banks = from_code(code).unwrap();

            assert_eq!(banks, size / 16);
        }
    }
}
