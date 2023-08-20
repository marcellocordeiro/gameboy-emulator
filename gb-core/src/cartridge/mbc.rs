use enum_dispatch::enum_dispatch;

use crate::cartridge::{Mbc1, Mbc2, Mbc3, Mbc5, NoMbc};

#[enum_dispatch]
pub(super) trait MbcInterface {
    fn read_rom(&self, address: u16) -> u8;
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
    Mbc5,
}
