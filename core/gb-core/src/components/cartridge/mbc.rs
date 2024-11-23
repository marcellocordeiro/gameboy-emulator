use enum_dispatch::enum_dispatch;

use self::{mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3, mbc5::Mbc5, mbc30::Mbc30, no_mbc::NoMbc};
use super::info::{Info, mbc_type::MbcType};

#[enum_dispatch]
pub(crate) trait MbcInterface {
    fn get_battery(&self) -> &[u8];
    fn load_battery(&mut self, file: Vec<u8>);

    fn read_rom_bank_0(&self, address: u16) -> u8;
    fn read_rom_bank_x(&self, address: u16) -> u8;

    fn read_ram(&self, address: u16) -> u8;

    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
}

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
    pub(crate) fn new(cartridge_info: &Info) -> Self {
        match cartridge_info.mbc_type {
            MbcType::NoMbc => NoMbc::new(cartridge_info).into(),
            MbcType::Mbc1 => Mbc1::new(cartridge_info).into(),
            MbcType::Mbc2 => Mbc2::new(cartridge_info).into(),
            MbcType::Mbc3 => Mbc3::new(cartridge_info).into(),
            MbcType::Mbc30 => Mbc30::new(cartridge_info).into(),
            MbcType::Mbc5 => Mbc5::new(cartridge_info).into(),
        }
    }
}

mod mbc1;
mod mbc2;
mod mbc3;
mod mbc30;
mod mbc5;
mod no_mbc;
