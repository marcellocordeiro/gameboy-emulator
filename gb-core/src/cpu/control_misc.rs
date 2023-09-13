use super::{registers::ImeState, Cpu};

impl Cpu {
    pub(super) fn opcode_unused(&self) {}

    pub(super) fn opcode_0x00(&self) {
        // NOP

        // Do absolutely nothing.
    }

    pub(super) fn opcode_0x10(&mut self) {
        // STOP

        // TODO?

        self.memory.speed_switch.process_speed_switch();
    }

    pub(super) fn opcode_0x76(&mut self) {
        // HALT

        if self.registers.ime.get_status() && self.memory.interrupts.get_queued_irq().is_some() {
            // TODO: implement halt bug
        }

        self.halt = true;
    }

    pub(super) fn opcode_0xcb(&mut self) {
        // PREFIX CB
        let operand = self.read_byte_operand();

        self.run_cb_instruction(operand);
    }

    pub(super) fn opcode_0xf3(&mut self) {
        // DI
        self.registers.ime = ImeState::Disabled;
    }

    pub(super) fn opcode_0xfb(&mut self) {
        // EI
        if self.registers.ime == ImeState::Disabled {
            self.registers.ime = ImeState::Pending;
        }
    }
}
