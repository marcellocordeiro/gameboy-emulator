use bitflags::Flags;

use self::registers::{ImeState, Registers};
use crate::{
    DeviceModel,
    components::memory::MemoryInterface,
    utils::{events::Events, macros::device_is_cgb},
};

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
    halt: bool,
    pub cycles: i32,

    cgb_mode: bool,

    device_model: DeviceModel,
}

impl Cpu {
    #[must_use]
    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            device_model,
            ..Default::default()
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    #[must_use]
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    pub(crate) fn skip_bootrom(&mut self) {
        self.registers.pc = 0x0100;
        self.registers.sp = 0xFFFE;

        if device_is_cgb!(self) {
            self.registers.set_af(0x1180);
            self.registers.set_bc(0x0000);
            self.registers.set_de(0x0008);
            self.registers.set_hl(0x007C);
        } else {
            self.registers.set_af(0x01B0);
            self.registers.set_bc(0x0013);
            self.registers.set_de(0x00D8);
            self.registers.set_hl(0x014D);
        }
    }

    pub(crate) fn step(&mut self, memory: &mut impl MemoryInterface) {
        // Cache the value in case there's a change
        // TODO: can we do this better?
        let interrupts_are_enabled = self.registers.ime.is_enabled();

        if !interrupts_are_enabled {
            // Takes effect in the next step
            self.registers.ime.process_request();
        }

        if self.halt {
            self.cycle_memory(memory);

            if memory.interrupts().has_queued_irq() {
                self.halt = false;
            }

            return;
        } else if interrupts_are_enabled && memory.interrupts().has_queued_irq() {
            self.handle_interrupts(memory);
            return;
        }

        let opcode = self.read_byte_operand(memory);
        self.run_instruction(memory, opcode);
    }

    pub(crate) fn run_frame(&mut self, memory: &mut impl MemoryInterface) {
        while !memory.events().contains(Events::VBLANK) {
            self.step(memory);
        }

        memory.events_mut().clear();
    }

    fn handle_interrupts(&mut self, memory: &mut impl MemoryInterface) {
        self.halt = false;
        self.registers.ime = ImeState::Disabled;

        self.cycle_memory(memory);
        self.cycle_memory(memory);
        self.cycle_memory(memory);

        let address = {
            let low = self.registers.pc as u8;
            let high = ((self.registers.pc) >> 8) as u8;

            self.push_byte_stack(memory, high);

            let address = memory
                .interrupts_mut()
                .take_queued_irq_address()
                .unwrap_or(0x0000);

            self.push_byte_stack(memory, low);

            address
        };

        self.registers.pc = address;
    }

    fn notify_cycle(&mut self, memory: &impl MemoryInterface) {
        self.cycles += if memory.speed_switch().double_speed() {
            2
        } else {
            4
        };
    }

    fn cycle_memory(&mut self, memory: &mut impl MemoryInterface) {
        self.notify_cycle(memory);
        memory.cycle();
    }

    fn read_byte(&mut self, memory: &mut impl MemoryInterface, address: u16) -> u8 {
        self.notify_cycle(memory);
        memory.read_cycle(address)
    }

    fn write_byte(&mut self, memory: &mut impl MemoryInterface, address: u16, value: u8) {
        self.notify_cycle(memory);
        memory.write_cycle(address, value);
    }

    fn read_word(&mut self, memory: &mut impl MemoryInterface, address: u16) -> u16 {
        let low = self.read_byte(memory, address) as u16;
        let high = self.read_byte(memory, address.wrapping_add(1)) as u16;

        (high << 8) | low
    }

    fn write_word(&mut self, memory: &mut impl MemoryInterface, address: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.write_byte(memory, address, low);
        self.write_byte(memory, address.wrapping_add(1), high);
    }

    fn push_byte_stack(&mut self, memory: &mut impl MemoryInterface, value: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(memory, self.registers.sp, value);
    }

    fn pop_byte_stack(&mut self, memory: &mut impl MemoryInterface) -> u8 {
        let value = self.read_byte(memory, self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        value
    }

    fn push_word_stack(&mut self, memory: &mut impl MemoryInterface, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.cycle_memory(memory);

        self.push_byte_stack(memory, high);
        self.push_byte_stack(memory, low);
    }

    fn pop_word_stack(&mut self, memory: &mut impl MemoryInterface) -> u16 {
        let low = self.pop_byte_stack(memory) as u16;
        let high = self.pop_byte_stack(memory) as u16;

        (high << 8) | low
    }

    fn read_byte_operand(&mut self, memory: &mut impl MemoryInterface) -> u8 {
        let value = self.read_byte(memory, self.registers.pc);
        self.add_to_pc(1);

        value
    }

    fn read_word_operand(&mut self, memory: &mut impl MemoryInterface) -> u16 {
        let value = self.read_word(memory, self.registers.pc);
        self.add_to_pc(2);

        value
    }

    // Control
    fn add_to_pc(&mut self, offset: i8) {
        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);
    }
}

mod alu;
mod instructions;
mod registers;

#[cfg(test)]
mod tests;
