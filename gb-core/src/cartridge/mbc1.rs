use log::info;

use crate::constants::ONE_KIB;

use super::{
    info::{
        get_ram_banks, get_rom_banks, RAM_BANKS_CODE_ADDRESS, RAM_BANK_SIZE,
        ROM_BANKS_CODE_ADDRESS, ROM_BANK_SIZE,
    },
    mbc::MbcInterface,
};

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    ram_enable: bool,

    bank_lo: u8,
    bank_hi: u8,

    mode: bool,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>) -> Result<Self, super::Error> {
        let rom_banks = {
            let code = *rom
                .get(ROM_BANKS_CODE_ADDRESS)
                .ok_or(super::Error::InvalidRom)?;

            get_rom_banks(code)
        }?;

        let ram_banks = {
            let code = *rom
                .get(RAM_BANKS_CODE_ADDRESS)
                .ok_or(super::Error::InvalidRom)?;

            get_ram_banks(code)
        }?;

        info!("MBC1");
        info!("ROM banks: {rom_banks}");
        info!("RAM banks: {ram_banks}");

        assert_eq!(rom.len(), (rom_banks / 2) * (32 * ONE_KIB));

        Ok(Self {
            rom,
            ram: vec![0; ram_banks * (8 * ONE_KIB)],
            ram_enable: false,
            bank_lo: 0x01,
            bank_hi: 0x00,
            mode: false,
        })
    }

    fn rom_0x0000_0x3fff_offset(&self) -> usize {
        if !self.mode {
            return 0;
        }

        let rom_bank = (self.bank_hi << 5) as usize;

        ROM_BANK_SIZE * rom_bank
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        let rom_bank = ((self.bank_hi << 5) + self.bank_lo) as usize;

        ROM_BANK_SIZE * rom_bank
    }

    fn ram_offset(&self) -> usize {
        if !self.mode {
            return 0;
        }

        let ram_bank = self.bank_hi;

        RAM_BANK_SIZE * (ram_bank as usize)
    }
}

impl MbcInterface for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        let mask = self.rom.len() - 1;

        if address < 0x4000 {
            let offset = self.rom_0x0000_0x3fff_offset();
            let mapped_address = (address as usize) + offset;

            return self.rom[mapped_address & mask];
        }

        let offset = self.rom_0x4000_0x7fff_offset();
        let mapped_address = (address as usize - 0x4000) + offset;

        self.rom[mapped_address & mask]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable || self.ram.is_empty() {
            return 0xFF;
        }

        let offset = self.ram_offset();
        let mapped_address = ((address as usize) - 0xA000) + offset;

        let mask = self.ram.len() - 1;

        self.ram[mapped_address & mask]
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

        let offset = self.ram_offset();
        let mapped_address = ((address as usize) - 0xA000) + offset;

        let mask = self.ram.len() - 1;

        self.ram[mapped_address & mask] = value;
    }
}
