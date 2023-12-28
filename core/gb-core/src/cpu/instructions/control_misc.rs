use crate::{cpu::Cpu, memory::MemoryInterface};

impl Cpu {
    pub(super) fn opcode_unused(&self) {}

    /// NOP
    pub(super) fn opcode_0x00(&self) {
        // Do absolutely nothing.
    }

    /// STOP
    pub(super) fn opcode_0x10(&mut self, memory: &mut impl MemoryInterface) {
        // TODO?

        memory.speed_switch_mut().process_speed_switch();
    }

    /// HALT
    pub(super) fn opcode_0x76(&mut self, memory: &impl MemoryInterface) {
        if self.registers.ime.is_enabled() && memory.interrupts().has_queued_irq() {
            // TODO: implement halt bug
        }

        self.halt = true;
    }

    /// PREFIX CB
    pub(super) fn opcode_0xcb(&mut self, memory: &mut impl MemoryInterface) {
        let operand = self.read_byte_operand(memory);

        self.run_cb_instruction(memory, operand);
    }

    /// DI
    pub(super) fn opcode_0xf3(&mut self) {
        self.registers.ime.disable();
    }

    /// EI
    pub(super) fn opcode_0xfb(&mut self) {
        self.registers.ime.request_enable();
    }
}
