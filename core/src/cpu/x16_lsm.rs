use super::Cpu;

// Completed.

impl Cpu {
    pub(super) fn opcode_0x01(&mut self) {
        // LD BC,u16
        let value = self.read_word_operand();

        self.registers.set_bc(value);
    }

    pub(super) fn opcode_0x08(&mut self) {
        // LD (u16),SP
        let address = self.read_word_operand();
        let value = self.registers.stack_pointer;

        self.write_word(address, value);
    }

    pub(super) fn opcode_0x11(&mut self) {
        // LD DE,u16
        let value = self.read_word_operand();

        self.registers.set_de(value);
    }

    pub(super) fn opcode_0x21(&mut self) {
        // LD HL,u16
        let value = self.read_word_operand();

        self.registers.set_hl(value);
    }

    pub(super) fn opcode_0x31(&mut self) {
        // LD SP,u16
        let value = self.read_word_operand();

        self.registers.stack_pointer = value;
    }

    pub(super) fn opcode_0xc1(&mut self) {
        // POP BC
        let value = self.pop_word_stack();

        self.registers.set_bc(value);
    }

    pub(super) fn opcode_0xc5(&mut self) {
        // PUSH BC
        let value = self.registers.get_bc();

        self.tick();

        self.push_word_stack(value);
    }

    pub(super) fn opcode_0xd1(&mut self) {
        // POP DE
        let value = self.pop_word_stack();

        self.registers.set_de(value);
    }

    pub(super) fn opcode_0xd5(&mut self) {
        // PUSH DE
        let value = self.registers.get_de();

        self.tick();

        self.push_word_stack(value);
    }

    pub(super) fn opcode_0xe1(&mut self) {
        // POP HL
        let value = self.pop_word_stack();

        self.registers.set_hl(value);
    }

    pub(super) fn opcode_0xe5(&mut self) {
        // PUSH HL
        let value = self.registers.get_hl();

        self.tick();

        self.push_word_stack(value);
    }

    pub(super) fn opcode_0xf1(&mut self) {
        // POP AF
        let value = self.pop_word_stack();

        self.registers.set_af(value);
    }

    pub(super) fn opcode_0xf5(&mut self) {
        // PUSH AF
        let value = self.registers.get_af();

        self.tick();

        self.push_word_stack(value);
    }

    pub(super) fn opcode_0xf9(&mut self) {
        // LD SP,HL
        self.tick();

        self.registers.stack_pointer = self.registers.get_hl();
    }
}
