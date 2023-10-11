use super::{registers::Flags, Cpu};

// Completed.

impl Cpu {
    /// JR i8
    pub(super) fn opcode_0x18(&mut self) {
        let offset = self.read_byte_operand() as i8;

        self.jump_relative(offset);
    }

    /// JR NZ,i8
    pub(super) fn opcode_0x20(&mut self) {
        let offset = self.read_byte_operand() as i8;
        let condition = !self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_relative(offset);
        }
    }

    /// JR Z,i8
    pub(super) fn opcode_0x28(&mut self) {
        let offset = self.read_byte_operand() as i8;
        let condition = self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_relative(offset);
        }
    }

    /// JR NC,i8
    pub(super) fn opcode_0x30(&mut self) {
        let offset = self.read_byte_operand() as i8;
        let condition = !self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_relative(offset);
        }
    }

    /// JR C,i8
    pub(super) fn opcode_0x38(&mut self) {
        let offset = self.read_byte_operand() as i8;
        let condition = self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_relative(offset);
        }
    }

    /// RET NZ
    pub(super) fn opcode_0xc0(&mut self) {
        let condition = !self.registers.f.contains(Flags::ZERO);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    /// JP NZ,u16
    pub(super) fn opcode_0xc2(&mut self) {
        let address = self.read_word_operand();
        let condition = !self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_absolute(address);
        }
    }

    /// JP u16
    pub(super) fn opcode_0xc3(&mut self) {
        let address = self.read_word_operand();

        self.jump_absolute(address);
    }

    /// CALL NZ,u16
    pub(super) fn opcode_0xc4(&mut self) {
        let routine_address = self.read_word_operand();
        let condition = !self.registers.f.contains(Flags::ZERO);

        if condition {
            self.call_routine(routine_address);
        }
    }

    /// RST 00h
    pub(super) fn opcode_0xc7(&mut self) {
        self.call_routine(0x00);
    }

    /// RET Z
    pub(super) fn opcode_0xc8(&mut self) {
        let condition = self.registers.f.contains(Flags::ZERO);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    /// RET
    pub(super) fn opcode_0xc9(&mut self) {
        self.return_from_routine();
    }

    /// JP Z,u16
    pub(super) fn opcode_0xca(&mut self) {
        let address = self.read_word_operand();
        let condition = self.registers.f.contains(Flags::ZERO);

        if condition {
            self.jump_absolute(address);
        }
    }

    /// CALL Z,u16
    pub(super) fn opcode_0xcc(&mut self) {
        let routine_address = self.read_word_operand();
        let condition = self.registers.f.contains(Flags::ZERO);

        if condition {
            self.call_routine(routine_address);
        }
    }

    /// CALL u16
    pub(super) fn opcode_0xcd(&mut self) {
        let routine_address = self.read_word_operand();

        self.call_routine(routine_address);
    }

    /// RST 08h
    pub(super) fn opcode_0xcf(&mut self) {
        self.call_routine(0x08);
    }

    /// RET NC
    pub(super) fn opcode_0xd0(&mut self) {
        let condition = !self.registers.f.contains(Flags::CARRY);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    /// JP NC,u16
    pub(super) fn opcode_0xd2(&mut self) {
        let address = self.read_word_operand();
        let condition = !self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_absolute(address);
        }
    }

    /// CALL NC,u16
    pub(super) fn opcode_0xd4(&mut self) {
        let routine_address = self.read_word_operand();
        let condition = !self.registers.f.contains(Flags::CARRY);

        if condition {
            self.call_routine(routine_address);
        }
    }

    /// RST 10h
    pub(super) fn opcode_0xd7(&mut self) {
        self.call_routine(0x10);
    }

    /// RET C
    pub(super) fn opcode_0xd8(&mut self) {
        let condition = self.registers.f.contains(Flags::CARRY);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    /// RETI
    pub(super) fn opcode_0xd9(&mut self) {
        self.registers.ime.force_enable();
        self.return_from_routine();
    }

    /// JP C,u16
    pub(super) fn opcode_0xda(&mut self) {
        let address = self.read_word_operand();
        let condition = self.registers.f.contains(Flags::CARRY);

        if condition {
            self.jump_absolute(address);
        }
    }

    /// CALL C,u16
    pub(super) fn opcode_0xdc(&mut self) {
        let routine_address = self.read_word_operand();
        let condition = self.registers.f.contains(Flags::CARRY);

        if condition {
            self.call_routine(routine_address);
        }
    }

    /// RST 18h
    pub(super) fn opcode_0xdf(&mut self) {
        self.call_routine(0x18);
    }

    /// RST 20h
    pub(super) fn opcode_0xe7(&mut self) {
        self.call_routine(0x20);
    }

    /// JP HL
    pub(super) fn opcode_0xe9(&mut self) {
        let address = self.registers.get_hl();

        // Can't use `jump_absolute` because this isn't supposed to tick.
        self.registers.program_counter = address;
    }

    /// RST 28h
    pub(super) fn opcode_0xef(&mut self) {
        self.call_routine(0x28);
    }

    /// RST 30h
    pub(super) fn opcode_0xf7(&mut self) {
        self.call_routine(0x30);
    }

    /// RST 38h
    pub(super) fn opcode_0xff(&mut self) {
        self.call_routine(0x38);
    }
}
