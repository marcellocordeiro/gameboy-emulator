use self::{mbc::Mbc, mbc1::Mbc1, mbc3::Mbc3, mbc5::Mbc5, no_mbc::NoMbc};

#[derive(Default)]
pub struct Cartridge {
    mbc: Option<Box<dyn Mbc>>,
}

impl Cartridge {
    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.mbc = Some(Self::get_mbc(rom));
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        if let Some(ref mbc) = self.mbc {
            mbc.read_rom(address)
        } else {
            0xFF
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if let Some(ref mbc) = self.mbc {
            mbc.read_ram(address)
        } else {
            0xFF
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        if let Some(ref mut mbc) = self.mbc {
            mbc.write_rom(address, value);
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if let Some(ref mut mbc) = self.mbc {
            mbc.write_ram(address, value);
        }
    }

    // todo: make this failable
    fn get_mbc(rom: Vec<u8>) -> Box<dyn Mbc> {
        let mbc: Box<dyn Mbc> = match rom[0x147] {
            0x00 => Box::new(NoMbc::new(rom)),
            0x01..=0x03 => Box::new(Mbc1::new(rom)),

            0x11..=0x13 => Box::new(Mbc3::new(rom)),

            0x19..=0x1E => Box::new(Mbc5::new(rom)),

            value => panic!("[cartridge.rs] MBC not implemented yet: ({value:#04X})"),
        };

        mbc
    }
}

mod mbc;
mod mbc1;
mod mbc3;
mod mbc5;
mod no_mbc;
