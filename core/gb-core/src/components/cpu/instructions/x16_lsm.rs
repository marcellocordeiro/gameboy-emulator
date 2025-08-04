use crate::components::{cpu::Cpu, memory::MemoryInterface};

impl Cpu {
    /// LD BC,u16
    pub(super) fn opcode_0x01(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.read_word_operand(memory);

        self.registers.set_bc(value);
    }

    /// LD (u16),SP
    pub(super) fn opcode_0x08(&mut self, memory: &mut impl MemoryInterface) {
        let address = self.read_word_operand(memory);
        let value = self.registers.sp;

        self.write_word(memory, address, value);
    }

    /// LD DE,u16
    pub(super) fn opcode_0x11(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.read_word_operand(memory);

        self.registers.set_de(value);
    }

    /// LD HL,u16
    pub(super) fn opcode_0x21(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.read_word_operand(memory);

        self.registers.set_hl(value);
    }

    /// LD SP,u16
    pub(super) fn opcode_0x31(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.read_word_operand(memory);

        self.registers.sp = value;
    }

    /// POP BC
    pub(super) fn opcode_0xc1(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.pop_word_stack(memory);

        self.registers.set_bc(value);
    }

    /// PUSH BC
    pub(super) fn opcode_0xc5(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.registers.get_bc();

        self.push_word_stack(memory, value);
    }

    /// POP DE
    pub(super) fn opcode_0xd1(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.pop_word_stack(memory);

        self.registers.set_de(value);
    }

    /// PUSH DE
    pub(super) fn opcode_0xd5(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.registers.get_de();

        self.push_word_stack(memory, value);
    }

    /// POP HL
    pub(super) fn opcode_0xe1(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.pop_word_stack(memory);

        self.registers.set_hl(value);
    }

    /// PUSH HL
    pub(super) fn opcode_0xe5(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.registers.get_hl();

        self.push_word_stack(memory, value);
    }

    /// POP AF
    pub(super) fn opcode_0xf1(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.pop_word_stack(memory);

        self.registers.set_af(value);
    }

    /// PUSH AF
    pub(super) fn opcode_0xf5(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.registers.get_af();

        self.push_word_stack(memory, value);
    }

    /// LD SP,HL
    pub(super) fn opcode_0xf9(&mut self, memory: &mut impl MemoryInterface) {
        self.cycle_memory(memory);

        self.registers.sp = self.registers.get_hl();
    }
}
