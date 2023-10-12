use super::{registers::Flags, Cpu};

impl Cpu {
    /// RL
    #[must_use]
    pub(super) fn alu_rotate_left(&mut self, value: u8) -> u8 {
        let carry = self.registers.f.contains(Flags::CARRY) as u8;
        let will_carry = (value & (1 << 7)) != 0;

        let result = (value << 1) | carry;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    /// RR
    #[must_use]
    pub(super) fn alu_rotate_right(&mut self, value: u8) -> u8 {
        let carry = self.registers.f.contains(Flags::CARRY) as u8;
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | (carry << 7);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    /// RLC
    #[must_use]
    pub(super) fn alu_rotate_left_through_carry(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 7)) != 0;

        let result = value.rotate_left(1);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    /// RRC
    #[must_use]
    pub(super) fn alu_rotate_right_through_carry(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 0)) != 0;

        let result = value.rotate_right(1);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    /// SRL
    #[must_use]
    pub(super) fn alu_shift_right_logical(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 0)) != 0;

        let result = value >> 1;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    /// SRA
    ///
    /// Bit 7 is unchanged.
    #[must_use]
    pub(super) fn alu_shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let original_msb = value & (1 << 7);
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | original_msb;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    /// SLA
    #[must_use]
    pub(super) fn alu_shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 7)) != 0;

        let result = value << 1;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // BIT
    pub(super) fn alu_test_bit(&mut self, bit: usize, value: u8) {
        let result = value & (1 << bit);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
    }

    // RES
    #[must_use]
    pub(super) fn alu_reset_bit(&self, bit: usize, value: u8) -> u8 {
        value & !(1 << bit)
    }

    // SET
    #[must_use]
    pub(super) fn alu_set_bit(&self, bit: usize, value: u8) -> u8 {
        value | (1 << bit)
    }

    // SWAP
    #[must_use]
    pub(super) fn alu_swap_nibbles(&mut self, value: u8) -> u8 {
        let low = value & 0x0F;
        let high = (value & 0xF0) >> 4;

        let result = (low << 4) | high;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);

        result
    }
}
