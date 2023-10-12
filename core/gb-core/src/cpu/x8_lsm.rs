use super::Cpu;

// Completed, may need some refactoring.

macro_rules! ld_r8_r8 {
    ($self:ident, $reg0:ident, $reg1:ident) => {
        $self.registers.$reg0 = $self.registers.$reg1
    };
}

macro_rules! ld_r8_hl {
    ($self:ident, $reg:ident, (hl)) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte(address);

        $self.registers.$reg = value;
    };
}

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
    }

    /// LD B,C
    pub(super) fn opcode_0x41(&mut self) {
        ld_r8_r8!(self, b, c);
    }

    /// LD B,D
    pub(super) fn opcode_0x42(&mut self) {
        ld_r8_r8!(self, b, d);
    }

    /// LD B,E
    pub(super) fn opcode_0x43(&mut self) {
        ld_r8_r8!(self, b, e);
    }

    /// LD B,H
    pub(super) fn opcode_0x44(&mut self) {
        ld_r8_r8!(self, b, h);
    }

    /// LD B,L
    pub(super) fn opcode_0x45(&mut self) {
        ld_r8_r8!(self, b, l);
    }

    /// LD B,(HL)
    pub(super) fn opcode_0x46(&mut self) {
        ld_r8_hl!(self, b, (hl));
    }

    /// LD B,A
    pub(super) fn opcode_0x47(&mut self) {
        ld_r8_r8!(self, b, a);
    }

    /// LD C,B
    pub(super) fn opcode_0x48(&mut self) {
        ld_r8_r8!(self, c, b);
    }

    /// LD C,C
    pub(super) fn opcode_0x49(&mut self) {
        // Self assignment.
    }

    /// LD C,D
    pub(super) fn opcode_0x4a(&mut self) {
        ld_r8_r8!(self, c, d);
    }

    /// LD C,E
    pub(super) fn opcode_0x4b(&mut self) {
        ld_r8_r8!(self, c, e);
    }

    /// LD C,H
    pub(super) fn opcode_0x4c(&mut self) {
        ld_r8_r8!(self, c, h);
    }

    /// LD C,L
    pub(super) fn opcode_0x4d(&mut self) {
        ld_r8_r8!(self, c, l);
    }

    /// LD C,(HL)
    pub(super) fn opcode_0x4e(&mut self) {
        ld_r8_hl!(self, c, (hl));
    }

    /// LD C,A
    pub(super) fn opcode_0x4f(&mut self) {
        ld_r8_r8!(self, c, a);
    }

    /// LD D,B
    pub(super) fn opcode_0x50(&mut self) {
        ld_r8_r8!(self, d, b);
    }

    /// LD D,C
    pub(super) fn opcode_0x51(&mut self) {
        ld_r8_r8!(self, d, c);
    }

    /// LD D,D
    pub(super) fn opcode_0x52(&mut self) {
        // Self assignment.
    }

    /// LD D,E
    pub(super) fn opcode_0x53(&mut self) {
        ld_r8_r8!(self, d, e);
    }

    /// LD D,H
    pub(super) fn opcode_0x54(&mut self) {
        ld_r8_r8!(self, d, h);
    }

    /// LD D,L
    pub(super) fn opcode_0x55(&mut self) {
        ld_r8_r8!(self, d, l);
    }

    /// LD D,(HL)
    pub(super) fn opcode_0x56(&mut self) {
        ld_r8_hl!(self, d, (hl));
    }

    /// LD D,A
    pub(super) fn opcode_0x57(&mut self) {
        ld_r8_r8!(self, d, a);
    }

    /// LD E,B
    pub(super) fn opcode_0x58(&mut self) {
        ld_r8_r8!(self, e, b);
    }

    /// LD E,C
    pub(super) fn opcode_0x59(&mut self) {
        ld_r8_r8!(self, e, c);
    }

    /// LD E,D
    pub(super) fn opcode_0x5a(&mut self) {
        ld_r8_r8!(self, e, d);
    }

    /// LD E,E
    pub(super) fn opcode_0x5b(&mut self) {
        // Self assignment.
    }

    /// LD E,H
    pub(super) fn opcode_0x5c(&mut self) {
        ld_r8_r8!(self, e, h);
    }

    /// LD E,L
    pub(super) fn opcode_0x5d(&mut self) {
        ld_r8_r8!(self, e, l);
    }

    /// LD E,(HL)
    pub(super) fn opcode_0x5e(&mut self) {
        ld_r8_hl!(self, e, (hl));
    }

    /// LD E,A
    pub(super) fn opcode_0x5f(&mut self) {
        ld_r8_r8!(self, e, a);
    }

    /// LD H,B
    pub(super) fn opcode_0x60(&mut self) {
        ld_r8_r8!(self, h, b);
    }

    /// LD H,C
    pub(super) fn opcode_0x61(&mut self) {
        ld_r8_r8!(self, h, c);
    }

    /// LD H,D
    pub(super) fn opcode_0x62(&mut self) {
        ld_r8_r8!(self, h, d);
    }

    /// LD H,E
    pub(super) fn opcode_0x63(&mut self) {
        ld_r8_r8!(self, h, e);
    }

    /// LD H,H
    pub(super) fn opcode_0x64(&mut self) {
        // Self assignment.
    }

    /// LD H,L
    pub(super) fn opcode_0x65(&mut self) {
        ld_r8_r8!(self, h, l);
    }

    /// LD H,(HL)
    pub(super) fn opcode_0x66(&mut self) {
        ld_r8_hl!(self, h, (hl));
    }

    /// LD H,A
    pub(super) fn opcode_0x67(&mut self) {
        ld_r8_r8!(self, h, a);
    }

    /// LD L,B
    pub(super) fn opcode_0x68(&mut self) {
        ld_r8_r8!(self, l, b);
    }

    /// LD L,C
    pub(super) fn opcode_0x69(&mut self) {
        ld_r8_r8!(self, l, c);
    }

    /// LD L,D
    pub(super) fn opcode_0x6a(&mut self) {
        ld_r8_r8!(self, l, d);
    }

    /// LD L,E
    pub(super) fn opcode_0x6b(&mut self) {
        ld_r8_r8!(self, l, e);
    }

    /// LD L,H
    pub(super) fn opcode_0x6c(&mut self) {
        ld_r8_r8!(self, l, h);
    }

    /// LD L,L
    pub(super) fn opcode_0x6d(&mut self) {
        // Self assignment.
    }

    /// LD L,(HL)
    pub(super) fn opcode_0x6e(&mut self) {
        ld_r8_hl!(self, l, (hl));
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
        ld_r8_r8!(self, a, b);
    }

    /// LD A,C
    pub(super) fn opcode_0x79(&mut self) {
        ld_r8_r8!(self, a, c);
    }

    /// LD A,D
    pub(super) fn opcode_0x7a(&mut self) {
        ld_r8_r8!(self, a, d);
    }

    /// LD A,E
    pub(super) fn opcode_0x7b(&mut self) {
        ld_r8_r8!(self, a, e);
    }

    /// LD A,H
    pub(super) fn opcode_0x7c(&mut self) {
        ld_r8_r8!(self, a, h);
    }

    /// LD A,L
    pub(super) fn opcode_0x7d(&mut self) {
        ld_r8_r8!(self, a, l);
    }

    /// LD A,(HL)
    pub(super) fn opcode_0x7e(&mut self) {
        ld_r8_hl!(self, a, (hl));
    }

    /// LD A,A
    pub(super) fn opcode_0x7f(&mut self) {
        // Self assignment.
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
