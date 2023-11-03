use crate::cpu::Cpu;

impl Cpu {
    pub(super) fn opcode_unused(&self) {}

    /// NOP
    pub(super) fn opcode_0x00(&self) {

        // Do absolutely nothing.
    }

    /// STOP
    pub(super) fn opcode_0x10(&mut self) {
        // TODO?

        self.memory.speed_switch.process_speed_switch();
    }

    /// HALT
    pub(super) fn opcode_0x76(&mut self) {
        if self.registers.ime.is_enabled() && self.memory.interrupts.has_queued_irq() {
            // TODO: implement halt bug
        }

        self.halt = true;
    }

    /// PREFIX CB
    pub(super) fn opcode_0xcb(&mut self) {
        let operand = self.read_byte_operand();

        self.run_cb_instruction(operand);
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
