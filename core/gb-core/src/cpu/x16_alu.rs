use super::{registers::Flags, Cpu};

// Completed, may need some refactoring.

impl Cpu {
    pub(super) fn opcode_0x03(&mut self) {
        // INC BC
        self.tick();

        self.registers
            .set_bc(self.registers.get_bc().wrapping_add(1));
    }

    pub(super) fn opcode_0x09(&mut self) {
        // ADD HL,BC
        self.tick();

        self.add_to_hl(self.registers.get_bc());
    }

    pub(super) fn opcode_0x0b(&mut self) {
        // DEC BC
        self.tick();

        self.registers
            .set_bc(self.registers.get_bc().wrapping_sub(1));
    }

    pub(super) fn opcode_0x13(&mut self) {
        // INC DE
        self.tick();

        self.registers
            .set_de(self.registers.get_de().wrapping_add(1));
    }

    pub(super) fn opcode_0x19(&mut self) {
        // ADD HL,DE
        self.tick();

        self.add_to_hl(self.registers.get_de());
    }

    pub(super) fn opcode_0x1b(&mut self) {
        // DEC DE
        self.tick();

        self.registers
            .set_de(self.registers.get_de().wrapping_sub(1));
    }

    pub(super) fn opcode_0x23(&mut self) {
        // INC HL
        self.tick();

        self.registers
            .set_hl(self.registers.get_hl().wrapping_add(1));
    }

    pub(super) fn opcode_0x29(&mut self) {
        // ADD HL,HL
        self.tick();

        self.add_to_hl(self.registers.get_hl());
    }

    pub(super) fn opcode_0x2b(&mut self) {
        // DEC HL
        self.tick();

        self.registers
            .set_hl(self.registers.get_hl().wrapping_sub(1));
    }

    pub(super) fn opcode_0x33(&mut self) {
        // INC SP
        self.tick();

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
    }

    pub(super) fn opcode_0x39(&mut self) {
        // ADD HL,SP
        self.tick();

        self.add_to_hl(self.registers.stack_pointer);
    }

    pub(super) fn opcode_0x3b(&mut self) {
        // DEC SP
        self.tick();

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    pub(super) fn opcode_0xe8(&mut self) {
        // ADD SP,i8
        let value = self.read_byte_operand() as i8;

        self.tick();
        self.tick();

        self.add_to_stack_pointer(value.into());
    }

    pub(super) fn opcode_0xf8(&mut self) {
        // LD HL,SP+i8
        let stack_pointer = self.registers.stack_pointer;
        let offset = self.read_byte_operand() as i8 as i16;

        let result = stack_pointer.wrapping_add_signed(offset);

        let half_carry = (stack_pointer & 0x000F).wrapping_add_signed(offset & 0x000F) > 0x000F;
        let carry = (stack_pointer & 0x00FF).wrapping_add_signed(offset & 0x00FF) > 0x00FF;

        // Store results.
        self.registers.flags.set(Flags::ZERO, false);
        self.registers.flags.set(Flags::N_ADD_SUB, false);
        self.registers.flags.set(Flags::HALF_CARRY, half_carry);
        self.registers.flags.set(Flags::CARRY, carry);

        self.tick();

        self.registers.set_hl(result);
    }
}
