use super::{registers::Flags, Cpu};

impl Cpu {
    // ADD
    pub(super) fn add_to_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator.wrapping_add(value);

        let half_carry = ((accumulator & 0x0F) + (value & 0x0F)) > 0x0F;
        let carry = (accumulator as u16 + value as u16) > 0xFF;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.f.set(Flags::CARRY, carry);

        self.registers.a = result;
    }

    pub(super) fn add_to_accumulator_with_carry(&mut self, value: u8) {
        let accumulator = self.registers.a;
        let carry = self.registers.f.contains(Flags::CARRY) as u8;

        let result = accumulator.wrapping_add(value).wrapping_add(carry);

        let half_carry = ((accumulator & 0x0F) + (value & 0x0F) + carry) > 0x0F;
        let carry = (accumulator as u16 + value as u16 + carry as u16) > 0xFF;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.f.set(Flags::CARRY, carry);

        self.registers.a = result;
    }

    // SUB
    pub(super) fn sub_from_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator.wrapping_sub(value);

        let half_carry = (accumulator & 0x0F) < (value & 0x0F);
        let carry = (accumulator as u16) < (value as u16);

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.f.set(Flags::CARRY, carry);

        self.registers.a = result;
    }

    pub(super) fn sub_from_accumulator_with_carry(&mut self, value: u8) {
        let accumulator = self.registers.a;
        let carry = self.registers.f.contains(Flags::CARRY) as u8;

        let result = accumulator.wrapping_sub(value).wrapping_sub(carry);

        let half_carry = (accumulator & 0x0F) < (value & 0x0F) + carry;
        let carry = (accumulator as u16) < (value as u16) + (carry as u16);

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.f.set(Flags::CARRY, carry);

        self.registers.a = result;
    }

    pub(super) fn cp_compare_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator.wrapping_sub(value);

        let half_carry = (accumulator & 0x0F) < (value & 0x0F);
        let carry = (accumulator as u16) < (value as u16);

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.f.set(Flags::CARRY, carry);
    }

    // INC/DEC
    #[must_use]
    pub(super) fn inc_increment(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        let half_carry = (value & 0x0F) + 1 > 0x0F;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);

        result
    }

    #[must_use]
    pub(super) fn dec_decrement(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        let half_carry = (value & 0x0F) == 0;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);

        result
    }

    // Bitwise
    pub(super) fn and_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator & value;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
        self.registers.f.set(Flags::CARRY, false);

        self.registers.a = result;
    }

    pub(super) fn or_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator | value;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);

        self.registers.a = result;
    }

    pub(super) fn xor_with_accumulator(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator ^ value;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);

        self.registers.a = result;
    }

    // Flags
    pub(super) fn ccf_complement_carry_flag(&mut self) {
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.toggle(Flags::CARRY);
    }

    pub(super) fn scf_set_carry_flag(&mut self) {
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, true);
    }

    pub(super) fn cpl_complement_accumulator(&mut self) {
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, true);

        self.registers.a = !self.registers.a;
    }

    // DAA
    // PAAIN.
    pub(super) fn daa_decimal_adjust_accumulator(&mut self) {
        let accumulator = self.registers.a;

        let n_add_sub = self.registers.f.contains(Flags::N_ADD_SUB);
        let half_carry = self.registers.f.contains(Flags::HALF_CARRY);
        let carry = self.registers.f.contains(Flags::CARRY);

        let mut correction = 0u8;

        if half_carry || (!n_add_sub && (accumulator & 0x0F) > 0x09) {
            correction |= 0x06;
        }

        if carry || (!n_add_sub && (accumulator > 0x99)) {
            correction |= 0x60;
            self.registers.f.set(Flags::CARRY, true);
        }

        let result = if n_add_sub {
            accumulator.wrapping_sub(correction)
        } else {
            accumulator.wrapping_add(correction)
        };

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::HALF_CARRY, false);

        self.registers.a = result;
    }
}
