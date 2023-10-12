use crate::cpu::registers::Flags;

/// RL / Rotate Left
#[must_use]
pub fn rl(f: &mut Flags, value: u8) -> u8 {
    let carry = f.contains(Flags::CARRY) as u8;
    let will_carry = (value & (1 << 7)) != 0;

    let result = (value << 1) | carry;

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// RR / Rotate Right
#[must_use]
pub fn rr(f: &mut Flags, value: u8) -> u8 {
    let carry = f.contains(Flags::CARRY) as u8;
    let will_carry = (value & (1 << 0)) != 0;

    let result = (value >> 1) | (carry << 7);

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// RLC / Rotate Left through Carry
#[must_use]
pub fn rlc(f: &mut Flags, value: u8) -> u8 {
    let will_carry = (value & (1 << 7)) != 0;

    let result = value.rotate_left(1);

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// RRC / Rotate Right through Carry
#[must_use]
pub fn rrc(f: &mut Flags, value: u8) -> u8 {
    let will_carry = (value & (1 << 0)) != 0;

    let result = value.rotate_right(1);

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// SRL / Shift Right Logical
#[must_use]
pub fn srl(f: &mut Flags, value: u8) -> u8 {
    let will_carry = (value & (1 << 0)) != 0;

    let result = value >> 1;

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// SRA / Shift Right Arithmetic
///
/// Bit 7 is unchanged.
#[must_use]
pub fn sra(f: &mut Flags, value: u8) -> u8 {
    let original_msb = value & (1 << 7);
    let will_carry = (value & (1 << 0)) != 0;

    let result = (value >> 1) | original_msb;

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// SLA / Shift Left Arithmetic
#[must_use]
pub fn sla(f: &mut Flags, value: u8) -> u8 {
    let will_carry = (value & (1 << 7)) != 0;

    let result = value << 1;

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, will_carry);

    result
}

/// BIT / Test bit
pub fn bit(f: &mut Flags, bit: usize, value: u8) {
    let result = value & (1 << bit);

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, true);
}

/// RES / Reset bit
#[must_use]
pub fn res(_f: &mut Flags, bit: usize, value: u8) -> u8 {
    value & !(1 << bit)
}

/// SET / Set bit
#[must_use]
pub fn set(_f: &mut Flags, bit: usize, value: u8) -> u8 {
    value | (1 << bit)
}

/// SWAP / Swap nibbles
#[must_use]
pub fn swap(f: &mut Flags, value: u8) -> u8 {
    let low = value & 0x0F;
    let high = (value & 0xF0) >> 4;

    let result = (low << 4) | high;

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, false);

    result
}

#[cfg_attr(feature = "sm83-test-data", cfg(test))]
mod tests {
    use super::*;

    use crate::cpu::alu::test_utils::parse_tests;

    #[test]
    fn test_rl() {
        let tests = parse_tests("rl");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = rl(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_rr() {
        let tests = parse_tests("rr");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = rr(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_rlc() {
        let tests = parse_tests("rlc");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = rlc(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_rrc() {
        let tests = parse_tests("rrc");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = rrc(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_srl() {
        let tests = parse_tests("srl");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = srl(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_sra() {
        let tests = parse_tests("sra");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = sra(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_sla() {
        let tests = parse_tests("sla");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = sla(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_bit() {
        let tests = parse_tests("bit");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            bit(&mut flags, test.y as usize, test.x);

            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_res() {
        let tests = parse_tests("res");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = res(&mut flags, test.y as usize, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_set() {
        let tests = parse_tests("set");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = set(&mut flags, test.y as usize, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_swap() {
        let tests = parse_tests("swap");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = swap(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }
}
