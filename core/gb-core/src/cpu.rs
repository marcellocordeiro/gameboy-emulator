use self::registers::{ImeState, Registers};
use crate::{
    memory::MemoryInterface,
    utils::macros::device_is_cgb,
    DeviceConfig,
    DeviceModel,
    OptionalCgbComponent,
};

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
    halt: bool,
    pub cycles: i32,
    device_config: DeviceConfig,
}

impl OptionalCgbComponent for Cpu {
    fn with_device_model(model: DeviceModel) -> Self {
        let device_config = DeviceConfig {
            model,
            ..Default::default()
        };

        Self {
            device_config,
            ..Default::default()
        }
    }

    fn set_cgb_mode(&mut self, value: bool) {
        self.device_config.cgb_mode = value;
    }
}

impl Cpu {
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
        self.handle_interrupts(memory);

        if self.halt {
            self.force_cycle_memory(memory);

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

    fn handle_interrupts(&mut self, memory: &mut impl MemoryInterface) {
        if !self.registers.ime.is_enabled_mut() {
            return;
        }

        if !memory.interrupts().has_queued_irq() {
            return;
        };

        self.registers.ime = ImeState::Disabled;
        self.force_cycle_memory(memory);

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
        self.force_cycle_memory(memory);

        self.halt = false;
    }

    fn notify_cycle(&mut self) {
        self.cycles += 4;
    }

    fn force_cycle_memory(&mut self, memory: &mut impl MemoryInterface) {
        memory.force_cycle();
        self.notify_cycle();
    }

    fn read_byte(&mut self, memory: &mut impl MemoryInterface, address: u16) -> u8 {
        self.notify_cycle();
        memory.read_cycle(address)
    }

    fn write_byte(&mut self, memory: &mut impl MemoryInterface, address: u16, value: u8) {
        self.notify_cycle();
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

        self.force_cycle_memory(memory);

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
