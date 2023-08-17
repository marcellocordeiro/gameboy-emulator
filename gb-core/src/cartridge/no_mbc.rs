use log::info;

use super::mbc::MbcInterface;

pub struct NoMbc {
    rom: Vec<u8>,
}

impl NoMbc {
    pub fn new(rom: Vec<u8>) -> Self {
        info!("No MBC");

        Self { rom }
    }
}

impl MbcInterface for NoMbc {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_ram(&self, _address: u16) -> u8 {
        unreachable!("[no_mbc.rs] NoMBC does not have RAM.");
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {
        // Tetris attempts to write here.
        // We can't simply panic :(
        // unreachable!("[no_mbc.rs] NoMBC's ROM is read-only.");
    }

    fn write_ram(&mut self, _address: u16, _value: u8) {
        unreachable!("[no_mbc.rs] NoMBC does not have RAM.");
    }
}
