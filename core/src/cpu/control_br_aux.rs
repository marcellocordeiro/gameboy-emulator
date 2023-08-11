use super::Cpu;

impl Cpu {
    // JR
    pub(super) fn jump_relative(&mut self, offset: i8) {
        self.tick();

        // Offset can be negative.
        self.add_to_pc(offset);
    }

    // JP
    pub(super) fn jump_absolute(&mut self, address: u16) {
        self.tick();

        self.registers.program_counter = address;
    }

    // CALL
    pub(super) fn call_routine(&mut self, routine_address: u16) {
        self.tick();

        self.push_word_stack(self.registers.program_counter);
        self.registers.program_counter = routine_address;
    }

    // RET
    pub(super) fn return_from_routine(&mut self) {
        self.tick();

        self.registers.program_counter = self.pop_word_stack();
    }
}
