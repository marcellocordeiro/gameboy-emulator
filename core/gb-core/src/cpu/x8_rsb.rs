use super::{registers::Flags, Cpu};

// Completed, may need some refactoring.

impl Cpu {
    pub(super) fn opcode_0x07(&mut self) {
        // RLCA
        self.registers.accumulator = self.bit_rotate_left_c(self.registers.accumulator);
        self.registers.flags.set(Flags::ZERO, false);
    }

    pub(super) fn opcode_0x0f(&mut self) {
        // RRCA
        self.registers.accumulator = self.bit_rotate_right_c(self.registers.accumulator);
        self.registers.flags.set(Flags::ZERO, false);
    }

    pub(super) fn opcode_0x17(&mut self) {
        // RLA
        self.registers.accumulator = self.bit_rotate_left(self.registers.accumulator);
        self.registers.flags.set(Flags::ZERO, false);
    }

    pub(super) fn opcode_0x1f(&mut self) {
        // RRA
        self.registers.accumulator = self.bit_rotate_right(self.registers.accumulator);
        self.registers.flags.set(Flags::ZERO, false);
    }
}
