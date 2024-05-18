use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use self::{mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3, mbc30::Mbc30, mbc5::Mbc5, no_mbc::NoMbc};
use crate::cartridge_info::{CartridgeInfo, MbcType};

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
    pub(crate) fn new(info: &CartridgeInfo, rom: Arc<Box<[u8]>>) -> Self {
        match info.mbc_type {
            MbcType::NoMbc => NoMbc::new(info, rom).into(),
            MbcType::Mbc1 => Mbc1::new(info, rom).into(),
            MbcType::Mbc2 => Mbc2::new(info, rom).into(),
            MbcType::Mbc3 => Mbc3::new(info, rom).into(),
            MbcType::Mbc30 => Mbc30::new(info, rom).into(),
            MbcType::Mbc5 => Mbc5::new(info, rom).into(),
        }
    }
}

mod mbc1;
mod mbc2;
mod mbc3;
mod mbc30;
mod mbc5;
mod no_mbc;
