use std::sync::Arc;

use super::MbcInterface;
use crate::{
    components::cartridge::info::{Info, ram_banks::RAM_BANK_SIZE, rom_banks::ROM_BANK_SIZE},
    constants::ONE_KIB,
};

pub struct Mbc5 {
    rom: Arc<[u8]>,
    ram: Box<[u8]>,

    rom_bank_mask: usize,

    ram_enable: bool,

    rom_bank: u16,
    ram_bank: u8,
}

impl Mbc5 {
    pub fn new(cartridge_info: &Info) -> Self {
        let ram_banks = cartridge_info.ram_banks;

        let rom_bank_mask = cartridge_info.rom_banks - 1;

        Self {
            rom: cartridge_info.rom.clone(),
            ram: vec![0; ram_banks * (8 * ONE_KIB)].into_boxed_slice(),

            rom_bank_mask,

            ram_enable: false,

            rom_bank: 0x0001,
            ram_bank: 0x00,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        ROM_BANK_SIZE * ((self.rom_bank as usize) & self.rom_bank_mask)
    }

    fn ram_offset(&self) -> usize {
        RAM_BANK_SIZE * (self.ram_bank as usize)
    }
}

impl MbcInterface for Mbc5 {
    fn get_battery(&self) -> &[u8] {
        &self.ram
    }

    fn load_battery(&mut self, file: Vec<u8>) {
        if self.ram.is_empty() {
            log::error!("This cartridge does not have a battery backed RAM");
            return;
        } else if self.ram.len() != file.len() {
            log::error!("Size mismatch");
            return;
        }

        self.ram = file.into_boxed_slice();
    }

    fn read_rom_bank_0(&self, address: u16) -> u8 {
        let address = address as usize;
        self.rom[address]
    }

    fn read_rom_bank_x(&self, address: u16) -> u8 {
        let address = (address - 0x4000) as usize;
        let offset = self.rom_0x4000_0x7fff_offset();

        self.rom[address + offset]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable {
            return 0xFF;
        }

        let address = (address - 0xA000) as usize;
        let offset = self.ram_offset();

        self.ram[address + offset]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enable = (value & 0b1111) == 0x0A,

            0x2000..=0x2FFF => {
                let lo = value as u16;
                let hi = self.rom_bank & 0x0100;

                self.rom_bank = hi | lo;
            }

            0x3000..=0x3FFF => {
                let lo = self.rom_bank & 0x00FF;
                let hi = (value & 0b1) as u16;

                self.rom_bank = (hi << 8) | lo;
            }

            0x4000..=0x5FFF => self.ram_bank = value & 0b1111,
            0x6000..=0x7FFF => (),

            _ => unreachable!("Invalid write: ({address:#06x}) = {value:#04x}"),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable {
            return;
        }

        let address = (address - 0xA000) as usize;
        let offset = self.ram_offset();

        self.ram[address + offset] = value;
    }
}
