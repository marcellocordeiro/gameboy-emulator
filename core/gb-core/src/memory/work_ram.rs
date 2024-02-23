use crate::{
    constants::ONE_KIB,
    utils::macros::in_cgb_mode,
    DeviceConfig,
    DeviceModel,
    OptionalCgbComponent,
};

// const DMG_WRAM_BANKS: usize = 2;
const CGB_WRAM_BANKS: usize = 8;

/// 4KiB each, 4096 (0x1000)
const WRAM_BANK_SIZE: usize = 4 * ONE_KIB;

/// DMG: 8192 (0x2000)
// const DMG_WRAM_SIZE: usize = DMG_WRAM_BANKS * WRAM_BANK_SIZE;

/// CGB: 32768 (0x8000)
const CGB_WRAM_SIZE: usize = CGB_WRAM_BANKS * WRAM_BANK_SIZE;

pub struct WorkRam {
    data: [u8; CGB_WRAM_SIZE],
    svbk: u8, // (CGB) WRAM Bank Select.

    device_config: DeviceConfig,
}

impl Default for WorkRam {
    fn default() -> Self {
        Self {
            data: [0; CGB_WRAM_SIZE], // can't default this :(
            svbk: 0,
            device_config: DeviceConfig::default(),
        }
    }
}

impl OptionalCgbComponent for WorkRam {
    fn with_device_model(model: DeviceModel) -> Self {
        let device_config = DeviceConfig {
            model,
            ..Default::default()
        };

        Self {
            device_config,
            ..Default::default()
        }
    }

    fn set_cgb_mode(&mut self, value: bool) {
        self.device_config.cgb_mode = value;
    }
}

impl WorkRam {
    // 0xC000 ~ 0xDFFF

    // 0xC000 ~ 0xCFFF: bank 0.
    // 0xD000 ~ 0xDFFF: Bank 1. In CGB mode, switchable bank 1~7.
    // 0xE000 ~ 0xFDFF: ECHO RAM (prohibited area, but mirrors 0xC000 ~ 0xDDFF).

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xC000..=0xCFFF => self.data[address as usize - 0xC000],
            0xD000..=0xDFFF => self.data[address as usize - 0xC000 + self.bank_offset()], // Bank selection in CGB mode only.

            // ECHO RAM.
            0xE000..=0xEFFF => self.data[address as usize - 0xE000],
            0xF000..=0xFDFF => self.data[address as usize - 0xE000 + self.bank_offset()], // Bank selection in CGB mode only.

            _ => unreachable!("[work_ram.rs] Read out of bounds: {address:#06x}"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xC000..=0xCFFF => self.data[address as usize - 0xC000] = value,
            0xD000..=0xDFFF => self.data[address as usize - 0xC000 + self.bank_offset()] = value, // Bank selection in CGB mode only.

            // ECHO RAM.
            0xE000..=0xEFFF => self.data[address as usize - 0xE000] = value,
            0xF000..=0xFDFF => self.data[address as usize - 0xE000 + self.bank_offset()] = value, // Bank selection in CGB mode only.

            _ => unreachable!("[work_ram.rs] Write out of bounds: ({address:#06x}) = {value:#04x}"),
        }
    }

    /// Warning: CGB model only.
    pub fn read_svbk(&self) -> u8 {
        if !in_cgb_mode!(self) {
            return 0xFF;
        }

        0b1111_1000 | self.svbk
    }

    /// Warning: CGB model only.
    pub fn write_svbk(&mut self, value: u8) {
        if !in_cgb_mode!(self) {
            return;
        }

        self.svbk = value & 0b111;
    }

    fn bank_offset(&self) -> usize {
        let bank = self.svbk.max(0b001);
        WRAM_BANK_SIZE * (bank as usize - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "need to adjust this"]
    fn test_my_sanity_dmg() {
        let wram = WorkRam::with_device_model(DeviceModel::Dmg);
        assert_eq!(wram.data.len(), 0x2000);
    }

    #[test]
    fn test_my_sanity_cgb() {
        let wram = WorkRam::with_device_model(DeviceModel::Cgb);
        assert_eq!(wram.data.len(), 0x8000);
    }

    #[test]
    #[cfg(feature = "cgb")]
    fn test_read_banks() {
        let mut wram = WorkRam::default();
        wram.set_cgb_mode(true);

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
    #[cfg(feature = "cgb")]
    fn test_write_banks() {
        let mut wram = WorkRam::default();
        wram.set_cgb_mode(true);

        // Bank 0
        for address in 0xC000..=0xCFFF {
            wram.write(address, 0);
        }

        // Bank 1
        wram.write_svbk(0b1111_1000 | 0b000);
        assert_eq!(wram.svbk, 0b000, "Should ignore bits 3-7");

        for address in 0xD000..=0xDFFF {
            wram.write(address, 1);
        }

        // Bank 1
        wram.write_svbk(0b1111_1000 | 0b001);
        assert_eq!(wram.svbk, 0b001, "Should ignore bits 3-7");

        for address in 0xD000..=0xDFFF {
            assert_eq!(wram.read(address), 1);
        }

        // Bank 2
        wram.write_svbk(0b1111_1000 | 0b010);
        assert_eq!(wram.svbk, 0b010, "Should ignore bits 3-7");

        for address in 0xD000..=0xDFFF {
            wram.write(address, 2);
        }

        // Banks 3-7
        for bank in 3..=7 {
            wram.write_svbk(0b1111_1000 | bank);
            assert_eq!(wram.svbk, bank, "Should ignore bits 3-7");

            for address in 0xD000..=0xDFFF {
                wram.write(address, bank);
            }
        }

        verify_banks(&mut wram);
    }

    #[cfg(feature = "cgb")]
    fn verify_banks(wram: &mut WorkRam) {
        // Bank 0
        for address in 0xC000..=0xCFFF {
            assert_eq!(wram.read(address), 0);
        }

        // Bank 1 with svkb == 0
        wram.write_svbk(0b1111_1000 | 0b000);
        assert_eq!(wram.read_svbk() & 0b111, 0b000, "Should ignore bits 3-7");
        assert_eq!(wram.svbk, 0b000, "Should ignore bits 3-7");

        for address in 0xD000..=0xDFFF {
            assert_eq!(wram.read(address), 1);
        }

        // Bank 1 with svbk == 1 (both svbk == 0 and svkb == 1 should select bank 1)
        wram.write_svbk(0b1111_1000 | 0b001);
        assert_eq!(wram.read_svbk() & 0b111, 0b001, "Should ignore bits 3-7");
        assert_eq!(wram.svbk, 0b001, "Should ignore bits 3-7");

        for address in 0xD000..=0xDFFF {
            assert_eq!(wram.read(address), 1);
        }

        // Bank 2
        wram.write_svbk(0b1111_1000 | 0b0010);
        assert_eq!(wram.svbk, 0b010, "Should ignore bits 3-7");

        for address in 0xD000..=0xDFFF {
            assert_eq!(wram.read(address), 2);
        }

        // Banks 3-7
        for bank in 3..=7 {
            wram.write_svbk(0b1111_1000 | bank);
            assert_eq!(wram.svbk, bank, "Should ignore bits 3-7");

            for address in 0xD000..=0xDFFF {
                let svbk = wram.svbk;
                let actual = wram.read(address);

                assert_eq!(
                    actual, bank,
                    "At ({address:#06x}) with svkb = {svbk}, expected {bank}, found {actual}"
                );
            }
        }
    }
}
