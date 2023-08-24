use crate::constants::ONE_KIB;

const WRAM_BANKS: usize = 8;
const WRAM_BANK_SIZE: usize = 4 * ONE_KIB; // 4KiB each, 4096 (0x1000)
const WRAM_SIZE: usize = WRAM_BANKS * WRAM_BANK_SIZE; // 32768 (0x8000)

pub struct WorkRam {
    data: [u8; WRAM_SIZE],
    svbk: u8, // (CGB) WRAM Bank Select.
}

impl Default for WorkRam {
    fn default() -> Self {
        Self {
            data: [0; WRAM_SIZE], // can't default this :(
            svbk: 0,
        }
    }
}

impl WorkRam {
    // 0xC000 ~ 0xDFFF

    // 0xC000 ~ 0xCFFF: bank 0.
    // 0xD000 ~ 0xDFFF: In CGB mode, switchable bank 1~7.
    // 0xE000 ~ 0xFDFF: ECHO RAM (prohibited area, but mirrors 0xC000 ~ 0xDDFF).

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xC000..=0xCFFF => self.data[address as usize - 0xC000],
            0xD000..=0xDFFF => self.data[address as usize - 0xC000 + self.bank_offset()], // CGB only.

            // ECHO RAM.
            0xE000..=0xEFFF => self.data[address as usize - 0xE000],
            0xF000..=0xFDFF => self.data[address as usize - 0xE000 + self.bank_offset()], // CGB only.

            0xFF70 => self.read_svbk(),

            _ => unreachable!("[work_ram.rs] Read out of bounds: {:#06x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xC000..=0xCFFF => self.data[address as usize - 0xC000] = value,
            0xD000..=0xDFFF => self.data[address as usize - 0xC000 + self.bank_offset()] = value, // CGB only.

            // ECHO RAM.
            0xE000..=0xEFFF => self.data[address as usize - 0xE000] = value,
            0xF000..=0xFDFF => self.data[address as usize - 0xE000 + self.bank_offset()] = value, // CGB only.

            0xFF70 => self.write_svbk(value),

            _ => unreachable!(
                "[work_ram.rs] Write out of bounds: ({:#06x}) = {:#04x}",
                address, value
            ),
        }
    }

    fn bank_offset(&self) -> usize {
        WRAM_BANK_SIZE * (self.svbk as usize)
    }

    fn read_svbk(&self) -> u8 {
        if cfg!(feature = "cgb-mode") {
            0b1111_1100 | self.svbk
        } else {
            0xFF
        }
    }

    fn write_svbk(&mut self, value: u8) {
        if cfg!(feature = "cgb-mode") {
            self.svbk = value & 0b111
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sanity() {
        let wram = WorkRam::default();

        assert_eq!(wram.data.len(), 0x8000);
    }

    #[test]
    #[cfg(feature = "cgb-mode")]
    fn test_read_banks() {
        let mut wram = WorkRam::default();

        let chunks = wram.data.chunks_exact_mut(WRAM_BANK_SIZE);

        assert_eq!(chunks.len(), WRAM_BANKS); // 8 banks

        for (bank, chunk) in chunks.enumerate() {
            let chunk_iter = chunk.iter_mut();

            assert_eq!(chunk_iter.len(), WRAM_BANK_SIZE);

            for element in chunk_iter {
                *element = bank as u8;
            }
        }

        verify_banks(&mut wram);
    }

    #[test]
    #[cfg(feature = "cgb-mode")]
    fn test_write_banks() {
        let mut wram = WorkRam::default();

        // Bank 0
        for address in 0xC000..0xCFFF {
            wram.write(address, 0);
        }

        // Bank 1
        wram.write(0xFF70, 0b1111_1000 | 0b000);
        assert_eq!(wram.svbk, 0b000, "Should ignore bits 2-7");

        for address in 0xD000..0xDFFF {
            wram.write(address, 1);
        }

        // Bank 2
        wram.write(0xFF70, 0b1111_1000 | 0b001);
        assert_eq!(wram.svbk, 0b001, "Should ignore bits 2-7");

        for address in 0xD000..0xDFFF {
            wram.write(address, 2);
        }

        // Banks 3-7
        for bank in 3..7 {
            wram.write(0xFF70, 0b1111_1000 | (bank - 1));
            assert_eq!(wram.svbk, bank - 1, "Should ignore bits 2-7");

            for address in 0xD000..0xDFFF {
                wram.write(address, bank);
            }
        }

        verify_banks(&mut wram);
    }

    #[cfg(feature = "cgb-mode")]
    fn verify_banks(wram: &mut WorkRam) {
        // Bank 0
        for address in 0xC000..0xCFFF {
            assert_eq!(wram.read(address), 0);
        }

        // Bank 1
        wram.write(0xFF70, 0b1111_1000 | 0b000);
        assert_eq!(wram.svbk, 0b000, "Should ignore bits 2-7");

        for address in 0xD000..0xDFFF {
            assert_eq!(wram.read(address), 1);
        }

        // Bank 2
        wram.write(0xFF70, 0b1111_1000 | 0b001);
        assert_eq!(wram.svbk, 0b001, "Should ignore bits 2-7");

        for address in 0xD000..0xDFFF {
            assert_eq!(wram.read(address), 2);
        }

        // Banks 3-7
        for bank in 3..7 {
            wram.write(0xFF70, 0b1111_1000 | (bank - 1));
            assert_eq!(wram.svbk, bank - 1, "Should ignore bits 2-7");

            for address in 0xD000..0xDFFF {
                assert_eq!(wram.read(address), bank);
            }
        }
    }
}
