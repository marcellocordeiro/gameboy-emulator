pub use self::{mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3, mbc30::Mbc30, mbc5::Mbc5, no_mbc::NoMbc};
use super::{
    error::Error as CartridgeError,
    info::{CartridgeType, Info},
};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(super) trait MbcInterface {
    fn reset(&mut self);

    fn get_battery(&self) -> &[u8];
    fn load_battery(&mut self, file: Vec<u8>);

    fn read_rom_bank_0(&self, address: u16) -> u8;
    fn read_rom_bank_x(&self, address: u16) -> u8;

    fn read_ram(&self, address: u16) -> u8;

    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
}

#[allow(clippy::enum_variant_names)]
#[enum_dispatch(MbcInterface)]
pub enum Mbc {
    NoMbc,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc30,
    Mbc5,
}

impl Mbc {
    pub(crate) fn try_new(rom: Vec<u8>, info: &Info) -> Result<Self, CartridgeError> {
        Ok(match info.cartridge_type {
            CartridgeType::NoMbc => NoMbc::new(rom, info).into(),
            CartridgeType::Mbc1 => Mbc1::new(rom, info).into(),
            CartridgeType::Mbc2 => Mbc2::new(rom, info).into(),
            CartridgeType::Mbc3 => Mbc3::new(rom, info).into(),
            CartridgeType::Mbc30 => Mbc30::new(rom, info).into(),
            CartridgeType::Mbc5 => Mbc5::new(rom, info).into(),

            cartridge_type => return Err(CartridgeError::UnsupportedMbc { cartridge_type }),
        })
    }
}

mod mbc1;
mod mbc2;
mod mbc3;
mod mbc30;
mod mbc5;
mod no_mbc;
