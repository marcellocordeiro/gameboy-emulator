use super::{registers::Flags, Cpu};

impl Cpu {
    /// ADD
    ///
    /// Add to accumulator
    pub(super) fn alu_add(&mut self, value: u8) {
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

    /// ADC
    ///
    /// Add to accumulator with carry
    pub(super) fn alu_adc(&mut self, value: u8) {
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

    /// SUB
    ///
    /// Sub from accumulator
    pub(super) fn alu_sub(&mut self, value: u8) {
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

    /// SBC
    ///
    /// Sub from accumulator with carry
    pub(super) fn alu_sbc(&mut self, value: u8) {
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

    /// CP
    pub(super) fn alu_cp(&mut self, value: u8) {
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

    /// INC
    #[must_use]
    pub(super) fn alu_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        let half_carry = (value & 0x0F) + 1 > 0x0F;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);

        result
    }

    /// DEC
    #[must_use]
    pub(super) fn alu_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        let half_carry = (value & 0x0F) == 0;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);

        result
    }

    /// AND
    pub(super) fn alu_and(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator & value;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
        self.registers.f.set(Flags::CARRY, false);

        self.registers.a = result;
    }

    /// OR
    pub(super) fn alu_or(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator | value;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);

        self.registers.a = result;
    }

    /// XOR
    pub(super) fn alu_xor(&mut self, value: u8) {
        let accumulator = self.registers.a;

        let result = accumulator ^ value;

        // Store results.
        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);

        self.registers.a = result;
    }

    /// CCF
    ///
    /// Complement carry flag
    pub(super) fn alu_ccf(&mut self) {
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.toggle(Flags::CARRY);
    }

    /// SCF
    ///
    /// Set carry flag
    pub(super) fn alu_scf(&mut self) {
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, true);
    }

    /// CPL
    ///
    /// Complement accumulator
    pub(super) fn alu_cpl(&mut self) {
        self.registers.f.set(Flags::N_ADD_SUB, true);
        self.registers.f.set(Flags::HALF_CARRY, true);

        self.registers.a = !self.registers.a;
    }

    /// DAA
    ///
    /// Decimal adjust accumulator
    pub(super) fn alu_daa(&mut self) {
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
