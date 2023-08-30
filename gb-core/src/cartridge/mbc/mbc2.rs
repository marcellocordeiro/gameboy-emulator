use crate::cartridge::info::{CartridgeType, Info, ROM_BANK_SIZE};

use super::MbcInterface;

pub struct Mbc2 {
    rom: Vec<u8>,
    ram: [u8; 512],

    ram_enable: bool,

    rom_bank: u8,
}

impl Mbc2 {
    pub fn new(rom: Vec<u8>, info: &Info) -> Self {
        assert_eq!(info.cartridge_type, CartridgeType::Mbc2);

        Self {
            rom,
            ram: [0; 512],
            ram_enable: false,
            rom_bank: 0x01,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        ROM_BANK_SIZE * (self.rom_bank as usize)
    }
}

impl MbcInterface for Mbc2 {
    fn read_rom(&self, address: u16) -> u8 {
        if address < 0x4000 {
            return self.rom[address as usize];
        }

        let offset = self.rom_0x4000_0x7fff_offset();
        let mapped_address = (address as usize - 0x4000) + offset;
        let mask = self.rom.len() - 1;

        self.rom[mapped_address & mask]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable {
            return 0xFF;
        }

        let mapped_address = (address - 0xA000) & 0x01FF;

        self.ram[mapped_address as usize] | 0xF0
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        if address >= 0x4000 {
            return;
        }

        // If the bit 8 (LSB from the upper address byte) is set,
        // select `ram_enable`. Otherwise, select `rom_bank`.
        if address & 0x0100 == 0 {
            self.ram_enable = (value & 0b1111) == 0x0A;
        } else {
            self.rom_bank = value & 0b1111;
            if self.rom_bank == 0 {
                self.rom_bank = 1;
            }
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable {
            return;
        }

        let mapped_address = (address - 0xA000) & 0x01FF;

        self.ram[mapped_address as usize] = value;
    }
}
