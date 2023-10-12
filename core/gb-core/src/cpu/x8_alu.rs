use super::Cpu;

// Completed, may need some refactoring.

macro_rules! alu_u8_reg_op {
    ($self:ident, $F:ident, (hl)) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte(address);

        let result = $self.$F(value);

        $self.write_byte(address, result);
    };

    ($self:ident, $F:ident, $reg:ident) => {
        $self.registers.$reg = $self.$F($self.registers.$reg);
    };
}

macro_rules! alu_acc_op {
    ($self:ident, $F:ident, (hl)) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte(address);

        $self.$F(value);
    };

    ($self:ident, $F:ident, $reg:ident) => {
        $self.$F($self.registers.$reg);
    };
}

impl Cpu {
    /// INC B
    pub(super) fn opcode_0x04(&mut self) {
        alu_u8_reg_op!(self, alu_inc, b);
    }

    /// DEC B
    pub(super) fn opcode_0x05(&mut self) {
        alu_u8_reg_op!(self, alu_dec, b);
    }

    /// INC C
    pub(super) fn opcode_0x0c(&mut self) {
        alu_u8_reg_op!(self, alu_inc, c);
    }

    /// DEC C
    pub(super) fn opcode_0x0d(&mut self) {
        alu_u8_reg_op!(self, alu_dec, c);
    }

    /// INC D
    pub(super) fn opcode_0x14(&mut self) {
        alu_u8_reg_op!(self, alu_inc, d);
    }

    /// DEC D
    pub(super) fn opcode_0x15(&mut self) {
        alu_u8_reg_op!(self, alu_dec, d);
    }

    /// INC E
    pub(super) fn opcode_0x1c(&mut self) {
        alu_u8_reg_op!(self, alu_inc, e);
    }

    /// DEC E
    pub(super) fn opcode_0x1d(&mut self) {
        alu_u8_reg_op!(self, alu_dec, e);
    }

    /// INC H
    pub(super) fn opcode_0x24(&mut self) {
        alu_u8_reg_op!(self, alu_inc, h);
    }

    /// DEC H
    pub(super) fn opcode_0x25(&mut self) {
        alu_u8_reg_op!(self, alu_dec, h);
    }

    /// DAA
    pub(super) fn opcode_0x27(&mut self) {
        self.alu_daa();
    }

    /// INC L
    pub(super) fn opcode_0x2c(&mut self) {
        alu_u8_reg_op!(self, alu_inc, l);
    }

    /// DEC L
    pub(super) fn opcode_0x2d(&mut self) {
        alu_u8_reg_op!(self, alu_dec, l);
    }

    /// CPL
    pub(super) fn opcode_0x2f(&mut self) {
        self.alu_cpl();
    }

    /// INC (HL)
    pub(super) fn opcode_0x34(&mut self) {
        alu_u8_reg_op!(self, alu_inc, (hl));
    }

    /// DEC (HL)
    pub(super) fn opcode_0x35(&mut self) {
        alu_u8_reg_op!(self, alu_dec, (hl));
    }

    /// SCF
    pub(super) fn opcode_0x37(&mut self) {
        self.alu_scf();
    }

    /// INC A
    pub(super) fn opcode_0x3c(&mut self) {
        alu_u8_reg_op!(self, alu_inc, a);
    }

    /// DEC A
    pub(super) fn opcode_0x3d(&mut self) {
        alu_u8_reg_op!(self, alu_dec, a);
    }

    /// CCF
    pub(super) fn opcode_0x3f(&mut self) {
        self.alu_ccf();
    }

    /// ADD A,B
    pub(super) fn opcode_0x80(&mut self) {
        alu_acc_op!(self, alu_add, b);
    }

    /// ADD A,C
    pub(super) fn opcode_0x81(&mut self) {
        alu_acc_op!(self, alu_add, c);
    }

    /// ADD A,D
    pub(super) fn opcode_0x82(&mut self) {
        alu_acc_op!(self, alu_add, d);
    }

    /// ADD A,E
    pub(super) fn opcode_0x83(&mut self) {
        alu_acc_op!(self, alu_add, e);
    }

    /// ADD A,H
    pub(super) fn opcode_0x84(&mut self) {
        alu_acc_op!(self, alu_add, h);
    }

    /// ADD A,L
    pub(super) fn opcode_0x85(&mut self) {
        alu_acc_op!(self, alu_add, l);
    }

    /// ADD A,(HL)
    pub(super) fn opcode_0x86(&mut self) {
        alu_acc_op!(self, alu_add, (hl));
    }

    /// ADD A,A
    pub(super) fn opcode_0x87(&mut self) {
        alu_acc_op!(self, alu_add, a);
    }

    /// ADC A,B
    pub(super) fn opcode_0x88(&mut self) {
        alu_acc_op!(self, alu_adc, b);
    }

    /// ADC A,C
    pub(super) fn opcode_0x89(&mut self) {
        alu_acc_op!(self, alu_adc, c);
    }

    /// ADC A,D
    pub(super) fn opcode_0x8a(&mut self) {
        alu_acc_op!(self, alu_adc, d);
    }

    /// ADC A,E
    pub(super) fn opcode_0x8b(&mut self) {
        alu_acc_op!(self, alu_adc, e);
    }

    /// ADC A,H
    pub(super) fn opcode_0x8c(&mut self) {
        alu_acc_op!(self, alu_adc, h);
    }

    /// ADC A,L
    pub(super) fn opcode_0x8d(&mut self) {
        alu_acc_op!(self, alu_adc, l);
    }

    /// ADC A,(HL)
    pub(super) fn opcode_0x8e(&mut self) {
        alu_acc_op!(self, alu_adc, (hl));
    }

    /// ADC A,A
    pub(super) fn opcode_0x8f(&mut self) {
        alu_acc_op!(self, alu_adc, a);
    }

    /// SUB A,B
    pub(super) fn opcode_0x90(&mut self) {
        alu_acc_op!(self, alu_sub, b);
    }

    /// SUB A,C
    pub(super) fn opcode_0x91(&mut self) {
        alu_acc_op!(self, alu_sub, c);
    }

    /// SUB A,D
    pub(super) fn opcode_0x92(&mut self) {
        alu_acc_op!(self, alu_sub, d);
    }

    /// SUB A,E
    pub(super) fn opcode_0x93(&mut self) {
        alu_acc_op!(self, alu_sub, e);
    }

    /// SUB A,H
    pub(super) fn opcode_0x94(&mut self) {
        alu_acc_op!(self, alu_sub, h);
    }

    /// SUB A,L
    pub(super) fn opcode_0x95(&mut self) {
        alu_acc_op!(self, alu_sub, l);
    }

    /// SUB A,(HL)
    pub(super) fn opcode_0x96(&mut self) {
        alu_acc_op!(self, alu_sub, (hl));
    }

    /// SUB A,A
    pub(super) fn opcode_0x97(&mut self) {
        alu_acc_op!(self, alu_sub, a);
    }

    /// SBC A,B
    pub(super) fn opcode_0x98(&mut self) {
        alu_acc_op!(self, alu_sbc, b);
    }

    /// SBC A,C
    pub(super) fn opcode_0x99(&mut self) {
        alu_acc_op!(self, alu_sbc, c);
    }

    /// SBC A,D
    pub(super) fn opcode_0x9a(&mut self) {
        alu_acc_op!(self, alu_sbc, d);
    }

    /// SBC A,E
    pub(super) fn opcode_0x9b(&mut self) {
        alu_acc_op!(self, alu_sbc, e);
    }

    /// SBC A,H
    pub(super) fn opcode_0x9c(&mut self) {
        alu_acc_op!(self, alu_sbc, h);
    }

    /// SBC A,L
    pub(super) fn opcode_0x9d(&mut self) {
        alu_acc_op!(self, alu_sbc, l);
    }

    /// SBC A,(HL)
    pub(super) fn opcode_0x9e(&mut self) {
        alu_acc_op!(self, alu_sbc, (hl));
    }

    /// SBC A,A
    pub(super) fn opcode_0x9f(&mut self) {
        alu_acc_op!(self, alu_sbc, a);
    }

    /// AND A,B
    pub(super) fn opcode_0xa0(&mut self) {
        alu_acc_op!(self, alu_and, b);
    }

    /// AND A,C
    pub(super) fn opcode_0xa1(&mut self) {
        alu_acc_op!(self, alu_and, c);
    }

    /// AND A,D
    pub(super) fn opcode_0xa2(&mut self) {
        alu_acc_op!(self, alu_and, d);
    }

    /// AND A,E
    pub(super) fn opcode_0xa3(&mut self) {
        alu_acc_op!(self, alu_and, e);
    }

    /// AND A,H
    pub(super) fn opcode_0xa4(&mut self) {
        alu_acc_op!(self, alu_and, h);
    }

    /// AND A,L
    pub(super) fn opcode_0xa5(&mut self) {
        alu_acc_op!(self, alu_and, l);
    }

    /// AND A,(HL)
    pub(super) fn opcode_0xa6(&mut self) {
        alu_acc_op!(self, alu_and, (hl));
    }

    /// AND A,A
    pub(super) fn opcode_0xa7(&mut self) {
        alu_acc_op!(self, alu_and, a);
    }

    /// XOR A,B
    pub(super) fn opcode_0xa8(&mut self) {
        alu_acc_op!(self, alu_xor, b);
    }

    /// XOR A,C
    pub(super) fn opcode_0xa9(&mut self) {
        alu_acc_op!(self, alu_xor, c);
    }

    /// XOR A,D
    pub(super) fn opcode_0xaa(&mut self) {
        alu_acc_op!(self, alu_xor, d);
    }

    /// XOR A,E
    pub(super) fn opcode_0xab(&mut self) {
        alu_acc_op!(self, alu_xor, e);
    }

    /// XOR A,H
    pub(super) fn opcode_0xac(&mut self) {
        alu_acc_op!(self, alu_xor, h);
    }

    /// XOR A,L
    pub(super) fn opcode_0xad(&mut self) {
        alu_acc_op!(self, alu_xor, l);
    }

    /// XOR A,(HL)
    pub(super) fn opcode_0xae(&mut self) {
        alu_acc_op!(self, alu_xor, (hl));
    }

    /// XOR A,A
    pub(super) fn opcode_0xaf(&mut self) {
        alu_acc_op!(self, alu_xor, a);
    }

    /// OR A,B
    pub(super) fn opcode_0xb0(&mut self) {
        alu_acc_op!(self, alu_or, b);
    }

    /// OR A,C
    pub(super) fn opcode_0xb1(&mut self) {
        alu_acc_op!(self, alu_or, c);
    }

    /// OR A,D
    pub(super) fn opcode_0xb2(&mut self) {
        alu_acc_op!(self, alu_or, d);
    }

    /// OR A,E
    pub(super) fn opcode_0xb3(&mut self) {
        alu_acc_op!(self, alu_or, e);
    }

    /// OR A,H
    pub(super) fn opcode_0xb4(&mut self) {
        alu_acc_op!(self, alu_or, h);
    }

    /// OR A,L
    pub(super) fn opcode_0xb5(&mut self) {
        alu_acc_op!(self, alu_or, l);
    }

    /// OR A,(HL)
    pub(super) fn opcode_0xb6(&mut self) {
        alu_acc_op!(self, alu_or, (hl));
    }

    /// OR A,A
    pub(super) fn opcode_0xb7(&mut self) {
        alu_acc_op!(self, alu_or, a);
    }

    /// CP A,B
    pub(super) fn opcode_0xb8(&mut self) {
        alu_acc_op!(self, alu_cp, b);
    }

    /// CP A,C
    pub(super) fn opcode_0xb9(&mut self) {
        alu_acc_op!(self, alu_cp, c);
    }

    /// CP A,D
    pub(super) fn opcode_0xba(&mut self) {
        alu_acc_op!(self, alu_cp, d);
    }

    /// CP A,E
    pub(super) fn opcode_0xbb(&mut self) {
        alu_acc_op!(self, alu_cp, e);
    }

    /// CP A,H
    pub(super) fn opcode_0xbc(&mut self) {
        alu_acc_op!(self, alu_cp, h);
    }

    /// CP A,L
    pub(super) fn opcode_0xbd(&mut self) {
        alu_acc_op!(self, alu_cp, l);
    }

    /// CP A,(HL)
    pub(super) fn opcode_0xbe(&mut self) {
        alu_acc_op!(self, alu_cp, (hl));
    }

    /// CP A,A
    pub(super) fn opcode_0xbf(&mut self) {
        alu_acc_op!(self, alu_cp, a);
    }

    /// ADD A,u8
    pub(super) fn opcode_0xc6(&mut self) {
        let value = self.read_byte_operand();

        self.alu_add(value);
    }

    /// ADC A,u8
    pub(super) fn opcode_0xce(&mut self) {
        let value = self.read_byte_operand();

        self.alu_adc(value);
    }

    /// SUB A,u8
    pub(super) fn opcode_0xd6(&mut self) {
        let value = self.read_byte_operand();

        self.alu_sub(value);
    }

    /// SBC A,u8
    pub(super) fn opcode_0xde(&mut self) {
        let value = self.read_byte_operand();

        self.alu_sbc(value);
    }

    /// AND A,u8
    pub(super) fn opcode_0xe6(&mut self) {
        let value = self.read_byte_operand();

        self.alu_and(value);
    }

    /// XOR A,u8
    pub(super) fn opcode_0xee(&mut self) {
        let value = self.read_byte_operand();

        self.alu_xor(value);
    }

    /// OR A,u8
    pub(super) fn opcode_0xf6(&mut self) {
        let value = self.read_byte_operand();

        self.alu_or(value);
    }

    /// CP A,u8
    pub(super) fn opcode_0xfe(&mut self) {
        let value = self.read_byte_operand();

        self.alu_cp(value);
    }
}
