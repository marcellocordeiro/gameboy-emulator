use crate::memory::Memory;

use self::registers::{ImeState, Registers};

#[derive(Default)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,

    pub halt: bool,

    pub cycles: i32,
}

impl Cpu {
    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.memory.reset();
        self.halt = false;
        self.cycles = 0;
    }

    pub fn skip_bootrom(&mut self) {
        self.registers.program_counter = 0x0100;
        self.registers.stack_pointer = 0xFFFE;

        if cfg!(feature = "cgb") {
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

        self.memory.skip_bootrom();
    }

    pub fn step(&mut self) {
        self.handle_interrupts();

        if self.halt {
            self.tick();

            if !self.memory.interrupts.has_queued_irq() {
                return;
            }

            self.halt = false;
        }

        let opcode = self.read_byte_operand();

        self.run_next_instruction(opcode);
    }

    fn tick(&mut self) {
        self.memory.tick();

        self.cycles += 4;
    }

    fn handle_interrupts(&mut self) {
        if !self.registers.ime.is_enabled_mut() {
            return;
        }

        let Some(address) = self.memory.interrupts.take_queued_irq() else {
            return;
        };

        self.registers.ime = ImeState::Disabled;

        self.jump_to_isr(address);
        self.halt = false;
    }

    fn push_byte_stack(&mut self, value: u8) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
        self.write_byte(self.registers.stack_pointer, value);
    }

    fn pop_byte_stack(&mut self) -> u8 {
        let value = self.read_byte(self.registers.stack_pointer);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);

        value
    }

    fn push_word_stack(&mut self, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.push_byte_stack(high);
        self.push_byte_stack(low);
    }

    fn pop_word_stack(&mut self) -> u16 {
        let low = self.pop_byte_stack() as u16;
        let high = self.pop_byte_stack() as u16;

        (high << 8) | low
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.tick();

        self.memory.read(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.tick();

        self.memory.write(address, value);
    }

    fn read_word(&mut self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;

        (high << 8) | low
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    fn read_byte_operand(&mut self) -> u8 {
        let value = self.read_byte(self.registers.program_counter);
        self.add_to_pc(1);

        value
    }

    fn read_word_operand(&mut self) -> u16 {
        let value = self.read_word(self.registers.program_counter);
        self.add_to_pc(2);

        value
    }

    // Control
    fn add_to_pc(&mut self, offset: i8) {
        self.registers.program_counter = self
            .registers
            .program_counter
            .wrapping_add_signed(offset as i16);
    }

    fn jump_to_isr(&mut self, address: u16) {
        self.tick();

        self.push_word_stack(self.registers.program_counter);
        self.registers.program_counter = address;

        self.tick();
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ie_line = format!("IE: {:#04X}", self.memory.read(0xFFFF));
        let if_line = format!("IF: {:#04X}", self.memory.read(0xFF0F));

        let ime_line = format!("EI: {}", self.registers.ime);

        write!(
            f,
            "{}\n\n{}\n\n{}\n{}",
            self.registers, ime_line, ie_line, if_line
        )
    }
}

mod opcode_map;
mod registers;

mod control_br;
mod control_br_aux;
mod control_misc;
mod x16_alu;
mod x16_alu_aux;
mod x16_lsm;
mod x8_alu;
mod x8_alu_aux;
mod x8_lsm;
mod x8_rsb;
mod x8_rsb_aux;
mod x8_rsb_prefixed;
