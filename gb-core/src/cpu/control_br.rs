use super::{
    registers::{Flags, ImeState},
    Cpu,
};

// Completed.

impl Cpu {
    pub(super) fn opcode_0x18(&mut self) {
        // JR i8
        let offset = self.read_byte_operand() as i8;

        self.jump_relative(offset);
    }

    pub(super) fn opcode_0x20(&mut self) {
        // JR NZ,i8
        let offset = self.read_byte_operand() as i8;
        let condition = !self.registers.flags.contains(Flags::ZERO);

        if condition {
            self.jump_relative(offset);
        }
    }

    pub(super) fn opcode_0x28(&mut self) {
        // JR Z,i8
        let offset = self.read_byte_operand() as i8;
        let condition = self.registers.flags.contains(Flags::ZERO);

        if condition {
            self.jump_relative(offset);
        }
    }

    pub(super) fn opcode_0x30(&mut self) {
        // JR NC,i8
        let offset = self.read_byte_operand() as i8;
        let condition = !self.registers.flags.contains(Flags::CARRY);

        if condition {
            self.jump_relative(offset);
        }
    }

    pub(super) fn opcode_0x38(&mut self) {
        // JR C,i8
        let offset = self.read_byte_operand() as i8;
        let condition = self.registers.flags.contains(Flags::CARRY);

        if condition {
            self.jump_relative(offset);
        }
    }

    pub(super) fn opcode_0xc0(&mut self) {
        // RET NZ
        let condition = !self.registers.flags.contains(Flags::ZERO);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    pub(super) fn opcode_0xc2(&mut self) {
        // JP NZ,u16
        let address = self.read_word_operand();
        let condition = !self.registers.flags.contains(Flags::ZERO);

        if condition {
            self.jump_absolute(address);
        }
    }

    pub(super) fn opcode_0xc3(&mut self) {
        // JP u16
        let address = self.read_word_operand();

        self.jump_absolute(address);
    }

    pub(super) fn opcode_0xc4(&mut self) {
        // CALL NZ,u16
        let routine_address = self.read_word_operand();
        let condition = !self.registers.flags.contains(Flags::ZERO);

        if condition {
            self.call_routine(routine_address);
        }
    }

    pub(super) fn opcode_0xc7(&mut self) {
        // RST 00h
        self.call_routine(0x00);
    }

    pub(super) fn opcode_0xc8(&mut self) {
        // RET Z
        let condition = self.registers.flags.contains(Flags::ZERO);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    pub(super) fn opcode_0xc9(&mut self) {
        // RET
        self.return_from_routine();
    }

    pub(super) fn opcode_0xca(&mut self) {
        // JP Z,u16
        let address = self.read_word_operand();
        let condition = self.registers.flags.contains(Flags::ZERO);

        if condition {
            self.jump_absolute(address);
        }
    }

    pub(super) fn opcode_0xcc(&mut self) {
        // CALL Z,u16
        let routine_address = self.read_word_operand();
        let condition = self.registers.flags.contains(Flags::ZERO);

        if condition {
            self.call_routine(routine_address);
        }
    }

    pub(super) fn opcode_0xcd(&mut self) {
        // CALL u16
        let routine_address = self.read_word_operand();

        self.call_routine(routine_address);
    }

    pub(super) fn opcode_0xcf(&mut self) {
        // RST 08h
        self.call_routine(0x08);
    }

    pub(super) fn opcode_0xd0(&mut self) {
        // RET NC
        let condition = !self.registers.flags.contains(Flags::CARRY);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    pub(super) fn opcode_0xd2(&mut self) {
        // JP NC,u16
        let address = self.read_word_operand();
        let condition = !self.registers.flags.contains(Flags::CARRY);

        if condition {
            self.jump_absolute(address);
        }
    }

    pub(super) fn opcode_0xd4(&mut self) {
        // CALL NC,u16
        let routine_address = self.read_word_operand();
        let condition = !self.registers.flags.contains(Flags::CARRY);

        if condition {
            self.call_routine(routine_address);
        }
    }

    pub(super) fn opcode_0xd7(&mut self) {
        // RST 10h
        self.call_routine(0x10);
    }

    pub(super) fn opcode_0xd8(&mut self) {
        // RET C
        let condition = self.registers.flags.contains(Flags::CARRY);

        self.tick();

        if condition {
            self.return_from_routine();
        }
    }

    pub(super) fn opcode_0xd9(&mut self) {
        // RETI
        self.registers.ime = ImeState::Enabled;
        self.return_from_routine();
    }

    pub(super) fn opcode_0xda(&mut self) {
        // JP C,u16
        let address = self.read_word_operand();
        let condition = self.registers.flags.contains(Flags::CARRY);

        if condition {
            self.jump_absolute(address);
        }
    }

    pub(super) fn opcode_0xdc(&mut self) {
        // CALL C,u16
        let routine_address = self.read_word_operand();
        let condition = self.registers.flags.contains(Flags::CARRY);

        if condition {
            self.call_routine(routine_address);
        }
    }

    pub(super) fn opcode_0xdf(&mut self) {
        // RST 18h
        self.call_routine(0x18);
    }

    pub(super) fn opcode_0xe7(&mut self) {
        // RST 20h
        self.call_routine(0x20);
    }

    pub(super) fn opcode_0xe9(&mut self) {
        // JP HL
        let address = self.registers.get_hl();

        // Can't use `jump_absolute` because this isn't supposed to tick.
        self.registers.program_counter = address;
    }

    pub(super) fn opcode_0xef(&mut self) {
        // RST 28h
        self.call_routine(0x28);
    }

    pub(super) fn opcode_0xf7(&mut self) {
        // RST 30h
        self.call_routine(0x30);
    }

    pub(super) fn opcode_0xff(&mut self) {
        // RST 38h
        self.call_routine(0x38);
    }
}
