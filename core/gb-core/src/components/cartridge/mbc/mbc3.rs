use std::sync::Arc;

use super::MbcInterface;
use crate::{
    components::cartridge::info::{Info, ram_banks::RAM_BANK_SIZE, rom_banks::ROM_BANK_SIZE},
    constants::ONE_KIB,
};

pub struct Mbc3 {
    rom: Arc<[u8]>,
    ram: Box<[u8]>,

    ram_rtc_enable: bool,

    rom_bank: u8,
    ram_rtc_sel: Option<RamRtcSelection>,

    is_mbc30: bool,
}

#[derive(Debug, Clone, Copy)]
enum RamRtcSelection {
    RamBank(u8),
    RtcRegister(u8),
}

impl Mbc3 {
    // FIXME: need to check if the cartridge supports the timer
    pub fn new(cartridge_info: &Info) -> Self {
        let rom_banks = cartridge_info.rom_banks;
        let ram_banks = cartridge_info.ram_banks;

        // ROM size > 2MiB or RAM size > 32KiB
        let is_mbc30 = rom_banks > 128 || ram_banks > 4;

        if is_mbc30 {
            log::info!("MBC30 variant");
        }

        Self {
            rom: cartridge_info.rom.clone(),
            ram: vec![0; ram_banks * (8 * ONE_KIB)].into_boxed_slice(),

            ram_rtc_enable: false,

            rom_bank: 0x01,
            ram_rtc_sel: Some(RamRtcSelection::RamBank(0)),

            is_mbc30,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        ROM_BANK_SIZE * (self.rom_bank as usize)
    }

    fn ram_offset(bank: u8) -> usize {
        // Bank is guaranteed to be between 0 and 7.
        RAM_BANK_SIZE * (bank as usize)
    }
}

impl MbcInterface for Mbc3 {
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
        use RamRtcSelection::{RamBank, RtcRegister};

        if !self.ram_rtc_enable {
            return 0xFF;
        }

        let Some(ram_rtc_sel) = self.ram_rtc_sel else {
            return 0xFF;
        };

        match ram_rtc_sel {
            RamBank(bank) => {
                let address = (address - 0xA000) as usize;
                let offset = Self::ram_offset(bank);

                self.ram[address + offset]
            }

            RtcRegister(_register) => 0x00,
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        use RamRtcSelection::{RamBank, RtcRegister};

        match address {
            0x0000..=0x1FFF => self.ram_rtc_enable = (value & 0b1111) == 0x0A,

            0x2000..=0x3FFF => {
                self.rom_bank = if self.is_mbc30 {
                    value
                } else {
                    value & 0b0111_1111
                };

                if self.rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }

            0x4000..=0x5FFF => {
                self.ram_rtc_sel = match value {
                    0x00..=0x03 if !self.is_mbc30 => Some(RamBank(value)),
                    0x00..=0x07 if self.is_mbc30 => Some(RamBank(value)),
                    0x08..=0x0C => Some(RtcRegister(value)),

                    _ => None,
                }
            }

            0x6000..=0x7FFF => { /* Latch RTC registers */ }

            _ => unreachable!("Invalid write: ({address:#06x}) = {value:#04x}"),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        use RamRtcSelection::{RamBank, RtcRegister};

        if !self.ram_rtc_enable {
            return;
        }

        let Some(ram_rtc_sel) = self.ram_rtc_sel else {
            return;
        };

        match ram_rtc_sel {
            RamBank(bank) => {
                let address = (address - 0xA000) as usize;
                let offset = Self::ram_offset(bank);

                self.ram[address + offset] = value;
            }

            RtcRegister(_register) => {}
        }
    }
}
