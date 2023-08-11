use super::{registers::Flags, Cpu};

impl Cpu {
    pub(super) fn add_to_stack_pointer(&mut self, offset: i16) {
        let stack_pointer = self.registers.stack_pointer;

        let result = stack_pointer.wrapping_add_signed(offset);

        let half_carry = (stack_pointer & 0x000F).wrapping_add_signed(offset & 0x000F) > 0x000F;
        let carry = (stack_pointer & 0x00FF).wrapping_add_signed(offset & 0x00FF) > 0x00FF;

        // Store results.
        self.registers.flags.set(Flags::ZERO, false);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.registers.stack_pointer = result;
    }

    pub(super) fn add_to_hl(&mut self, value: u16) {
        let hl = self.registers.get_hl();

        let result = hl.wrapping_add(value);

        let half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
        let carry = hl > (0xFFFF - value);

        // Store results.
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.registers.set_hl(result);
    }
}
