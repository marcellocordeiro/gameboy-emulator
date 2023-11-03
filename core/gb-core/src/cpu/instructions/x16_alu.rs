use crate::{
    cpu::{alu, Cpu},
    memory::Memory,
};

// Completed, may need some refactoring.

impl Cpu {
    /// INC BC
    pub(super) fn opcode_0x03(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers
            .set_bc(self.registers.get_bc().wrapping_add(1));
    }

    /// ADD HL,BC
    pub(super) fn opcode_0x09(&mut self, memory: &mut Memory) {
        self.tick(memory);

        let hl = self.registers.get_hl();
        let value = self.registers.get_bc();

        let result = alu::add_to_hl(&mut self.registers.f, hl, value);

        self.registers.set_hl(result);
    }

    /// DEC BC
    pub(super) fn opcode_0x0b(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers
            .set_bc(self.registers.get_bc().wrapping_sub(1));
    }

    /// INC DE
    pub(super) fn opcode_0x13(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers
            .set_de(self.registers.get_de().wrapping_add(1));
    }

    /// ADD HL,DE
    pub(super) fn opcode_0x19(&mut self, memory: &mut Memory) {
        self.tick(memory);

        let hl = self.registers.get_hl();
        let value = self.registers.get_de();

        let result = alu::add_to_hl(&mut self.registers.f, hl, value);

        self.registers.set_hl(result);
    }

    /// DEC DE
    pub(super) fn opcode_0x1b(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers
            .set_de(self.registers.get_de().wrapping_sub(1));
    }

    /// INC HL
    pub(super) fn opcode_0x23(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers
            .set_hl(self.registers.get_hl().wrapping_add(1));
    }

    /// ADD HL,HL
    pub(super) fn opcode_0x29(&mut self, memory: &mut Memory) {
        self.tick(memory);

        let hl = self.registers.get_hl();
        let value = hl;

        let result = alu::add_to_hl(&mut self.registers.f, hl, value);

        self.registers.set_hl(result);
    }

    /// DEC HL
    pub(super) fn opcode_0x2b(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers
            .set_hl(self.registers.get_hl().wrapping_sub(1));
    }

    /// INC SP
    pub(super) fn opcode_0x33(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers.sp = self.registers.sp.wrapping_add(1);
    }

    /// ADD HL,SP
    pub(super) fn opcode_0x39(&mut self, memory: &mut Memory) {
        self.tick(memory);

        let hl = self.registers.get_hl();
        let value = self.registers.sp;

        let result = alu::add_to_hl(&mut self.registers.f, hl, value);

        self.registers.set_hl(result);
    }

    /// DEC SP
    pub(super) fn opcode_0x3b(&mut self, memory: &mut Memory) {
        self.tick(memory);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    /// ADD SP,i8
    pub(super) fn opcode_0xe8(&mut self, memory: &mut Memory) {
        let value = self.read_byte_operand(memory) as i8;

        self.tick(memory);
        self.tick(memory);

        self.registers.sp = alu::add_to_sp(&mut self.registers.f, self.registers.sp, value.into());
    }

    /// LD HL,SP+i8
    pub(super) fn opcode_0xf8(&mut self, memory: &mut Memory) {
        let value = self.read_byte_operand(memory) as i8;

        let result = alu::add_to_sp(&mut self.registers.f, self.registers.sp, value.into());

        self.tick(memory);

        self.registers.set_hl(result);
    }
}
