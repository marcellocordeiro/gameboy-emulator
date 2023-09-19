use crate::{
    cartridge::info::{CartridgeType, Info},
    constants::ONE_KIB,
};

use super::MbcInterface;

pub struct NoMbc {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: Vec<u8>, info: &Info) -> Self {
        assert_eq!(info.cartridge_type, CartridgeType::NoMbc);

        let ram_banks = info.ram_banks;

        Self {
            rom,
            ram: vec![0; ram_banks * (8 * ONE_KIB)],
        }
    }
}

impl MbcInterface for NoMbc {
    fn reset(&mut self) {
        self.ram.fill(0);
    }

    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram.is_empty() {
            unreachable!("[no_mbc.rs] RAM is unsupported.");
        }

        self.ram[(address as usize) - 0xA000]
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {
        // Tetris attempts to write here.
        // We can't simply panic :(
        // unreachable!("[no_mbc.rs] NoMBC's ROM is read-only.");
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram.is_empty() {
            unreachable!("[no_mbc.rs] RAM is unsupported.");
        }

        self.ram[(address as usize) - 0xA000] = value;
    }
}
