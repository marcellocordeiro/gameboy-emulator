use super::Cpu;

// Completed, may need some refactoring.

impl Cpu {
    /// LD (BC),A
    pub(super) fn opcode_0x02(&mut self) {
        let address = self.registers.get_bc();
        let value = self.registers.a;

        self.write_byte(address, value);
    }

    /// LD B,u8
    pub(super) fn opcode_0x06(&mut self) {
        let value = self.read_byte_operand();

        self.registers.b = value;
    }

    /// LD A,(BC)
    pub(super) fn opcode_0x0a(&mut self) {
        let address = self.registers.get_bc();
        let value = self.read_byte(address);

        self.registers.a = value;
    }

    /// LD C,u8
    pub(super) fn opcode_0x0e(&mut self) {
        let value = self.read_byte_operand();

        self.registers.c = value;
    }

    /// LD (DE),A
    pub(super) fn opcode_0x12(&mut self) {
        let address = self.registers.get_de();
        let value = self.registers.a;

        self.write_byte(address, value);
    }

    /// LD D,u8
    pub(super) fn opcode_0x16(&mut self) {
        let value = self.read_byte_operand();

        self.registers.d = value;
    }

    /// LD A,(DE)
    pub(super) fn opcode_0x1a(&mut self) {
        let address = self.registers.get_de();
        let value = self.read_byte(address);

        self.registers.a = value;
    }

    /// LD E,u8
    pub(super) fn opcode_0x1e(&mut self) {
        let value = self.read_byte_operand();

        self.registers.e = value;
    }

    /// LD (HL+),A
    pub(super) fn opcode_0x22(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.write_byte(address, value);
        self.registers.set_hl(address.wrapping_add(1));
    }

    /// LD H,u8
    pub(super) fn opcode_0x26(&mut self) {
        let value = self.read_byte_operand();

        self.registers.h = value;
    }

    /// LD A,(HL+)
    pub(super) fn opcode_0x2a(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.set_hl(address.wrapping_add(1));
        self.registers.a = value;
    }

    /// LD L,u8
    pub(super) fn opcode_0x2e(&mut self) {
        let value = self.read_byte_operand();

        self.registers.l = value;
    }

    /// LD (HL-),A
    pub(super) fn opcode_0x32(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.registers.set_hl(address.wrapping_sub(1));
        self.write_byte(address, value);
    }

    /// LD (HL),u8
    pub(super) fn opcode_0x36(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte_operand();

        self.write_byte(address, value);
    }

    /// LD A,(HL-)
    pub(super) fn opcode_0x3a(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(self.registers.get_hl());

        self.registers.set_hl(address.wrapping_sub(1));
        self.registers.a = value;
    }

    /// LD A,u8
    pub(super) fn opcode_0x3e(&mut self) {
        let value = self.read_byte_operand();

        self.registers.a = value;
    }

    /// LD B,B
    pub(super) fn opcode_0x40(&mut self) {

        // Self assignment.
        // self.registers.b = self.registers.b;
    }

    /// LD B,C
    pub(super) fn opcode_0x41(&mut self) {
        self.registers.b = self.registers.c;
    }

    /// LD B,D
    pub(super) fn opcode_0x42(&mut self) {
        self.registers.b = self.registers.d;
    }

    /// LD B,E
    pub(super) fn opcode_0x43(&mut self) {
        self.registers.b = self.registers.e;
    }

    /// LD B,H
    pub(super) fn opcode_0x44(&mut self) {
        self.registers.b = self.registers.h;
    }

    /// LD B,L
    pub(super) fn opcode_0x45(&mut self) {
        self.registers.b = self.registers.l;
    }

    /// LD B,(HL)
    pub(super) fn opcode_0x46(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.b = value;
    }

    /// LD B,A
    pub(super) fn opcode_0x47(&mut self) {
        self.registers.b = self.registers.a;
    }

    /// LD C,B
    pub(super) fn opcode_0x48(&mut self) {
        self.registers.c = self.registers.b;
    }

    /// LD C,C
    pub(super) fn opcode_0x49(&mut self) {
        // Self assignment.
        // self.registers.c = self.registers.c;
    }

    /// LD C,D
    pub(super) fn opcode_0x4a(&mut self) {
        self.registers.c = self.registers.d;
    }

    /// LD C,E
    pub(super) fn opcode_0x4b(&mut self) {
        self.registers.c = self.registers.e;
    }

    /// LD C,H
    pub(super) fn opcode_0x4c(&mut self) {
        self.registers.c = self.registers.h;
    }

    /// LD C,L
    pub(super) fn opcode_0x4d(&mut self) {
        self.registers.c = self.registers.l;
    }

    /// LD C,(HL)
    pub(super) fn opcode_0x4e(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.c = value;
    }

    /// LD C,A
    pub(super) fn opcode_0x4f(&mut self) {
        self.registers.c = self.registers.a;
    }

    /// LD D,B
    pub(super) fn opcode_0x50(&mut self) {
        self.registers.d = self.registers.b;
    }

    /// LD D,C
    pub(super) fn opcode_0x51(&mut self) {
        self.registers.d = self.registers.c;
    }

    /// LD D,D
    pub(super) fn opcode_0x52(&mut self) {
        // Self assignment.
        // self.registers.d = self.registers.d;
    }

    /// LD D,E
    pub(super) fn opcode_0x53(&mut self) {
        self.registers.d = self.registers.e;
    }

    /// LD D,H
    pub(super) fn opcode_0x54(&mut self) {
        self.registers.d = self.registers.h;
    }

    /// LD D,L
    pub(super) fn opcode_0x55(&mut self) {
        self.registers.d = self.registers.l;
    }

    /// LD D,(HL)
    pub(super) fn opcode_0x56(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.d = value;
    }

    /// LD D,A
    pub(super) fn opcode_0x57(&mut self) {
        self.registers.d = self.registers.a;
    }

    /// LD E,B
    pub(super) fn opcode_0x58(&mut self) {
        self.registers.e = self.registers.b;
    }

    /// LD E,C
    pub(super) fn opcode_0x59(&mut self) {
        self.registers.e = self.registers.c;
    }

    /// LD E,D
    pub(super) fn opcode_0x5a(&mut self) {
        self.registers.e = self.registers.d;
    }

    /// LD E,E
    pub(super) fn opcode_0x5b(&mut self) {
        // Self assignment.
        // self.registers.e = self.registers.e;
    }

    /// LD E,H
    pub(super) fn opcode_0x5c(&mut self) {
        self.registers.e = self.registers.h;
    }

    /// LD E,L
    pub(super) fn opcode_0x5d(&mut self) {
        self.registers.e = self.registers.l;
    }

    /// LD E,(HL)
    pub(super) fn opcode_0x5e(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.e = value;
    }

    /// LD E,A
    pub(super) fn opcode_0x5f(&mut self) {
        self.registers.e = self.registers.a;
    }

    /// LD H,B
    pub(super) fn opcode_0x60(&mut self) {
        self.registers.h = self.registers.b;
    }

    /// LD H,C
    pub(super) fn opcode_0x61(&mut self) {
        self.registers.h = self.registers.c;
    }

    /// LD H,D
    pub(super) fn opcode_0x62(&mut self) {
        self.registers.h = self.registers.d;
    }

    /// LD H,E
    pub(super) fn opcode_0x63(&mut self) {
        self.registers.h = self.registers.e;
    }

    /// LD H,H
    pub(super) fn opcode_0x64(&mut self) {
        // Self assignment.
        // self.registers.h = self.registers.h;
    }

    /// LD H,L
    pub(super) fn opcode_0x65(&mut self) {
        self.registers.h = self.registers.l;
    }

    /// LD H,(HL)
    pub(super) fn opcode_0x66(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.h = value;
    }

    /// LD H,A
    pub(super) fn opcode_0x67(&mut self) {
        self.registers.h = self.registers.a;
    }

    /// LD L,B
    pub(super) fn opcode_0x68(&mut self) {
        self.registers.l = self.registers.b;
    }

    /// LD L,C
    pub(super) fn opcode_0x69(&mut self) {
        self.registers.l = self.registers.c;
    }

    /// LD L,D
    pub(super) fn opcode_0x6a(&mut self) {
        self.registers.l = self.registers.d;
    }

    /// LD L,E
    pub(super) fn opcode_0x6b(&mut self) {
        self.registers.l = self.registers.e;
    }

    /// LD L,H
    pub(super) fn opcode_0x6c(&mut self) {
        self.registers.l = self.registers.h;
    }

    /// LD L,L
    pub(super) fn opcode_0x6d(&mut self) {
        // Self assignment.
        // self.registers.l = self.registers.l;
    }

    /// LD L,(HL)
    pub(super) fn opcode_0x6e(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.l = value;
    }

    /// LD L,A
    pub(super) fn opcode_0x6f(&mut self) {
        self.registers.l = self.registers.a;
    }

    /// LD (HL),B
    pub(super) fn opcode_0x70(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.b;

        self.write_byte(address, value);
    }

    /// LD (HL),C
    pub(super) fn opcode_0x71(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.c;

        self.write_byte(address, value);
    }

    /// LD (HL),D
    pub(super) fn opcode_0x72(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.d;

        self.write_byte(address, value);
    }

    /// LD (HL),E
    pub(super) fn opcode_0x73(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.e;

        self.write_byte(address, value);
    }

    /// LD (HL),H
    pub(super) fn opcode_0x74(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.h;

        self.write_byte(address, value);
    }

    /// LD (HL),L
    pub(super) fn opcode_0x75(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.l;

        self.write_byte(address, value);
    }

    /// LD (HL),A
    pub(super) fn opcode_0x77(&mut self) {
        let address = self.registers.get_hl();
        let value = self.registers.a;

        self.write_byte(address, value);
    }

    /// LD A,B
    pub(super) fn opcode_0x78(&mut self) {
        self.registers.a = self.registers.b;
    }

    /// LD A,C
    pub(super) fn opcode_0x79(&mut self) {
        self.registers.a = self.registers.c;
    }

    /// LD A,D
    pub(super) fn opcode_0x7a(&mut self) {
        self.registers.a = self.registers.d;
    }

    /// LD A,E
    pub(super) fn opcode_0x7b(&mut self) {
        self.registers.a = self.registers.e;
    }

    /// LD A,H
    pub(super) fn opcode_0x7c(&mut self) {
        self.registers.a = self.registers.h;
    }

    /// LD A,L
    pub(super) fn opcode_0x7d(&mut self) {
        self.registers.a = self.registers.l;
    }

    /// LD A,(HL)
    pub(super) fn opcode_0x7e(&mut self) {
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.registers.a = value;
    }

    /// LD A,A
    pub(super) fn opcode_0x7f(&mut self) {
        // Self assignment.
        // self.registers.a = self.registers.a;
    }

    /// LD (FF00+u8),A
    pub(super) fn opcode_0xe0(&mut self) {
        let address = 0xFF00 + (self.read_byte_operand() as u16);
        let value = self.registers.a;

        self.write_byte(address, value);
    }

    /// LD (FF00+C),A
    pub(super) fn opcode_0xe2(&mut self) {
        let address = 0xFF00 + (self.registers.c as u16);
        let value = self.registers.a;

        self.write_byte(address, value);
    }

    /// LD (u16),A
    pub(super) fn opcode_0xea(&mut self) {
        let address = self.read_word_operand();
        let value = self.registers.a;

        self.write_byte(address, value);
    }

    /// LD A,(FF00+u8)
    pub(super) fn opcode_0xf0(&mut self) {
        let address = 0xFF00 + (self.read_byte_operand() as u16);
        let value = self.read_byte(address);

        self.registers.a = value;
    }

    /// LD A,(FF00+C)
    pub(super) fn opcode_0xf2(&mut self) {
        let address = 0xFF00 + (self.registers.c as u16);
        let value = self.read_byte(address);

        self.registers.a = value;
    }

    /// LD A,(u16)
    pub(super) fn opcode_0xfa(&mut self) {
        let address = self.read_word_operand();
        let value = self.read_byte(address);

        self.registers.a = value;
    }
}
