use std::sync::Arc;

use self::{
    info::CartridgeInfo,
    mbc::{Mbc, MbcInterface},
};

pub struct Cartridge {
    mbc: Mbc,
}

impl Cartridge {
    pub(crate) fn new(cartridge_info: &CartridgeInfo, rom: Arc<Box<[u8]>>) -> Self {
        // Panics if the validation fails.
        cartridge_info.validate();

        let mbc = Mbc::new(cartridge_info, rom);

        Self { mbc }
    }

    pub(crate) fn get_battery(&self) -> &[u8] {
        self.mbc.get_battery()
    }

    pub(crate) fn load_battery(&mut self, file: Vec<u8>) {
        self.mbc.load_battery(file);
    }

    pub(crate) fn read_rom_bank_0(&self, address: u16) -> u8 {
        self.mbc.read_rom_bank_0(address)
    }

    pub(crate) fn read_rom_bank_x(&self, address: u16) -> u8 {
        self.mbc.read_rom_bank_x(address)
    }

    pub(crate) fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(address)
    }

    pub(crate) fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc.write_rom(address, value);
    }

    pub(crate) fn write_ram(&mut self, address: u16, value: u8) {
        self.mbc.write_ram(address, value);
    }
}

pub mod error;
pub mod info;
mod mbc;
