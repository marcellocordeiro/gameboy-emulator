use super::{alu, registers::Flags, Cpu};

// Completed, may need some refactoring.

macro_rules! alu_op_a {
    ($self:ident, $F:ident) => {
        $self.registers.a = alu::$F(&mut $self.registers.f, $self.registers.a);
        $self.registers.f.set(Flags::ZERO, false);
    };
}

impl Cpu {
    /// RLCA
    pub(super) fn opcode_0x07(&mut self) {
        alu_op_a!(self, rlc);
    }

    /// RRCA
    pub(super) fn opcode_0x0f(&mut self) {
        alu_op_a!(self, rrc);
    }

    /// RLA
    pub(super) fn opcode_0x17(&mut self) {
        alu_op_a!(self, rl);
    }

    /// RRA
    pub(super) fn opcode_0x1f(&mut self) {
        alu_op_a!(self, rr);
    }
}
