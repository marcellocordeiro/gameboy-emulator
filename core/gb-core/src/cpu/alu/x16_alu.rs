use crate::cpu::registers::Flags;

#[must_use]
pub fn add_to_sp(f: &mut Flags, stack_pointer: u16, value: i16) -> u16 {
    let result = stack_pointer.wrapping_add_signed(value);

    let half_carry = (stack_pointer & 0x000F).wrapping_add_signed(value & 0x000F) > 0x000F;
    let carry = (stack_pointer & 0x00FF).wrapping_add_signed(value & 0x00FF) > 0x00FF;

    f.set(Flags::ZERO, false);
    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);

    result
}

#[must_use]
pub fn add_to_hl(f: &mut Flags, hl: u16, value: u16) -> u16 {
    let result = hl.wrapping_add(value);

    let half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
    let carry = hl > (0xFFFF - value);

    f.set(Flags::N_ADD_SUB, false);
    f.set(Flags::HALF_CARRY, half_carry);
    f.set(Flags::CARRY, carry);

    result
}
