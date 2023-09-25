pub const JOYP_SELECTION_MASK: u8 = 0b0011_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LineSelection {
    Both = 0b00,
    Action = 0b01,
    Direction = 0b10,
    None = 0b11,
}

impl LineSelection {
    pub fn from_joyp_bits(value: u8) -> Self {
        match (value & JOYP_SELECTION_MASK) >> 4 {
            0b00 => Self::Both,
            0b01 => Self::Action,
            0b10 => Self::Direction,
            0b11 => Self::None,

            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JOYP_SELECTION_MASK: u8 = 0b0011_0000;

    #[test]
    fn test_both_selection_from_joyp() {
        for joyp in 0..u8::MAX {
            let mask = 0b0000_0000;

            assert_eq!(
                LineSelection::Both,
                LineSelection::from_joyp_bits((joyp & !JOYP_SELECTION_MASK) | mask),
                "with joyp = {:#010b}",
                (joyp | mask)
            );
        }
    }

    #[test]
    fn test_action_selection_from_joyp() {
        for joyp in 0..u8::MAX {
            let mask = 0b0001_0000;

            assert_eq!(
                LineSelection::Action,
                LineSelection::from_joyp_bits((joyp & !JOYP_SELECTION_MASK) | mask),
                "with joyp = {:#010b}",
                (joyp | mask)
            );
        }
    }

    #[test]
    fn test_direction_selection_from_joyp() {
        for joyp in 0..u8::MAX {
            let mask = 0b0010_0000;

            assert_eq!(
                LineSelection::Direction,
                LineSelection::from_joyp_bits((joyp & !JOYP_SELECTION_MASK) | mask),
                "with joyp = {:#010b}",
                (joyp | mask)
            );
        }
    }

    #[test]
    fn test_none_selection_from_joyp() {
        for joyp in 0..u8::MAX {
            let mask = 0b0011_0000;

            assert_eq!(
                LineSelection::None,
                LineSelection::from_joyp_bits((joyp & !JOYP_SELECTION_MASK) | mask),
                "with joyp = {:#010b}",
                (joyp | mask)
            );
        }
    }
}
