use super::{registers::Flags, Cpu};

impl Cpu {
    // ADD
    pub(super) fn add_to_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;

        let result = accumulator.wrapping_add(value);

        let half_carry = ((accumulator & 0x0F) + (value & 0x0F)) > 0x0F;
        let carry = (accumulator as u16 + value as u16) > 0xFF;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.registers.accumulator = result;
    }

    pub(super) fn add_to_accumulator_with_carry(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;
        let carry = self.registers.flags.contains(Flags::CARRY) as u8;

        let result = accumulator.wrapping_add(value).wrapping_add(carry);

        let half_carry = ((accumulator & 0x0F) + (value & 0x0F) + carry) > 0x0F;
        let carry = (accumulator as u16 + value as u16 + carry as u16) > 0xFF;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.registers.accumulator = result;
    }

    // SUB
    pub(super) fn sub_from_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;

        let result = accumulator.wrapping_sub(value);

        let half_carry = (accumulator & 0x0F) < (value & 0x0F);
        let carry = (accumulator as u16) < (value as u16);

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, true);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.registers.accumulator = result;
    }

    pub(super) fn sub_from_accumulator_with_carry(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;
        let carry = self.registers.flags.contains(Flags::CARRY) as u8;

        let result = accumulator.wrapping_sub(value).wrapping_sub(carry);

        let half_carry = (accumulator & 0x0F) < (value & 0x0F) + carry;
        let carry = (accumulator as u16) < (value as u16) + (carry as u16);

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, true);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.registers.accumulator = result;
    }

    pub(super) fn cp_compare_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;

        let result = accumulator.wrapping_sub(value);

        let half_carry = (accumulator & 0x0F) < (value & 0x0F);
        let carry = (accumulator as u16) < (value as u16);

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, true);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);
    }

    // INC/DEC
    pub(super) fn inc_increment(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        let half_carry = (value & 0x0F) + 1 > 0x0F;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);

        result
    }

    pub(super) fn dec_decrement(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        let half_carry = (value & 0x0F) == 0;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, true);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);

        result
    }

    // Bitwise
    pub(super) fn and_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;

        let result = accumulator & value;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, true);
        self.registers.flags.set(Flags::CARRY, false);

        self.registers.accumulator = result;
    }

    pub(super) fn or_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;

        let result = accumulator | value;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, false);

        self.registers.accumulator = result;
    }

    pub(super) fn xor_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.accumulator;

        let result = accumulator ^ value;

        // Store results.
        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, false);

        self.registers.accumulator = result;
    }

    // Flags
    pub(super) fn ccf_complement_carry_flag(&mut self) {
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.toggle(Flags::CARRY);
    }

    pub(super) fn scf_set_carry_flag(&mut self) {
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, true);
    }

    pub(super) fn cpl_complement_accumulator(&mut self) {
        self.registers.flags.set(Flags::N_ADD_SUB, true);
        self.registers.flags.set(Flags::HALF_CARRY, true);

        self.registers.accumulator = !self.registers.accumulator;
    }

    // DAA
    // PAAIN.
    pub(super) fn daa_decimal_adjust_accumulator(&mut self) {
        let accumulator = self.registers.accumulator;

        let n_add_sub = self.registers.flags.contains(Flags::N_ADD_SUB);
        let half_carry = self.registers.flags.contains(Flags::HALF_CARRY);
        let carry = self.registers.flags.contains(Flags::CARRY);

        let mut correction = 0u8;

        if half_carry || (!n_add_sub && (accumulator & 0x0F) > 0x09) {
            correction |= 0x06;
        }

        if carry || (!n_add_sub && (accumulator > 0x99)) {
            correction |= 0x60;
            self.registers.flags.set(Flags::CARRY, true);
        }

        if n_add_sub {
            self.registers.accumulator = accumulator.wrapping_sub(correction);
        } else {
            self.registers.accumulator = accumulator.wrapping_add(correction);
        }

        self.registers
            .flags
            .set(Flags::ZERO, self.registers.accumulator == 0);
        self.registers.flags.set(Flags::HALF_CARRY, false);
    }
}
