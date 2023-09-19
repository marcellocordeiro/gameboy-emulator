pub const SGB_FLAG_ADDRESS: usize = 0x0146;

pub fn from(value: u8) -> bool {
    value == 0x03
}
