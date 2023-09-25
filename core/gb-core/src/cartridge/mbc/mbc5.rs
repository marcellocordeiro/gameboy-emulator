use crate::{
    cartridge::info::{CartridgeType, Info, RAM_BANK_SIZE, ROM_BANK_SIZE},
    constants::ONE_KIB,
};

use super::MbcInterface;

pub struct Mbc5 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    ram_enable: bool,

    rom_bank_lo: u8,
    rom_bank_hi: u8,
    ram_bank: u8,
}

impl Mbc5 {
    pub fn new(rom: Vec<u8>, info: &Info) -> Self {
        assert_eq!(info.cartridge_type, CartridgeType::Mbc5);

        let ram_banks = info.ram_banks;

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

impl MbcInterface for Mbc5 {
    fn reset(&mut self) {
        self.ram.fill(0);
        self.ram_enable = false;
        self.rom_bank_lo = 0x01;
        self.rom_bank_hi = 0x00;
        self.ram_bank = 0x00;
    }

    fn get_battery(&self) -> &[u8] {
        &self.ram
    }

    fn load_battery(&mut self, file: Vec<u8>) {
        if self.ram.is_empty() {
            log::error!("This cartridge does not have a battery backed RAM.");
        } else if self.ram.len() != file.len() {
            log::error!("Size mismatch.");
        }

        self.ram = file;
    }

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

            0x2000..=0x2FFF => self.rom_bank_lo = value,

            0x3000..=0x3FFF => self.rom_bank_hi = value & 0b1,

            0x4000..=0x5FFF => self.ram_bank = value & 0b1111,

            0x6000..=0x7FFF => (),

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
