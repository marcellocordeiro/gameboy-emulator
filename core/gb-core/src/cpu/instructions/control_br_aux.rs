use crate::{cpu::Cpu, memory::MemoryInterface};

impl Cpu {
    // JR
    pub(super) fn jump_relative(&mut self, memory: &mut impl MemoryInterface, offset: i8) {
        self.tick(memory);

        // Offset can be negative.
        self.add_to_pc(offset);
    }

    // JP
    pub(super) fn jump_absolute(&mut self, memory: &mut impl MemoryInterface, address: u16) {
        self.tick(memory);

        self.registers.pc = address;
    }

    // CALL
    pub(super) fn call_routine(&mut self, memory: &mut impl MemoryInterface, routine_address: u16) {
        self.tick(memory);

        self.push_word_stack(memory, self.registers.pc);
        self.registers.pc = routine_address;
    }

    // RET
    pub(super) fn return_from_routine(&mut self, memory: &mut impl MemoryInterface) {
        self.tick(memory);

        self.registers.pc = self.pop_word_stack(memory);
    }
}
