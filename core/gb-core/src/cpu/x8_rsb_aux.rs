use super::{registers::Flags, Cpu};

impl Cpu {
    // RL
    #[must_use]
    pub(super) fn bit_rotate_left(&mut self, value: u8) -> u8 {
        let carry = self.registers.f.contains(Flags::CARRY) as u8;
        let will_carry = (value & (1 << 7)) != 0;

        let result = (value << 1) | carry;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // RR
    #[must_use]
    pub(super) fn bit_rotate_right(&mut self, value: u8) -> u8 {
        let carry = self.registers.f.contains(Flags::CARRY) as u8;
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | (carry << 7);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // RLC
    #[must_use]
    pub(super) fn bit_rotate_left_c(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 7)) != 0;

        let result = (value << 1) | (will_carry as u8);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // RRC
    #[must_use]
    pub(super) fn bit_rotate_right_c(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | ((will_carry as u8) << 7);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // SRL
    #[must_use]
    pub(super) fn bit_srl_logical_shift_right(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 0)) != 0;

        let result = value >> 1;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // SRA
    #[must_use]
    pub(super) fn bit_sra_arithmetic_shift_right(&mut self, value: u8) -> u8 {
        let original_msb = value & (1 << 7);
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | original_msb;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // SLA
    #[must_use]
    pub(super) fn bit_sla_arithmetic_shift_left(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 7)) != 0;

        let result = value << 1;

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, will_carry);

        result
    }

    // BIT
    pub(super) fn bit_test_bit(&mut self, bit: usize, value: u8) {
        let result = value & (1 << bit);

        self.registers.f.set(Flags::ZERO, result == 0);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
    }

    // RES
    #[must_use]
    pub(super) fn bit_reset_bit(&self, bit: usize, value: u8) -> u8 {
        value & !(1 << bit)
    }

    // SET
    #[must_use]
    pub(super) fn bit_set_bit(&self, bit: usize, value: u8) -> u8 {
        value | (1 << bit)
    }

    // SWAP
    #[must_use]
    pub(super) fn swap_nibbles(&mut self, value: u8) -> u8 {
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
