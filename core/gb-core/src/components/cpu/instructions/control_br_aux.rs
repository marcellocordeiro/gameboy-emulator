use crate::components::{cpu::Cpu, memory::MemoryInterface};

impl Cpu {
    /// JR (relative jump to address)
    ///
    /// The offset can be negative.
    pub(super) fn jr(&mut self, memory: &mut impl MemoryInterface, offset: i8) {
        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);
        self.cycle_memory(memory);
    }

    /// JP (absolute jump to address)
    pub(super) fn jp(&mut self, memory: &mut impl MemoryInterface, address: u16) {
        self.registers.pc = address;
        self.cycle_memory(memory);
    }

    /// CALL (call address)
    pub(super) fn call(&mut self, memory: &mut impl MemoryInterface, address: u16) {
        self.push_word_stack(memory, self.registers.pc);
        self.registers.pc = address;
    }

    /// RET (return from routine)
    pub(super) fn ret(&mut self, memory: &mut impl MemoryInterface) {
        self.registers.pc = self.pop_word_stack(memory);
        self.cycle_memory(memory);
    }

    /// JR cc (conditional JR)
    pub(super) fn jr_cc(&mut self, memory: &mut impl MemoryInterface, condition: bool) {
        let offset = self.read_byte_operand(memory) as i8;

        if condition {
            self.jr(memory, offset);
        }
    }

    /// RET cc (conditional RET)
    pub(super) fn ret_cc(&mut self, memory: &mut impl MemoryInterface, condition: bool) {
        self.cycle_memory(memory);

        if condition {
            self.ret(memory);
        }
    }

    /// JP cc (conditional JP)
    pub(super) fn jp_cc(&mut self, memory: &mut impl MemoryInterface, condition: bool) {
        let address = self.read_word_operand(memory);

        if condition {
            self.jp(memory, address);
        }
    }

    /// CALL cc (conditional CALL)
    pub(super) fn call_cc(&mut self, memory: &mut impl MemoryInterface, condition: bool) {
        let address = self.read_word_operand(memory);

        if condition {
            self.call(memory, address);
        }
    }
}
