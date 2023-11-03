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
        self.registers.pc = 0x0100;
        self.registers.sp = 0xFFFE;

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

        self.run_instruction(opcode);
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
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, value);
    }

    fn pop_byte_stack(&mut self) -> u8 {
        let value = self.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

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
        let value = self.read_byte(self.registers.pc);
        self.add_to_pc(1);

        value
    }

    fn read_word_operand(&mut self) -> u16 {
        let value = self.read_word(self.registers.pc);
        self.add_to_pc(2);

        value
    }

    // Control
    fn add_to_pc(&mut self, offset: i8) {
        self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);
    }

    fn jump_to_isr(&mut self, address: u16) {
        self.tick();

        self.push_word_stack(self.registers.pc);
        self.registers.pc = address;

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

pub mod alu;
pub mod instructions;
pub mod registers;
