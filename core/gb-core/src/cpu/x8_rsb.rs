use super::{registers::Flags, Cpu};

// Completed, may need some refactoring.

impl Cpu {
    /// RLCA
    pub(super) fn opcode_0x07(&mut self) {
        self.registers.a = self.alu_rotate_left_through_carry(self.registers.a);
        self.registers.f.set(Flags::ZERO, false);
    }

    /// RRCA
    pub(super) fn opcode_0x0f(&mut self) {
        self.registers.a = self.alu_rotate_right_through_carry(self.registers.a);
        self.registers.f.set(Flags::ZERO, false);
    }

    /// RLA
    pub(super) fn opcode_0x17(&mut self) {
        self.registers.a = self.alu_rotate_left(self.registers.a);
        self.registers.f.set(Flags::ZERO, false);
    }

    /// RRA
    pub(super) fn opcode_0x1f(&mut self) {
        self.registers.a = self.alu_rotate_right(self.registers.a);
        self.registers.f.set(Flags::ZERO, false);
    }
}
