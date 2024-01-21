use std::sync::Arc;

use self::{
    error::Error as CartridgeError,
    info::Info,
    mbc::{Mbc, MbcInterface},
};

pub struct Cartridge {
    pub(crate) info: Info,
    mbc: Mbc,
}

impl Cartridge {
    pub(crate) fn new(rom: Arc<Box<[u8]>>) -> Result<Self, CartridgeError> {
        let info = Info::try_from(rom.as_ref().as_ref())?;

        // Panics if the validation fails.
        info.validate();

        let mbc = Mbc::try_new(rom, &info)?;

        Ok(Self { info, mbc })
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
