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
        alu_u8_reg_op!(self, inc_increment, b);
    }

    /// DEC B
    pub(super) fn opcode_0x05(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, b);
    }

    /// INC C
    pub(super) fn opcode_0x0c(&mut self) {
        alu_u8_reg_op!(self, inc_increment, c);
    }

    /// DEC C
    pub(super) fn opcode_0x0d(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, c);
    }

    /// INC D
    pub(super) fn opcode_0x14(&mut self) {
        alu_u8_reg_op!(self, inc_increment, d);
    }

    /// DEC D
    pub(super) fn opcode_0x15(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, d);
    }

    /// INC E
    pub(super) fn opcode_0x1c(&mut self) {
        alu_u8_reg_op!(self, inc_increment, e);
    }

    /// DEC E
    pub(super) fn opcode_0x1d(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, e);
    }

    /// INC H
    pub(super) fn opcode_0x24(&mut self) {
        alu_u8_reg_op!(self, inc_increment, h);
    }

    /// DEC H
    pub(super) fn opcode_0x25(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, h);
    }

    /// DAA
    pub(super) fn opcode_0x27(&mut self) {
        self.daa_decimal_adjust_accumulator();
    }

    /// INC L
    pub(super) fn opcode_0x2c(&mut self) {
        alu_u8_reg_op!(self, inc_increment, l);
    }

    /// DEC L
    pub(super) fn opcode_0x2d(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, l);
    }

    /// CPL
    pub(super) fn opcode_0x2f(&mut self) {
        self.cpl_complement_accumulator();
    }

    /// INC (HL)
    pub(super) fn opcode_0x34(&mut self) {
        alu_u8_reg_op!(self, inc_increment, (hl));
    }

    /// DEC (HL)
    pub(super) fn opcode_0x35(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, (hl));
    }

    /// SCF
    pub(super) fn opcode_0x37(&mut self) {
        self.scf_set_carry_flag();
    }

    /// INC A
    pub(super) fn opcode_0x3c(&mut self) {
        alu_u8_reg_op!(self, inc_increment, a);
    }

    /// DEC A
    pub(super) fn opcode_0x3d(&mut self) {
        alu_u8_reg_op!(self, dec_decrement, a);
    }

    /// CCF
    pub(super) fn opcode_0x3f(&mut self) {
        self.ccf_complement_carry_flag();
    }

    /// ADD A,B
    pub(super) fn opcode_0x80(&mut self) {
        alu_acc_op!(self, add_to_accumulator, b);
    }

    /// ADD A,C
    pub(super) fn opcode_0x81(&mut self) {
        alu_acc_op!(self, add_to_accumulator, c);
    }

    /// ADD A,D
    pub(super) fn opcode_0x82(&mut self) {
        alu_acc_op!(self, add_to_accumulator, d);
    }

    /// ADD A,E
    pub(super) fn opcode_0x83(&mut self) {
        alu_acc_op!(self, add_to_accumulator, e);
    }

    /// ADD A,H
    pub(super) fn opcode_0x84(&mut self) {
        alu_acc_op!(self, add_to_accumulator, h);
    }

    /// ADD A,L
    pub(super) fn opcode_0x85(&mut self) {
        alu_acc_op!(self, add_to_accumulator, l);
    }

    /// ADD A,(HL)
    pub(super) fn opcode_0x86(&mut self) {
        alu_acc_op!(self, add_to_accumulator, (hl));
    }

    /// ADD A,A
    pub(super) fn opcode_0x87(&mut self) {
        alu_acc_op!(self, add_to_accumulator, a);
    }

    /// ADC A,B
    pub(super) fn opcode_0x88(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, b);
    }

    /// ADC A,C
    pub(super) fn opcode_0x89(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, c);
    }

    /// ADC A,D
    pub(super) fn opcode_0x8a(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, d);
    }

    /// ADC A,E
    pub(super) fn opcode_0x8b(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, e);
    }

    /// ADC A,H
    pub(super) fn opcode_0x8c(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, h);
    }

    /// ADC A,L
    pub(super) fn opcode_0x8d(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, l);
    }

    /// ADC A,(HL)
    pub(super) fn opcode_0x8e(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, (hl));
    }

    /// ADC A,A
    pub(super) fn opcode_0x8f(&mut self) {
        alu_acc_op!(self, add_to_accumulator_with_carry, a);
    }

    /// SUB A,B
    pub(super) fn opcode_0x90(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, b);
    }

    /// SUB A,C
    pub(super) fn opcode_0x91(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, c);
    }

    /// SUB A,D
    pub(super) fn opcode_0x92(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, d);
    }

    /// SUB A,E
    pub(super) fn opcode_0x93(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, e);
    }

    /// SUB A,H
    pub(super) fn opcode_0x94(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, h);
    }

    /// SUB A,L
    pub(super) fn opcode_0x95(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, l);
    }

    /// SUB A,(HL)
    pub(super) fn opcode_0x96(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, (hl));
    }

    /// SUB A,A
    pub(super) fn opcode_0x97(&mut self) {
        alu_acc_op!(self, sub_from_accumulator, a);
    }

    /// SBC A,B
    pub(super) fn opcode_0x98(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, b);
    }

    /// SBC A,C
    pub(super) fn opcode_0x99(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, c);
    }

    /// SBC A,D
    pub(super) fn opcode_0x9a(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, d);
    }

    /// SBC A,E
    pub(super) fn opcode_0x9b(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, e);
    }

    /// SBC A,H
    pub(super) fn opcode_0x9c(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, h);
    }

    /// SBC A,L
    pub(super) fn opcode_0x9d(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, l);
    }

    /// SBC A,(HL)
    pub(super) fn opcode_0x9e(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, (hl));
    }

    /// SBC A,A
    pub(super) fn opcode_0x9f(&mut self) {
        alu_acc_op!(self, sub_from_accumulator_with_carry, a);
    }

    /// AND A,B
    pub(super) fn opcode_0xa0(&mut self) {
        alu_acc_op!(self, and_with_accumulator, b);
    }

    /// AND A,C
    pub(super) fn opcode_0xa1(&mut self) {
        alu_acc_op!(self, and_with_accumulator, c);
    }

    /// AND A,D
    pub(super) fn opcode_0xa2(&mut self) {
        alu_acc_op!(self, and_with_accumulator, d);
    }

    /// AND A,E
    pub(super) fn opcode_0xa3(&mut self) {
        alu_acc_op!(self, and_with_accumulator, e);
    }

    /// AND A,H
    pub(super) fn opcode_0xa4(&mut self) {
        alu_acc_op!(self, and_with_accumulator, h);
    }

    /// AND A,L
    pub(super) fn opcode_0xa5(&mut self) {
        alu_acc_op!(self, and_with_accumulator, l);
    }

    /// AND A,(HL)
    pub(super) fn opcode_0xa6(&mut self) {
        alu_acc_op!(self, and_with_accumulator, (hl));
    }

    /// AND A,A
    pub(super) fn opcode_0xa7(&mut self) {
        alu_acc_op!(self, and_with_accumulator, a);
    }

    /// XOR A,B
    pub(super) fn opcode_0xa8(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, b);
    }

    /// XOR A,C
    pub(super) fn opcode_0xa9(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, c);
    }

    /// XOR A,D
    pub(super) fn opcode_0xaa(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, d);
    }

    /// XOR A,E
    pub(super) fn opcode_0xab(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, e);
    }

    /// XOR A,H
    pub(super) fn opcode_0xac(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, h);
    }

    /// XOR A,L
    pub(super) fn opcode_0xad(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, l);
    }

    /// XOR A,(HL)
    pub(super) fn opcode_0xae(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, (hl));
    }

    /// XOR A,A
    pub(super) fn opcode_0xaf(&mut self) {
        alu_acc_op!(self, xor_with_accumulator, a);
    }

    /// OR A,B
    pub(super) fn opcode_0xb0(&mut self) {
        alu_acc_op!(self, or_with_accumulator, b);
    }

    /// OR A,C
    pub(super) fn opcode_0xb1(&mut self) {
        alu_acc_op!(self, or_with_accumulator, c);
    }

    /// OR A,D
    pub(super) fn opcode_0xb2(&mut self) {
        alu_acc_op!(self, or_with_accumulator, d);
    }

    /// OR A,E
    pub(super) fn opcode_0xb3(&mut self) {
        alu_acc_op!(self, or_with_accumulator, e);
    }

    /// OR A,H
    pub(super) fn opcode_0xb4(&mut self) {
        alu_acc_op!(self, or_with_accumulator, h);
    }

    /// OR A,L
    pub(super) fn opcode_0xb5(&mut self) {
        alu_acc_op!(self, or_with_accumulator, l);
    }

    /// OR A,(HL)
    pub(super) fn opcode_0xb6(&mut self) {
        alu_acc_op!(self, or_with_accumulator, (hl));
    }

    /// OR A,A
    pub(super) fn opcode_0xb7(&mut self) {
        alu_acc_op!(self, or_with_accumulator, a);
    }

    /// CP A,B
    pub(super) fn opcode_0xb8(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, b);
    }

    /// CP A,C
    pub(super) fn opcode_0xb9(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, c);
    }

    /// CP A,D
    pub(super) fn opcode_0xba(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, d);
    }

    /// CP A,E
    pub(super) fn opcode_0xbb(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, e);
    }

    /// CP A,H
    pub(super) fn opcode_0xbc(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, h);
    }

    /// CP A,L
    pub(super) fn opcode_0xbd(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, l);
    }

    /// CP A,(HL)
    pub(super) fn opcode_0xbe(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, (hl));
    }

    /// CP A,A
    pub(super) fn opcode_0xbf(&mut self) {
        alu_acc_op!(self, cp_compare_with_accumulator, a);
    }

    /// ADD A,u8
    pub(super) fn opcode_0xc6(&mut self) {
        let value = self.read_byte_operand();

        self.add_to_accumulator(value);
    }

    /// ADC A,u8
    pub(super) fn opcode_0xce(&mut self) {
        let value = self.read_byte_operand();

        self.add_to_accumulator_with_carry(value);
    }

    /// SUB A,u8
    pub(super) fn opcode_0xd6(&mut self) {
        let value = self.read_byte_operand();

        self.sub_from_accumulator(value);
    }

    /// SBC A,u8
    pub(super) fn opcode_0xde(&mut self) {
        let value = self.read_byte_operand();

        self.sub_from_accumulator_with_carry(value);
    }

    /// AND A,u8
    pub(super) fn opcode_0xe6(&mut self) {
        let value = self.read_byte_operand();

        self.and_with_accumulator(value);
    }

    /// XOR A,u8
    pub(super) fn opcode_0xee(&mut self) {
        let value = self.read_byte_operand();

        self.xor_with_accumulator(value);
    }

    /// OR A,u8
    pub(super) fn opcode_0xf6(&mut self) {
        let value = self.read_byte_operand();

        self.or_with_accumulator(value);
    }

    /// CP A,u8
    pub(super) fn opcode_0xfe(&mut self) {
        let value = self.read_byte_operand();

        self.cp_compare_with_accumulator(value);
    }
}
