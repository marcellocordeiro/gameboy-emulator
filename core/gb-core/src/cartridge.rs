use self::{
    error::Error as CartridgeError,
    info::{CartridgeType, Info},
    mbc::{Mbc, Mbc1, Mbc2, Mbc3, Mbc30, Mbc5, MbcInterface, NoMbc},
};

pub struct Cartridge {
    pub info: Info,
    mbc: Mbc,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self, CartridgeError> {
        let info = Info::try_from(&rom)?;

        // Panics if the validation fails.
        info.validate();

        let mbc = Self::get_mbc(rom, &info)?;

        Ok(Self { info, mbc })
    }

    pub fn reset(&mut self) {
        self.mbc.reset();
    }

    pub fn get_battery(&self) -> &[u8] {
        self.mbc.get_battery()
    }

    pub fn load_battery(&mut self, file: Vec<u8>) {
        self.mbc.load_battery(file);
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

    fn get_mbc(rom: Vec<u8>, info: &Info) -> Result<Mbc, CartridgeError> {
        let mbc: Mbc = match info.cartridge_type {
            CartridgeType::NoMbc => NoMbc::new(rom, info).into(),
            CartridgeType::Mbc1 => Mbc1::new(rom, info).into(),
            CartridgeType::Mbc2 => Mbc2::new(rom, info).into(),
            CartridgeType::Mbc3 => Mbc3::new(rom, info).into(),
            CartridgeType::Mbc30 => Mbc30::new(rom, info).into(),
            CartridgeType::Mbc5 => Mbc5::new(rom, info).into(),

            cartridge_type => return Err(CartridgeError::UnsupportedMbc { cartridge_type }),
        };

        Ok(mbc)
    }
}

pub mod error;
pub mod info;
mod mbc;
