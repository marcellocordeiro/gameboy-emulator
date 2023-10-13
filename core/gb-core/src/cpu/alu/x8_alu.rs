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
