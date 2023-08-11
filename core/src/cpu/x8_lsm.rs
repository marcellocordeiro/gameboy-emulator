use super::Cpu;

// Completed, may need some refactoring.

impl Cpu {
    pub(super) fn opcode_0x02(&mut self) {
        // LD (BC),A
        let address = self.registers.get_bc();
        let value = self.registers.accumulator;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x06(&mut self) {
        // LD B,u8
        let value = self.read_byte_operand();

        self.registers.b = value;
    }

    pub(super) fn opcode_0x0a(&mut self) {
        // LD A,(BC)
        let address = self.registers.get_bc();
        let value = self.read_byte(address);

        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0x0e(&mut self) {
        // LD C,u8
        let value = self.read_byte_operand();

        self.registers.c = value;
    }

    pub(super) fn opcode_0x12(&mut self) {
        // LD (DE),A
        let address = self.registers.get_de();
        let value = self.registers.accumulator;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x16(&mut self) {
        // LD D,u8
        let value = self.read_byte_operand();

        self.registers.d = value;
    }

    pub(super) fn opcode_0x1a(&mut self) {
        // LD A,(DE)
        let address = self.registers.get_de();
        let value = self.read_byte(address);

        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0x1e(&mut self) {
        // LD E,u8
        let value = self.read_byte_operand();

        self.registers.e = value;
    }

    pub(super) fn opcode_0x22(&mut self) {
        // LD (HL+),A
        let address = self.registers.get_hl();
        let value = self.registers.accumulator;

        self.write_byte(address, value);
        self.registers.set_hl(address.wrapping_add(1));
    }

    pub(super) fn opcode_0x26(&mut self) {
        // LD H,u8
        let value = self.read_byte_operand();

        self.registers.h = value;
    }

    pub(super) fn opcode_0x2a(&mut self) {
        // LD A,(HL+)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.set_hl(address.wrapping_add(1));
        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0x2e(&mut self) {
        // LD L,u8
        let value = self.read_byte_operand();

        self.registers.l = value;
    }

    pub(super) fn opcode_0x32(&mut self) {
        // LD (HL-),A
        let address = self.registers.get_hl();
        let value = self.registers.accumulator;

        self.registers.set_hl(address.wrapping_sub(1));
        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x36(&mut self) {
        // LD (HL),u8
        let address = self.registers.get_hl();
        let value = self.read_byte_operand();

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x3a(&mut self) {
        // LD A,(HL-)
        let address = self.registers.get_hl();
        let value = self.read_byte(self.registers.get_hl());

        self.registers.set_hl(address.wrapping_sub(1));
        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0x3e(&mut self) {
        // LD A,u8
        let value = self.read_byte_operand();

        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0x40(&mut self) {
        // LD B,B

        // Self assignment.
        // self.registers.b = self.registers.b;
    }

    pub(super) fn opcode_0x41(&mut self) {
        // LD B,C
        self.registers.b = self.registers.c;
    }

    pub(super) fn opcode_0x42(&mut self) {
        // LD B,D
        self.registers.b = self.registers.d;
    }

    pub(super) fn opcode_0x43(&mut self) {
        // LD B,E
        self.registers.b = self.registers.e;
    }

    pub(super) fn opcode_0x44(&mut self) {
        // LD B,H
        self.registers.b = self.registers.h;
    }

    pub(super) fn opcode_0x45(&mut self) {
        // LD B,L
        self.registers.b = self.registers.l;
    }

    pub(super) fn opcode_0x46(&mut self) {
        // LD B,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.b = value;
    }

    pub(super) fn opcode_0x47(&mut self) {
        // LD B,A
        self.registers.b = self.registers.accumulator;
    }

    pub(super) fn opcode_0x48(&mut self) {
        // LD C,B
        self.registers.c = self.registers.b;
    }

    pub(super) fn opcode_0x49(&mut self) {
        // LD C,C

        // Self assignment.
        // self.registers.c = self.registers.c;
    }

    pub(super) fn opcode_0x4a(&mut self) {
        // LD C,D
        self.registers.c = self.registers.d;
    }

    pub(super) fn opcode_0x4b(&mut self) {
        // LD C,E
        self.registers.c = self.registers.e;
    }

    pub(super) fn opcode_0x4c(&mut self) {
        // LD C,H
        self.registers.c = self.registers.h;
    }

    pub(super) fn opcode_0x4d(&mut self) {
        // LD C,L
        self.registers.c = self.registers.l;
    }

    pub(super) fn opcode_0x4e(&mut self) {
        // LD C,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.c = value;
    }

    pub(super) fn opcode_0x4f(&mut self) {
        // LD C,A
        self.registers.c = self.registers.accumulator;
    }

    pub(super) fn opcode_0x50(&mut self) {
        // LD D,B
        self.registers.d = self.registers.b;
    }

    pub(super) fn opcode_0x51(&mut self) {
        // LD D,C
        self.registers.d = self.registers.c;
    }

    pub(super) fn opcode_0x52(&mut self) {
        // LD D,D

        // Self assignment.
        // self.registers.d = self.registers.d;
    }

    pub(super) fn opcode_0x53(&mut self) {
        // LD D,E
        self.registers.d = self.registers.e;
    }

    pub(super) fn opcode_0x54(&mut self) {
        // LD D,H
        self.registers.d = self.registers.h;
    }

    pub(super) fn opcode_0x55(&mut self) {
        // LD D,L
        self.registers.d = self.registers.l;
    }

    pub(super) fn opcode_0x56(&mut self) {
        // LD D,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.d = value;
    }

    pub(super) fn opcode_0x57(&mut self) {
        // LD D,A
        self.registers.d = self.registers.accumulator;
    }

    pub(super) fn opcode_0x58(&mut self) {
        // LD E,B
        self.registers.e = self.registers.b;
    }

    pub(super) fn opcode_0x59(&mut self) {
        // LD E,C
        self.registers.e = self.registers.c;
    }

    pub(super) fn opcode_0x5a(&mut self) {
        // LD E,D
        self.registers.e = self.registers.d;
    }

    pub(super) fn opcode_0x5b(&mut self) {
        // LD E,E

        // Self assignment.
        // self.registers.e = self.registers.e;
    }

    pub(super) fn opcode_0x5c(&mut self) {
        // LD E,H
        self.registers.e = self.registers.h;
    }

    pub(super) fn opcode_0x5d(&mut self) {
        // LD E,L
        self.registers.e = self.registers.l;
    }

    pub(super) fn opcode_0x5e(&mut self) {
        // LD E,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.e = value;
    }

    pub(super) fn opcode_0x5f(&mut self) {
        // LD E,A
        self.registers.e = self.registers.accumulator;
    }

    pub(super) fn opcode_0x60(&mut self) {
        // LD H,B
        self.registers.h = self.registers.b;
    }

    pub(super) fn opcode_0x61(&mut self) {
        // LD H,C
        self.registers.h = self.registers.c;
    }

    pub(super) fn opcode_0x62(&mut self) {
        // LD H,D
        self.registers.h = self.registers.d;
    }

    pub(super) fn opcode_0x63(&mut self) {
        // LD H,E
        self.registers.h = self.registers.e;
    }

    pub(super) fn opcode_0x64(&mut self) {
        // LD H,H

        // Self assignment.
        // self.registers.h = self.registers.h;
    }

    pub(super) fn opcode_0x65(&mut self) {
        // LD H,L
        self.registers.h = self.registers.l;
    }

    pub(super) fn opcode_0x66(&mut self) {
        // LD H,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.h = value;
    }

    pub(super) fn opcode_0x67(&mut self) {
        // LD H,A
        self.registers.h = self.registers.accumulator;
    }

    pub(super) fn opcode_0x68(&mut self) {
        // LD L,B
        self.registers.l = self.registers.b;
    }

    pub(super) fn opcode_0x69(&mut self) {
        // LD L,C
        self.registers.l = self.registers.c;
    }

    pub(super) fn opcode_0x6a(&mut self) {
        // LD L,D
        self.registers.l = self.registers.d;
    }

    pub(super) fn opcode_0x6b(&mut self) {
        // LD L,E
        self.registers.l = self.registers.e;
    }

    pub(super) fn opcode_0x6c(&mut self) {
        // LD L,H
        self.registers.l = self.registers.h;
    }

    pub(super) fn opcode_0x6d(&mut self) {
        // LD L,L

        // Self assignment.
        // self.registers.l = self.registers.l;
    }

    pub(super) fn opcode_0x6e(&mut self) {
        // LD L,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.l = value;
    }

    pub(super) fn opcode_0x6f(&mut self) {
        // LD L,A
        self.registers.l = self.registers.accumulator;
    }

    pub(super) fn opcode_0x70(&mut self) {
        // LD (HL),B
        let address = self.registers.get_hl();
        let value = self.registers.b;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x71(&mut self) {
        // LD (HL),C
        let address = self.registers.get_hl();
        let value = self.registers.c;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x72(&mut self) {
        // LD (HL),D
        let address = self.registers.get_hl();
        let value = self.registers.d;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x73(&mut self) {
        // LD (HL),E
        let address = self.registers.get_hl();
        let value = self.registers.e;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x74(&mut self) {
        // LD (HL),H
        let address = self.registers.get_hl();
        let value = self.registers.h;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x75(&mut self) {
        // LD (HL),L
        let address = self.registers.get_hl();
        let value = self.registers.l;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x77(&mut self) {
        // LD (HL),A
        let address = self.registers.get_hl();
        let value = self.registers.accumulator;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0x78(&mut self) {
        // LD A,B
        self.registers.accumulator = self.registers.b;
    }

    pub(super) fn opcode_0x79(&mut self) {
        // LD A,C
        self.registers.accumulator = self.registers.c;
    }

    pub(super) fn opcode_0x7a(&mut self) {
        // LD A,D
        self.registers.accumulator = self.registers.d;
    }

    pub(super) fn opcode_0x7b(&mut self) {
        // LD A,E
        self.registers.accumulator = self.registers.e;
    }

    pub(super) fn opcode_0x7c(&mut self) {
        // LD A,H
        self.registers.accumulator = self.registers.h;
    }

    pub(super) fn opcode_0x7d(&mut self) {
        // LD A,L
        self.registers.accumulator = self.registers.l;
    }

    pub(super) fn opcode_0x7e(&mut self) {
        // LD A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0x7f(&mut self) {
        // LD A,A

        // Self assignment.
        // self.registers.accumulator = self.registers.accumulator;
    }

    pub(super) fn opcode_0xe0(&mut self) {
        // LD (FF00+u8),A
        let address = 0xFF00 + (self.read_byte_operand() as u16);
        let value = self.registers.accumulator;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0xe2(&mut self) {
        // LD (FF00+C),A
        let address = 0xFF00 + (self.registers.c as u16);
        let value = self.registers.accumulator;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0xea(&mut self) {
        // LD (u16),A
        let address = self.read_word_operand();
        let value = self.registers.accumulator;

        self.write_byte(address, value);
    }

    pub(super) fn opcode_0xf0(&mut self) {
        // LD A,(FF00+u8)
        let address = 0xFF00 + (self.read_byte_operand() as u16);
        let value = self.read_byte(address);

        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0xf2(&mut self) {
        // LD A,(FF00+C)
        let address = 0xFF00 + (self.registers.c as u16);
        let value = self.read_byte(address);

        self.registers.accumulator = value;
    }

    pub(super) fn opcode_0xfa(&mut self) {
        // LD A,(u16)
        let address = self.read_word_operand();
        let value = self.read_byte(address);

        self.registers.accumulator = value;
    }
}
