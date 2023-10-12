use crate::cpu::registers::Flags;

/// ADD
///
/// Add to accumulator
#[must_use]
pub fn add(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let result = accumulator.wrapping_add(value);

    let half_carry = ((accumulator & 0x0F) + (value & 0x0F)) > 0x0F;
    let carry = (accumulator as u16 + value as u16) > 0xFF;

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);

    result
}

/// ADC
///
/// Add to accumulator with carry
#[must_use]
pub fn adc(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let carry = f.contains(Flags::CARRY) as u8;

    let result = accumulator.wrapping_add(value).wrapping_add(carry);

    let half_carry = ((accumulator & 0x0F) + (value & 0x0F) + carry) > 0x0F;
    let carry = (accumulator as u16 + value as u16 + carry as u16) > 0xFF;

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);

    result
}

/// SUB
///
/// Sub from accumulator
#[must_use]
pub fn sub(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let result = accumulator.wrapping_sub(value);

    let half_carry = (accumulator & 0x0F) < (value & 0x0F);
    let carry = (accumulator as u16) < (value as u16);

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, true);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);

    result
}

/// SBC
///
/// Sub from accumulator with carry
#[must_use]
pub fn sbc(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let carry = f.contains(Flags::CARRY) as u8;

    let result = accumulator.wrapping_sub(value).wrapping_sub(carry);

    let half_carry = (accumulator & 0x0F) < (value & 0x0F) + carry;
    let carry = (accumulator as u16) < (value as u16) + (carry as u16);

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, true);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);

    result
}

/// CP
pub fn cp(f: &mut Flags, accumulator: u8, value: u8) {
    let result = accumulator.wrapping_sub(value);

    let half_carry = (accumulator & 0x0F) < (value & 0x0F);
    let carry = (accumulator as u16) < (value as u16);

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, true);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);
}

/// INC
#[must_use]
pub fn inc(f: &mut Flags, value: u8) -> u8 {
    let result = value.wrapping_add(1);

    let half_carry = (value & 0x0F) + 1 > 0x0F;

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, half_carry);

    result
}

/// DEC
#[must_use]
pub fn dec(f: &mut Flags, value: u8) -> u8 {
    let result = value.wrapping_sub(1);

    let half_carry = (value & 0x0F) == 0;

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, true);
    f.set(Flags::HALF_CARRY, half_carry);

    result
}

/// AND
#[must_use]
pub fn and(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let result = accumulator & value;

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, true);
    f.set(Flags::CARRY, false);

    result
}

/// OR
#[must_use]
pub fn or(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let result = accumulator | value;

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, false);

    result
}

/// XOR
#[must_use]
pub fn xor(f: &mut Flags, accumulator: u8, value: u8) -> u8 {
    let result = accumulator ^ value;

    // Store results.
    f.set(Flags::ZERO, result == 0);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, false);

    result
}

/// CCF
///
/// Complement carry flag
pub fn ccf(f: &mut Flags) {
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.toggle(Flags::CARRY);
}

/// SCF
///
/// Set carry flag
pub fn scf(f: &mut Flags) {
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, false);
    f.set(Flags::CARRY, true);
}

/// CPL
///
/// Complement accumulator
#[must_use]
pub fn cpl(f: &mut Flags, accumulator: u8) -> u8 {
    f.set(Flags::N_ADD_SUB, true);
    f.set(Flags::HALF_CARRY, true);

    !accumulator
}

/// DAA
///
/// Decimal adjust accumulator
#[must_use]
pub fn daa(f: &mut Flags, accumulator: u8) -> u8 {
    let n_add_sub = f.contains(Flags::N_ADD_SUB);
    let half_carry = f.contains(Flags::HALF_CARRY);
    let carry = f.contains(Flags::CARRY);

    let mut correction = 0u8;

    if half_carry || (!n_add_sub && (accumulator & 0x0F) > 0x09) {
        correction |= 0x06;
    }

    if carry || (!n_add_sub && (accumulator > 0x99)) {
        correction |= 0x60;
        f.set(Flags::CARRY, true);
    }

    let result = if n_add_sub {
        accumulator.wrapping_sub(correction)
    } else {
        accumulator.wrapping_add(correction)
    };

    f.set(Flags::ZERO, result == 0);
    f.set(Flags::HALF_CARRY, false);

    result
}

#[cfg(feature = "sm83-test-data")]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::cpu::alu::test_utils::parse_tests;

    #[test]
    fn test_add() {
        let tests = parse_tests("add");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = add(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_adc() {
        let tests = parse_tests("adc");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = adc(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_sub() {
        let tests = parse_tests("sub");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = sub(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_sbc() {
        let tests = parse_tests("sbc");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = sbc(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_cp() {
        let tests = parse_tests("cp");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            cp(&mut flags, test.x, test.y);

            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_and() {
        let tests = parse_tests("and");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = and(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_or() {
        let tests = parse_tests("or");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = or(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_xor() {
        let tests = parse_tests("xor");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = xor(&mut flags, test.x, test.y);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_ccf() {
        let tests = parse_tests("ccf");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            ccf(&mut flags);

            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_scf() {
        let tests = parse_tests("scf");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            scf(&mut flags);

            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_cpl() {
        let tests = parse_tests("cpl");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = cpl(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }

    #[test]
    fn test_daa() {
        let tests = parse_tests("daa");

        for (i, test) in tests.into_iter().enumerate() {
            let mut flags = Flags::from_bits_truncate(test.flags);
            let result = daa(&mut flags, test.x);

            assert_eq!(result, test.result.value, "Test #{i} failed.");
            assert_eq!(flags.bits(), test.result.flags, "Test #{i} failed.");
        }
    }
}
