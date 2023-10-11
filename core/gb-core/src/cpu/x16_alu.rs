use super::{registers::Flags, Cpu};

// Completed, may need some refactoring.

impl Cpu {
    /// INC BC
    pub(super) fn opcode_0x03(&mut self) {
        self.tick();

        self.registers
            .set_bc(self.registers.get_bc().wrapping_add(1));
    }

    /// ADD HL,BC
    pub(super) fn opcode_0x09(&mut self) {
        self.tick();

        self.add_to_hl(self.registers.get_bc());
    }

    /// DEC BC
    pub(super) fn opcode_0x0b(&mut self) {
        self.tick();

        self.registers
            .set_bc(self.registers.get_bc().wrapping_sub(1));
    }

    /// INC DE
    pub(super) fn opcode_0x13(&mut self) {
        self.tick();

        self.registers
            .set_de(self.registers.get_de().wrapping_add(1));
    }

    /// ADD HL,DE
    pub(super) fn opcode_0x19(&mut self) {
        self.tick();

        self.add_to_hl(self.registers.get_de());
    }

    /// DEC DE
    pub(super) fn opcode_0x1b(&mut self) {
        self.tick();

        self.registers
            .set_de(self.registers.get_de().wrapping_sub(1));
    }

    /// INC HL
    pub(super) fn opcode_0x23(&mut self) {
        self.tick();

        self.registers
            .set_hl(self.registers.get_hl().wrapping_add(1));
    }

    /// ADD HL,HL
    pub(super) fn opcode_0x29(&mut self) {
        self.tick();

        self.add_to_hl(self.registers.get_hl());
    }

    /// DEC HL
    pub(super) fn opcode_0x2b(&mut self) {
        self.tick();

        self.registers
            .set_hl(self.registers.get_hl().wrapping_sub(1));
    }

    /// INC SP
    pub(super) fn opcode_0x33(&mut self) {
        self.tick();

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
    }

    /// ADD HL,SP
    pub(super) fn opcode_0x39(&mut self) {
        self.tick();

        self.add_to_hl(self.registers.stack_pointer);
    }

    /// DEC SP
    pub(super) fn opcode_0x3b(&mut self) {
        self.tick();

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    /// ADD SP,i8
    pub(super) fn opcode_0xe8(&mut self) {
        let value = self.read_byte_operand() as i8;

        self.tick();
        self.tick();

        self.add_to_stack_pointer(value.into());
    }

    /// LD HL,SP+i8
    pub(super) fn opcode_0xf8(&mut self) {
        let stack_pointer = self.registers.stack_pointer;
        let offset = self.read_byte_operand() as i8 as i16;

        let result = stack_pointer.wrapping_add_signed(offset);

        let half_carry = (stack_pointer & 0x000F).wrapping_add_signed(offset & 0x000F) > 0x000F;
        let carry = (stack_pointer & 0x00FF).wrapping_add_signed(offset & 0x00FF) > 0x00FF;

        // Store results.
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::N_ADD_SUB, false);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.f.set(Flags::CARRY, carry);

        self.tick();

        self.registers.set_hl(result);
    }
}
