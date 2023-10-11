use super::Cpu;

// Completed.

impl Cpu {
    /// LD BC,u16
    pub(super) fn opcode_0x01(&mut self) {
        let value = self.read_word_operand();

        self.registers.set_bc(value);
    }

    /// LD (u16),SP
    pub(super) fn opcode_0x08(&mut self) {
        let address = self.read_word_operand();
        let value = self.registers.stack_pointer;

        self.write_word(address, value);
    }

    /// LD DE,u16
    pub(super) fn opcode_0x11(&mut self) {
        let value = self.read_word_operand();

        self.registers.set_de(value);
    }

    /// LD HL,u16
    pub(super) fn opcode_0x21(&mut self) {
        let value = self.read_word_operand();

        self.registers.set_hl(value);
    }

    /// LD SP,u16
    pub(super) fn opcode_0x31(&mut self) {
        let value = self.read_word_operand();

        self.registers.stack_pointer = value;
    }

    /// POP BC
    pub(super) fn opcode_0xc1(&mut self) {
        let value = self.pop_word_stack();

        self.registers.set_bc(value);
    }

    /// PUSH BC
    pub(super) fn opcode_0xc5(&mut self) {
        let value = self.registers.get_bc();

        self.tick();

        self.push_word_stack(value);
    }

    /// POP DE
    pub(super) fn opcode_0xd1(&mut self) {
        let value = self.pop_word_stack();

        self.registers.set_de(value);
    }

    /// PUSH DE
    pub(super) fn opcode_0xd5(&mut self) {
        let value = self.registers.get_de();

        self.tick();

        self.push_word_stack(value);
    }

    /// POP HL
    pub(super) fn opcode_0xe1(&mut self) {
        let value = self.pop_word_stack();

        self.registers.set_hl(value);
    }

    /// PUSH HL
    pub(super) fn opcode_0xe5(&mut self) {
        let value = self.registers.get_hl();

        self.tick();

        self.push_word_stack(value);
    }

    /// POP AF
    pub(super) fn opcode_0xf1(&mut self) {
        let value = self.pop_word_stack();

        self.registers.set_af(value);
    }

    /// PUSH AF
    pub(super) fn opcode_0xf5(&mut self) {
        let value = self.registers.get_af();

        self.tick();

        self.push_word_stack(value);
    }

    /// LD SP,HL
    pub(super) fn opcode_0xf9(&mut self) {
        self.tick();

        self.registers.stack_pointer = self.registers.get_hl();
    }
}
