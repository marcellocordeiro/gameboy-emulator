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

#[cfg(test)]
mod tests {
    use super::*;

    const SELECTION_MASK: u8 = 0b0011_0000;

    #[test]
    fn test_both_selection_from_joyp() {
        for joyp in 0..u8::MAX {
            let mask = 0b0000_0000;

            assert_eq!(
                LineSelection::Both,
                LineSelection::from_joyp_bits((joyp & !SELECTION_MASK) | mask),
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
                LineSelection::from_joyp_bits((joyp & !SELECTION_MASK) | mask),
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
                LineSelection::from_joyp_bits((joyp & !SELECTION_MASK) | mask),
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
                LineSelection::from_joyp_bits((joyp & !SELECTION_MASK) | mask),
                "with joyp = {:#010b}",
                (joyp | mask)
            );
        }
    }

    #[test]
    fn test_both_selection_to_joyp() {
        let joyp = 0b0000_0000;
        assert_eq!(joyp, LineSelection::Both.to_joyp_bits());
    }

    #[test]
    fn test_action_selection_to_joyp() {
        let joyp = 0b0001_0000;
        assert_eq!(joyp, LineSelection::Action.to_joyp_bits());
    }

    #[test]
    fn test_direction_selection_to_joyp() {
        let joyp = 0b0010_0000;
        assert_eq!(joyp, LineSelection::Direction.to_joyp_bits());
    }

    #[test]
    fn test_none_selection_to_joyp() {
        let joyp = 0b0011_0000;
        assert_eq!(joyp, LineSelection::None.to_joyp_bits());
    }
}
