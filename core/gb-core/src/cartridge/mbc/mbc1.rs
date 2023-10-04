use crate::{
    cartridge::info::{CartridgeType, Info, RAM_BANK_SIZE, ROM_BANK_SIZE},
    constants::ONE_KIB,
};

use super::MbcInterface;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_bank_mask: usize,
    ram_bank_mask: usize,

    ram_enable: bool,

    mode: bool,

    bank_lo: u8,
    bank_hi: u8,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>, info: &Info) -> Self {
        assert_eq!(info.cartridge_type, CartridgeType::Mbc1);

        let ram_banks = info.ram_banks;

        let rom_bank_mask = info.rom_banks - 1;
        let ram_bank_mask = if ram_banks == 0 { 0 } else { ram_banks - 1 };

        Self {
            rom,
            ram: vec![0; ram_banks * (8 * ONE_KIB)],

            ram_enable: false,
            mode: false,

            rom_bank_mask,
            ram_bank_mask,

            bank_lo: 0x01,
            bank_hi: 0x00,
        }
    }

    fn rom_0x0000_0x3fff_offset(&self) -> usize {
        if !self.mode {
            return 0;
        }

        let rom_bank = (self.bank_hi << 5) as usize;

        ROM_BANK_SIZE * (rom_bank & self.rom_bank_mask)
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        let rom_bank = ((self.bank_hi << 5) | self.bank_lo) as usize;

        ROM_BANK_SIZE * (rom_bank & self.rom_bank_mask)
    }

    fn ram_offset(&self) -> usize {
        if !self.mode {
            return 0;
        }

        let ram_bank = self.bank_hi as usize;

        RAM_BANK_SIZE * (ram_bank & self.ram_bank_mask)
    }
}

impl MbcInterface for Mbc1 {
    fn reset(&mut self) {
        self.ram.fill(0);
        self.ram_enable = false;
        self.mode = false;
        self.bank_lo = 0x01;
        self.bank_hi = 0x00;
    }

    fn get_battery(&self) -> &[u8] {
        &self.ram
    }

    fn load_battery(&mut self, file: Vec<u8>) {
        if self.ram.is_empty() {
            log::error!("This cartridge does not have a battery backed RAM.");
            return;
        } else if self.ram.len() != file.len() {
            log::error!("Size mismatch.");
            return;
        }

        self.ram = file;
    }

    fn read_rom_bank_0(&self, address: u16) -> u8 {
        let address = address as usize;
        let offset = self.rom_0x0000_0x3fff_offset();

        self.rom[address + offset]
    }

    fn read_rom_bank_x(&self, address: u16) -> u8 {
        let address = (address - 0x4000) as usize;
        let offset = self.rom_0x4000_0x7fff_offset();

        self.rom[address + offset]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable || self.ram.is_empty() {
            return 0xFF;
        }

        let address = (address - 0xA000) as usize;
        let offset = self.ram_offset();

        self.ram[address + offset]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enable = (value & 0b1111) == 0x0A,

            0x2000..=0x3FFF => {
                self.bank_lo = value & 0b0001_1111;
                if self.bank_lo == 0 {
                    self.bank_lo = 1;
                }
            }

            0x4000..=0x5FFF => self.bank_hi = value & 0b11,
            0x6000..=0x7FFF => self.mode = (value & 0b1) != 0,

            _ => unreachable!(
                "[mbc1.rs] Invalid write: ({:#06x}) = {:#04x}",
                address, value
            ),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable || self.ram.is_empty() {
            return;
        }

        let address = (address - 0xA000) as usize;
        let offset = self.ram_offset();

        self.ram[address + offset] = value;
    }
}
