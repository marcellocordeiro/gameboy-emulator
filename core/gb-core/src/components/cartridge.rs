use std::sync::Arc;

use error::CartridgeError;
use info::Info;
use mbc::{Mbc, MbcInterface};

pub struct Cartridge {
    pub rom: Arc<Box<[u8]>>,
    pub info: Info,
    pub mbc: Mbc,
}

impl Cartridge {
    pub fn new(rom: Arc<Box<[u8]>>) -> Result<Self, CartridgeError> {
        let info = Info::new(rom.clone())?;
        let mbc = Mbc::new(&info);

        Ok(Self { rom, info, mbc })
    }

    #[must_use]
    pub fn get_battery(&self) -> &[u8] {
        self.mbc.get_battery()
    }

    pub fn load_battery(&mut self, file: Vec<u8>) {
        self.mbc.load_battery(file);
    }

    #[must_use]
    pub fn read_rom_bank_0(&self, address: u16) -> u8 {
        self.mbc.read_rom_bank_0(address)
    }

    #[must_use]
    pub fn read_rom_bank_x(&self, address: u16) -> u8 {
        self.mbc.read_rom_bank_x(address)
    }

    #[must_use]
    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(address)
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc.write_rom(address, value);
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.mbc.write_ram(address, value);
    }
}

pub mod error;
pub mod info;
pub mod mbc;
