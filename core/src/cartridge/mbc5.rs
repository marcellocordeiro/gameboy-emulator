use super::mbc::{Mbc, ONE_KIB, RAM_BANK_SIZE, ROM_BANK_SIZE};

pub(super) struct Mbc5 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    ram_enable: bool,

    rom_bank_lo: u8,
    rom_bank_hi: u8,
    ram_bank: u8,
}

impl Mbc5 {
    pub fn new(rom: Vec<u8>) -> Self {
        let rom_banks = {
            let code = rom[0x148];

            match code {
                0x00 => 2,   // 32 KiB
                0x01 => 4,   // 64 KiB
                0x02 => 8,   // 128 KiB
                0x03 => 16,  // 256 KiB
                0x04 => 32,  // 512 KiB
                0x05 => 64,  // 1 MiB
                0x06 => 128, // 2 MiB
                0x07 => 256, // 4 MiB
                0x08 => 512, // 8 MiB

                _ => panic!("[mbc5.rs] Unsupported number of banks."),
            }
        };

        let ram_banks = {
            let code = rom[0x149];

            match code {
                0x00 => 0,
                0x01 => 1,
                0x02 => 1,
                0x03 => 4,
                0x04 => 16,
                0x05 => 8,

                _ => panic!("[mbc5.rs] Unsupported number of banks."),
            }
        };

        println!("MBC5");
        println!("ROM banks: {rom_banks}");
        println!("RAM banks: {ram_banks}");

        assert_eq!(rom.len(), (rom_banks / 2) * (32 * ONE_KIB));

        Self {
            rom,
            ram: vec![0; ram_banks * (8 * ONE_KIB)],
            ram_enable: false,
            rom_bank_lo: 0x01,
            rom_bank_hi: 0x00,
            ram_bank: 0x00,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        let rom_bank = ((self.rom_bank_hi as usize) << 8) | (self.rom_bank_lo as usize);

        ROM_BANK_SIZE * rom_bank
    }

    fn ram_offset(&self) -> usize {
        RAM_BANK_SIZE * (self.ram_bank as usize)
    }
}

impl Mbc for Mbc5 {
    fn read_rom(&self, address: u16) -> u8 {
        let mask = self.rom.len() - 1;

        if address < 0x4000 {
            return self.rom[address as usize];
        }

        let offset = self.rom_0x4000_0x7fff_offset();
        let mapped_address = (address as usize - 0x4000) + offset;

        self.rom[mapped_address & mask]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable {
            return 0xFF;
        }

        let offset = self.ram_offset();
        let mapped_address = ((address as usize) - 0xA000) + offset;

        self.ram[mapped_address]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enable = (value & 0b1111) == 0x0A,

            0x2000..=0x2FFF => {
                self.rom_bank_lo = value;
            }

            0x3000..=0x3FFF => {
                self.rom_bank_hi = value & 0b1;
            }

            0x4000..=0x5FFF => {
                self.ram_bank = value & 0b1111;
            }

            _ => unreachable!(
                "[mbc5.rs] Invalid write: ({:#06x}) = {:#04x}",
                address, value
            ),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable {
            return;
        }

        let offset = self.ram_offset();
        let mapped_address = ((address as usize) - 0xA000) + offset;

        self.ram[mapped_address] = value;
    }
}