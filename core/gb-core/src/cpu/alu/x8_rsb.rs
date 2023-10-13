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
