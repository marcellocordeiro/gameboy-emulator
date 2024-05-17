// TODO: merge this with MBC3.
use std::sync::Arc;

use super::MbcInterface;
use crate::{
    cartridge::info::{CartridgeInfo, MbcType, RAM_BANK_SIZE, ROM_BANK_SIZE},
    constants::ONE_KIB,
};

pub struct Mbc30 {
    rom: Arc<Box<[u8]>>,
    ram: Box<[u8]>,

    ram_enable: bool,

    rom_bank: u8,
    ram_rtc_sel: u8,
}

impl Mbc30 {
    pub fn new(info: &CartridgeInfo, rom: Arc<Box<[u8]>>) -> Self {
        assert_eq!(info.mbc_type, MbcType::Mbc30);

        let ram_banks = info.ram_banks;

        Self {
            rom,
            ram: vec![0; ram_banks * (8 * ONE_KIB)].into_boxed_slice(),

            ram_enable: false,

            rom_bank: 0x01,
            ram_rtc_sel: 0x00,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        ROM_BANK_SIZE * (self.rom_bank as usize)
    }

    fn ram_offset(&self) -> usize {
        RAM_BANK_SIZE * ((self.ram_rtc_sel & 0b111) as usize)
    }
}

impl MbcInterface for Mbc30 {
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

            0x2000..=0x3FFF => {
                self.rom_bank = value;
                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }

            0x4000..=0x5FFF => self.ram_rtc_sel = value & 0x0F,

            0x6000..=0x7FFF => (), // todo!("[mbc3.rs] RTC not yet supported."),

            _ => unreachable!(
                "[mbc3.rs] Invalid write: ({:#06x}) = {:#04x}",
                address, value
            ),
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
