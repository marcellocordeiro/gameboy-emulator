use crate::{
    cartridge::info::{CartridgeType, Info, RAM_BANK_SIZE, ROM_BANK_SIZE},
    constants::ONE_KIB,
};

use super::MbcInterface;

pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    ram_enable: bool,

    rom_bank: u8,
    ram_rtc_sel: u8,
}

impl Mbc3 {
    pub fn new(rom: Vec<u8>, info: &Info) -> Self {
        // let rom_banks = info.rom_banks;
        let ram_banks = info.ram_banks;

        assert_eq!(info.cartridge_type, CartridgeType::Mbc3);
        // assert_eq!(rom.len(), (rom_banks / 2) * (32 * ONE_KIB));

        Self {
            rom,
            ram: vec![0; ram_banks * (8 * ONE_KIB)],
            ram_enable: false,
            rom_bank: 0x01,
            ram_rtc_sel: 0x00,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        ROM_BANK_SIZE * (self.rom_bank as usize)
    }

    fn ram_offset(&self) -> usize {
        RAM_BANK_SIZE * ((self.ram_rtc_sel & 0x03) as usize)
    }
}

impl MbcInterface for Mbc3 {
    fn read_rom(&self, address: u16) -> u8 {
        if address < 0x4000 {
            return self.rom[address as usize];
        }

        let offset = self.rom_0x4000_0x7fff_offset();
        let mapped_address = (address as usize - 0x4000) + offset;

        self.rom[mapped_address]
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

            0x2000..=0x3FFF => {
                self.rom_bank = value & 0b0111_1111;
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

        let offset = self.ram_offset();
        let mapped_address = ((address as usize) - 0xA000) + offset;

        self.ram[mapped_address] = value;
    }
}
