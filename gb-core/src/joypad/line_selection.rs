#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LineSelection {
    Both = 0b00,
    Action = 0b01,
    Direction = 0b10,
    None = 0b11,
}

impl LineSelection {
    pub fn from_joyp_bits(value: u8) -> Self {
        use LineSelection::*;

        match (value >> 4) & 0b11 {
            0b00 => Both,
            0b01 => Action,
            0b10 => Direction,
            0b11 => None,

            _ => unreachable!(),
        }
    }

    pub fn to_joyp_bits(self) -> u8 {
        ((self as u8) << 4) & 0b0011_0000
    }
}
