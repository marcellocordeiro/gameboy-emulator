use self::registers::{ImeState, Registers};
use crate::{memory::MemoryInterface, utils::macros::device_is_cgb};

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
    halt: bool,
    pub cycles: i32,
}

impl Cpu {
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    pub(crate) fn skip_bootrom(&mut self) {
        self.registers.pc = 0x0100;
        self.registers.sp = 0xFFFE;

        if device_is_cgb!() {
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
        self.handle_interrupts(memory);

        if self.halt {
            self.tick(memory);

            if !memory.interrupts().has_queued_irq() {
                return;
            }

            self.halt = false;
        }

        let opcode = self.read_byte_operand(memory);

        self.run_instruction(memory, opcode);
    }

    pub(crate) fn run_frame(&mut self, memory: &mut impl MemoryInterface) {
        self.cycles = 0;
        while self.cycles < 70224 {
            self.step(memory);
        }
    }

    fn tick(&mut self, memory: &mut impl MemoryInterface) {
        memory.tick();

        self.cycles += 4;
    }

    fn handle_interrupts(&mut self, memory: &mut impl MemoryInterface) {
        if !self.registers.ime.is_enabled_mut() {
            return;
        }

        let Some(address) = memory.interrupts_mut().take_queued_irq() else {
            return;
        };

        self.registers.ime = ImeState::Disabled;

        self.jump_to_isr(memory, address);
        self.halt = false;
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

        self.push_byte_stack(memory, high);
        self.push_byte_stack(memory, low);
    }

    fn pop_word_stack(&mut self, memory: &mut impl MemoryInterface) -> u16 {
        let low = self.pop_byte_stack(memory) as u16;
        let high = self.pop_byte_stack(memory) as u16;

        (high << 8) | low
    }

    fn read_byte(&mut self, memory: &mut impl MemoryInterface, address: u16) -> u8 {
        self.tick(memory);

        memory.read(address)
    }

    fn write_byte(&mut self, memory: &mut impl MemoryInterface, address: u16, value: u8) {
        self.tick(memory);

        memory.write(address, value);
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

    fn jump_to_isr(&mut self, memory: &mut impl MemoryInterface, address: u16) {
        self.tick(memory);

        self.push_word_stack(memory, self.registers.pc);
        self.registers.pc = address;

        self.tick(memory);
    }
}

mod alu;
mod instructions;
mod registers;

#[cfg(test)]
mod tests;
