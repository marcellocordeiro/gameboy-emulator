use crate::{
    cpu::{registers::Flags, Cpu},
    memory::Memory,
};

// Completed.

impl Cpu {
    /// JR i8
    pub(super) fn opcode_0x18(&mut self, memory: &mut Memory) {
        let offset = self.read_byte_operand(memory) as i8;

        self.jump_relative(memory, offset);
    }

    /// JR NZ,i8
    pub(super) fn opcode_0x20(&mut self, memory: &mut Memory) {
        let offset = self.read_byte_operand(memory) as i8;
        let condition = !self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_relative(memory, offset);
        }
    }

    /// JR Z,i8
    pub(super) fn opcode_0x28(&mut self, memory: &mut Memory) {
        let offset = self.read_byte_operand(memory) as i8;
        let condition = self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_relative(memory, offset);
        }
    }

    /// JR NC,i8
    pub(super) fn opcode_0x30(&mut self, memory: &mut Memory) {
        let offset = self.read_byte_operand(memory) as i8;
        let condition = !self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_relative(memory, offset);
        }
    }

    /// JR C,i8
    pub(super) fn opcode_0x38(&mut self, memory: &mut Memory) {
        let offset = self.read_byte_operand(memory) as i8;
        let condition = self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_relative(memory, offset);
        }
    }

    /// RET NZ
    pub(super) fn opcode_0xc0(&mut self, memory: &mut Memory) {
        let condition = !self.registers.f.contains(Flags::ZERO);

        self.tick(memory);

        if condition {
            self.return_from_routine(memory);
        }
    }

    /// JP NZ,u16
    pub(super) fn opcode_0xc2(&mut self, memory: &mut Memory) {
        let address = self.read_word_operand(memory);
        let condition = !self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_absolute(memory, address);
        }
    }

    /// JP u16
    pub(super) fn opcode_0xc3(&mut self, memory: &mut Memory) {
        let address = self.read_word_operand(memory);

        self.jump_absolute(memory, address);
    }

    /// CALL NZ,u16
    pub(super) fn opcode_0xc4(&mut self, memory: &mut Memory) {
        let routine_address = self.read_word_operand(memory);
        let condition = !self.registers.f.contains(Flags::ZERO);

        if condition {
            self.call_routine(memory, routine_address);
        }
    }

    /// RST 00h
    pub(super) fn opcode_0xc7(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x00);
    }

    /// RET Z
    pub(super) fn opcode_0xc8(&mut self, memory: &mut Memory) {
        let condition = self.registers.f.contains(Flags::ZERO);

        self.tick(memory);

        if condition {
            self.return_from_routine(memory);
        }
    }

    /// RET
    pub(super) fn opcode_0xc9(&mut self, memory: &mut Memory) {
        self.return_from_routine(memory);
    }

    /// JP Z,u16
    pub(super) fn opcode_0xca(&mut self, memory: &mut Memory) {
        let address = self.read_word_operand(memory);
        let condition = self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_absolute(memory, address);
        }
    }

    /// CALL Z,u16
    pub(super) fn opcode_0xcc(&mut self, memory: &mut Memory) {
        let routine_address = self.read_word_operand(memory);
        let condition = self.registers.f.contains(Flags::ZERO);

        if condition {
            self.call_routine(memory, routine_address);
        }
    }

    /// CALL u16
    pub(super) fn opcode_0xcd(&mut self, memory: &mut Memory) {
        let routine_address = self.read_word_operand(memory);

        self.call_routine(memory, routine_address);
    }

    /// RST 08h
    pub(super) fn opcode_0xcf(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x08);
    }

    /// RET NC
    pub(super) fn opcode_0xd0(&mut self, memory: &mut Memory) {
        let condition = !self.registers.f.contains(Flags::CARRY);

        self.tick(memory);

        if condition {
            self.return_from_routine(memory);
        }
    }

    /// JP NC,u16
    pub(super) fn opcode_0xd2(&mut self, memory: &mut Memory) {
        let address = self.read_word_operand(memory);
        let condition = !self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_absolute(memory, address);
        }
    }

    /// CALL NC,u16
    pub(super) fn opcode_0xd4(&mut self, memory: &mut Memory) {
        let routine_address = self.read_word_operand(memory);
        let condition = !self.registers.f.contains(Flags::CARRY);

        if condition {
            self.call_routine(memory, routine_address);
        }
    }

    /// RST 10h
    pub(super) fn opcode_0xd7(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x10);
    }

    /// RET C
    pub(super) fn opcode_0xd8(&mut self, memory: &mut Memory) {
        let condition = self.registers.f.contains(Flags::CARRY);

        self.tick(memory);

        if condition {
            self.return_from_routine(memory);
        }
    }

    /// RETI
    pub(super) fn opcode_0xd9(&mut self, memory: &mut Memory) {
        self.registers.ime.force_enable();
        self.return_from_routine(memory);
    }

    /// JP C,u16
    pub(super) fn opcode_0xda(&mut self, memory: &mut Memory) {
        let address = self.read_word_operand(memory);
        let condition = self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_absolute(memory, address);
        }
    }

    /// CALL C,u16
    pub(super) fn opcode_0xdc(&mut self, memory: &mut Memory) {
        let routine_address = self.read_word_operand(memory);
        let condition = self.registers.f.contains(Flags::CARRY);

        if condition {
            self.call_routine(memory, routine_address);
        }
    }

    /// RST 18h
    pub(super) fn opcode_0xdf(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x18);
    }

    /// RST 20h
    pub(super) fn opcode_0xe7(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x20);
    }

    /// JP HL
    pub(super) fn opcode_0xe9(&mut self) {
        let address = self.registers.get_hl();

        // Can't use `jump_absolute` because this isn't supposed to tick.
        self.registers.pc = address;
    }

    /// RST 28h
    pub(super) fn opcode_0xef(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x28);
    }

    /// RST 30h
    pub(super) fn opcode_0xf7(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x30);
    }

    /// RST 38h
    pub(super) fn opcode_0xff(&mut self, memory: &mut Memory) {
        self.call_routine(memory, 0x38);
    }
}
