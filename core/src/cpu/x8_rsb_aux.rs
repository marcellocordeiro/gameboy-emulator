use super::{registers::Flags, Cpu};

impl Cpu {
    // RL
    pub(super) fn bit_rotate_left(&mut self, value: u8) -> u8 {
        let carry = self.registers.flags.contains(Flags::CARRY) as u8;
        let will_carry = (value & (1 << 7)) != 0;

        let result = (value << 1) | carry;

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // RR
    pub(super) fn bit_rotate_right(&mut self, value: u8) -> u8 {
        let carry = self.registers.flags.contains(Flags::CARRY) as u8;
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | (carry << 7);

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // RLC
    pub(super) fn bit_rotate_left_c(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 7)) != 0;

        let result = (value << 1) | (will_carry as u8);

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // RRC
    pub(super) fn bit_rotate_right_c(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | ((will_carry as u8) << 7);

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // SRL
    pub(super) fn bit_srl_logical_shift_right(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 0)) != 0;

        let result = value >> 1;

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // SRA
    pub(super) fn bit_sra_arithmetic_shift_right(&mut self, value: u8) -> u8 {
        let original_msb = value & (1 << 7);
        let will_carry = (value & (1 << 0)) != 0;

        let result = (value >> 1) | original_msb;

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // SLA
    pub(super) fn bit_sla_arithmetic_shift_left(&mut self, value: u8) -> u8 {
        let will_carry = (value & (1 << 7)) != 0;

        let result = value << 1;

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, will_carry);

        result
    }

    // BIT
    pub(super) fn bit_test_bit(&mut self, bit: usize, value: u8) {
        let result = value & (1 << bit);

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, true);
    }

    // RES
    pub(super) fn bit_reset_bit(&self, bit: usize, value: u8) -> u8 {
        value & !(1 << bit)
    }

    // SET
    pub(super) fn bit_set_bit(&self, bit: usize, value: u8) -> u8 {
        value | (1 << bit)
    }

    // SWAP
    pub(super) fn swap_nibbles(&mut self, value: u8) -> u8 {
        let low = value & 0x0F;
        let high = (value & 0xF0) >> 4;

        let result = (low << 4) | high;

        self.registers.flags.set(Flags::ZERO, result == 0);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, false);
        self.registers.flags.set(Flags::CARRY, false);

        result
    }
}
