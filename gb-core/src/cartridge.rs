pub use self::error::Error;

use self::{
    mbc::{Mbc, MbcInterface},
    mbc1::Mbc1,
    mbc2::Mbc2,
    mbc3::Mbc3,
    mbc5::Mbc5,
    no_mbc::NoMbc,
};

#[derive(Default)]
pub struct Cartridge {
    mbc: Option<Mbc>,
}

impl Cartridge {
    pub fn load_cartridge(&mut self, rom: Vec<u8>) -> Result<(), self::Error> {
        self.mbc = Some(Self::get_mbc(rom)?);

        Ok(())
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.mbc
            .as_ref()
            .expect("ROM should be loaded before running")
            .read_rom(address)
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc
            .as_ref()
            .expect("ROM should be loaded before running")
            .read_ram(address)
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc
            .as_mut()
            .expect("ROM should be loaded before running")
            .write_rom(address, value);
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.mbc
            .as_mut()
            .expect("ROM should be loaded before running")
            .write_ram(address, value);
    }

    // todo: make this failable
    fn get_mbc(rom: Vec<u8>) -> Result<Mbc, self::Error> {
        const CARTRIDGE_TYPE_ADDRESS: usize = 0x147;

        let raw_cartridge_type = *rom
            .get(CARTRIDGE_TYPE_ADDRESS)
            .ok_or(self::Error::InvalidRom)?;

        let mbc: Mbc = match raw_cartridge_type {
            // $00 ROM ONLY
            // $08 ROM+RAM
            // $09 ROM+RAM+BATTERY
            0x00 => NoMbc::new(rom).into(),
            0x08..=0x09 => todo!("NoMBC RAM/RAM+BATTERY"),

            // $01 MBC1
            // $02 MBC1+RAM
            // $03 MBC1+RAM+BATTERY
            0x01..=0x03 => Mbc1::new(rom)?.into(),

            // $05 MBC2
            // $06 MBC2+BATTERY
            0x05..=0x06 => Mbc2::new(rom)?.into(),

            // $0F MBC3+TIMER+BATTERY
            // $10 MBC3+TIMER+RAM+BATTERY
            // $11 MBC3
            // $12 MBC3+RAM
            // $13 MBC3+RAM+BATTERY
            0x0F..=0x13 => Mbc3::new(rom)?.into(),

            // $19 MBC5
            // $1A MBC5+RAM
            // $1B MBC5+RAM+BATTERY
            // $1C MBC5+RUMBLE
            // $1D MBC5+RUMBLE+RAM
            // $1E MBC5+RUMBLE+RAM+BATTERY
            0x19..=0x1E => Mbc5::new(rom)?.into(),

            code => return Err(self::Error::UnsupportedMbc { code }), // value => panic!("[cartridge.rs] Unsupported MBC: ({value:#04X})"),
        };

        Ok(mbc)
    }
}

mod error;
mod info;
mod mbc;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod no_mbc;
