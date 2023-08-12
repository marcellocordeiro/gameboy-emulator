pub const ONE_KIB: usize = 0x400;

pub const ROM_BANK_SIZE: usize = 16 * ONE_KIB; // 0x4000
pub const RAM_BANK_SIZE: usize = 8 * ONE_KIB; // 0x2000

pub(super) trait Mbc {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;

    fn write_rom(&mut self, address: u16, value: u8);
    fn write_ram(&mut self, address: u16, value: u8);
}
