use std::sync::Arc;

use super::MbcInterface;
use crate::{components::cartridge::info::Info, constants::ONE_KIB};

pub struct NoMbc {
    rom: Arc<Box<[u8]>>,
    ram: Box<[u8]>,
}

impl NoMbc {
    pub fn new(cartridge_info: &Info) -> Self {
        let ram_banks = cartridge_info.ram_banks;

        Self {
            rom: cartridge_info.rom.clone(),
            ram: vec![0; ram_banks * (8 * ONE_KIB)].into_boxed_slice(),
        }
    }
}

impl MbcInterface for NoMbc {
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
        let address = address as usize;
        self.rom[address]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram.is_empty() {
            return 0xFF;
        }

        let address = (address - 0xA000) as usize;

        self.ram[address]
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {}

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram.is_empty() {
            return;
        }

        let address = (address - 0xA000) as usize;

        self.ram[address] = value;
    }
}
