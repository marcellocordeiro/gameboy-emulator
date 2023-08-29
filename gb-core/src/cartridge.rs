use self::{
    info::{CartridgeType, CgbFlag, Info},
    mbc::{Mbc, Mbc1, Mbc2, Mbc3, Mbc5, MbcInterface, NoMbc},
};

pub use self::error::Error;

pub struct Cartridge {
    mbc: Mbc,
}

impl Cartridge {
    pub fn try_new(rom: Vec<u8>) -> Result<Self, self::Error> {
        let info = Info::try_from(&rom)?;

        if info.cgb_flag == CgbFlag::CgbOnly {
            todo!("CGB mode not yet implemented.");
        }

        let mbc = Self::get_mbc(rom, &info)?;

        Ok(Self { mbc })
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.mbc.read_rom(address)
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(address)
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc.write_rom(address, value);
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.mbc.write_ram(address, value);
    }

    fn get_mbc(rom: Vec<u8>, info: &Info) -> Result<Mbc, self::Error> {
        let mbc: Mbc = match info.cartridge_type {
            CartridgeType::NoMbc => NoMbc::new(rom, info).into(),
            CartridgeType::Mbc1 => Mbc1::new(rom, info).into(),
            CartridgeType::Mbc2 => Mbc2::new(rom, info).into(),
            CartridgeType::Mbc3 => Mbc3::new(rom, info).into(),
            CartridgeType::Mbc5 => Mbc5::new(rom, info).into(),

            cartridge_type => return Err(self::Error::UnsupportedMbc { cartridge_type }),
        };

        Ok(mbc)
    }
}

mod error;
mod info;
mod mbc;
