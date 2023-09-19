use enum_dispatch::enum_dispatch;

pub use self::{mbc1::Mbc1, mbc2::Mbc2, mbc3::Mbc3, mbc30::Mbc30, mbc5::Mbc5, no_mbc::NoMbc};

#[enum_dispatch]
pub(super) trait MbcInterface {
    fn reset(&mut self);

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
    Mbc30,
    Mbc5,
}

mod mbc1;
mod mbc2;
mod mbc3;
mod mbc30;
mod mbc5;
mod no_mbc;
