use super::Cpu;

// Completed, may need some refactoring.

impl Cpu {
    pub(super) fn opcode_0x04(&mut self) {
        // INC B
        self.registers.b = self.inc_increment(self.registers.b);
    }

    pub(super) fn opcode_0x05(&mut self) {
        // DEC B
        self.registers.b = self.dec_decrement(self.registers.b);
    }

    pub(super) fn opcode_0x0c(&mut self) {
        // INC C
        self.registers.c = self.inc_increment(self.registers.c);
    }

    pub(super) fn opcode_0x0d(&mut self) {
        // DEC C
        self.registers.c = self.dec_decrement(self.registers.c);
    }

    pub(super) fn opcode_0x14(&mut self) {
        // INC D
        self.registers.d = self.inc_increment(self.registers.d);
    }

    pub(super) fn opcode_0x15(&mut self) {
        // DEC D
        self.registers.d = self.dec_decrement(self.registers.d);
    }

    pub(super) fn opcode_0x1c(&mut self) {
        // INC E
        self.registers.e = self.inc_increment(self.registers.e);
    }

    pub(super) fn opcode_0x1d(&mut self) {
        // DEC E
        self.registers.e = self.dec_decrement(self.registers.e);
    }

    pub(super) fn opcode_0x24(&mut self) {
        // INC H
        self.registers.h = self.inc_increment(self.registers.h);
    }

    pub(super) fn opcode_0x25(&mut self) {
        // DEC H
        self.registers.h = self.dec_decrement(self.registers.h);
    }

    pub(super) fn opcode_0x27(&mut self) {
        // DAA
        self.daa_decimal_adjust_accumulator();
    }

    pub(super) fn opcode_0x2c(&mut self) {
        // INC L
        self.registers.l = self.inc_increment(self.registers.l);
    }

    pub(super) fn opcode_0x2d(&mut self) {
        // DEC L
        self.registers.l = self.dec_decrement(self.registers.l);
    }

    pub(super) fn opcode_0x2f(&mut self) {
        // CPL
        self.cpl_complement_accumulator();
    }

    pub(super) fn opcode_0x34(&mut self) {
        // INC (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);
        let result = self.inc_increment(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_0x35(&mut self) {
        // DEC (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);
        let result = self.dec_decrement(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_0x37(&mut self) {
        // SCF
        self.scf_set_carry_flag();
    }

    pub(super) fn opcode_0x3c(&mut self) {
        // INC A
        self.registers.accumulator = self.inc_increment(self.registers.accumulator);
    }

    pub(super) fn opcode_0x3d(&mut self) {
        // DEC A
        self.registers.accumulator = self.dec_decrement(self.registers.accumulator);
    }

    pub(super) fn opcode_0x3f(&mut self) {
        // CCF
        self.ccf_complement_carry_flag();
    }

    pub(super) fn opcode_0x80(&mut self) {
        // ADD A,B
        self.add_to_accumulator(self.registers.b);
    }

    pub(super) fn opcode_0x81(&mut self) {
        // ADD A,C
        self.add_to_accumulator(self.registers.c);
    }

    pub(super) fn opcode_0x82(&mut self) {
        // ADD A,D
        self.add_to_accumulator(self.registers.d);
    }

    pub(super) fn opcode_0x83(&mut self) {
        // ADD A,E
        self.add_to_accumulator(self.registers.e);
    }

    pub(super) fn opcode_0x84(&mut self) {
        // ADD A,H
        self.add_to_accumulator(self.registers.h);
    }

    pub(super) fn opcode_0x85(&mut self) {
        // ADD A,L
        self.add_to_accumulator(self.registers.l);
    }

    pub(super) fn opcode_0x86(&mut self) {
        // ADD A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.add_to_accumulator(value);
    }

    pub(super) fn opcode_0x87(&mut self) {
        // ADD A,A
        self.add_to_accumulator(self.registers.accumulator);
    }

    pub(super) fn opcode_0x88(&mut self) {
        // ADC A,B
        self.add_to_accumulator_with_carry(self.registers.b);
    }

    pub(super) fn opcode_0x89(&mut self) {
        // ADC A,C
        self.add_to_accumulator_with_carry(self.registers.c);
    }

    pub(super) fn opcode_0x8a(&mut self) {
        // ADC A,D
        self.add_to_accumulator_with_carry(self.registers.d);
    }

    pub(super) fn opcode_0x8b(&mut self) {
        // ADC A,E
        self.add_to_accumulator_with_carry(self.registers.e);
    }

    pub(super) fn opcode_0x8c(&mut self) {
        // ADC A,H
        self.add_to_accumulator_with_carry(self.registers.h);
    }

    pub(super) fn opcode_0x8d(&mut self) {
        // ADC A,L
        self.add_to_accumulator_with_carry(self.registers.l);
    }

    pub(super) fn opcode_0x8e(&mut self) {
        // ADC A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.add_to_accumulator_with_carry(value);
    }

    pub(super) fn opcode_0x8f(&mut self) {
        // ADC A,A
        self.add_to_accumulator_with_carry(self.registers.accumulator);
    }

    pub(super) fn opcode_0x90(&mut self) {
        // SUB A,B
        self.sub_from_accumulator(self.registers.b);
    }

    pub(super) fn opcode_0x91(&mut self) {
        // SUB A,C
        self.sub_from_accumulator(self.registers.c);
    }

    pub(super) fn opcode_0x92(&mut self) {
        // SUB A,D
        self.sub_from_accumulator(self.registers.d);
    }

    pub(super) fn opcode_0x93(&mut self) {
        // SUB A,E
        self.sub_from_accumulator(self.registers.e);
    }

    pub(super) fn opcode_0x94(&mut self) {
        // SUB A,H
        self.sub_from_accumulator(self.registers.h);
    }

    pub(super) fn opcode_0x95(&mut self) {
        // SUB A,L
        self.sub_from_accumulator(self.registers.l);
    }

    pub(super) fn opcode_0x96(&mut self) {
        // SUB A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.sub_from_accumulator(value);
    }

    pub(super) fn opcode_0x97(&mut self) {
        // SUB A,A
        self.sub_from_accumulator(self.registers.accumulator);
    }

    pub(super) fn opcode_0x98(&mut self) {
        // SBC A,B
        self.sub_from_accumulator_with_carry(self.registers.b);
    }

    pub(super) fn opcode_0x99(&mut self) {
        // SBC A,C
        self.sub_from_accumulator_with_carry(self.registers.c);
    }

    pub(super) fn opcode_0x9a(&mut self) {
        // SBC A,D
        self.sub_from_accumulator_with_carry(self.registers.d);
    }

    pub(super) fn opcode_0x9b(&mut self) {
        // SBC A,E
        self.sub_from_accumulator_with_carry(self.registers.e);
    }

    pub(super) fn opcode_0x9c(&mut self) {
        // SBC A,H
        self.sub_from_accumulator_with_carry(self.registers.h);
    }

    pub(super) fn opcode_0x9d(&mut self) {
        // SBC A,L
        self.sub_from_accumulator_with_carry(self.registers.l);
    }

    pub(super) fn opcode_0x9e(&mut self) {
        // SBC A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.sub_from_accumulator_with_carry(value);
    }

    pub(super) fn opcode_0x9f(&mut self) {
        // SBC A,A
        self.sub_from_accumulator_with_carry(self.registers.accumulator);
    }

    pub(super) fn opcode_0xa0(&mut self) {
        // AND A,B
        self.and_with_accumulator(self.registers.b);
    }

    pub(super) fn opcode_0xa1(&mut self) {
        // AND A,C
        self.and_with_accumulator(self.registers.c);
    }

    pub(super) fn opcode_0xa2(&mut self) {
        // AND A,D
        self.and_with_accumulator(self.registers.d);
    }

    pub(super) fn opcode_0xa3(&mut self) {
        // AND A,E
        self.and_with_accumulator(self.registers.e);
    }

    pub(super) fn opcode_0xa4(&mut self) {
        // AND A,H
        self.and_with_accumulator(self.registers.h);
    }

    pub(super) fn opcode_0xa5(&mut self) {
        // AND A,L
        self.and_with_accumulator(self.registers.l);
    }

    pub(super) fn opcode_0xa6(&mut self) {
        // AND A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.and_with_accumulator(value);
    }

    pub(super) fn opcode_0xa7(&mut self) {
        // AND A,A
        self.and_with_accumulator(self.registers.accumulator);
    }

    pub(super) fn opcode_0xa8(&mut self) {
        // XOR A,B
        self.xor_with_accumulator(self.registers.b);
    }

    pub(super) fn opcode_0xa9(&mut self) {
        // XOR A,C
        self.xor_with_accumulator(self.registers.c);
    }

    pub(super) fn opcode_0xaa(&mut self) {
        // XOR A,D
        self.xor_with_accumulator(self.registers.d);
    }

    pub(super) fn opcode_0xab(&mut self) {
        // XOR A,E
        self.xor_with_accumulator(self.registers.e);
    }

    pub(super) fn opcode_0xac(&mut self) {
        // XOR A,H
        self.xor_with_accumulator(self.registers.h);
    }

    pub(super) fn opcode_0xad(&mut self) {
        // XOR A,L
        self.xor_with_accumulator(self.registers.l);
    }

    pub(super) fn opcode_0xae(&mut self) {
        // XOR A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.xor_with_accumulator(value);
    }

    pub(super) fn opcode_0xaf(&mut self) {
        // XOR A,A
        self.xor_with_accumulator(self.registers.accumulator);
    }

    pub(super) fn opcode_0xb0(&mut self) {
        // OR A,B
        self.or_with_accumulator(self.registers.b);
    }

    pub(super) fn opcode_0xb1(&mut self) {
        // OR A,C
        self.or_with_accumulator(self.registers.c);
    }

    pub(super) fn opcode_0xb2(&mut self) {
        // OR A,D
        self.or_with_accumulator(self.registers.d);
    }

    pub(super) fn opcode_0xb3(&mut self) {
        // OR A,E
        self.or_with_accumulator(self.registers.e);
    }

    pub(super) fn opcode_0xb4(&mut self) {
        // OR A,H
        self.or_with_accumulator(self.registers.h);
    }

    pub(super) fn opcode_0xb5(&mut self) {
        // OR A,L
        self.or_with_accumulator(self.registers.l);
    }

    pub(super) fn opcode_0xb6(&mut self) {
        // OR A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.or_with_accumulator(value);
    }

    pub(super) fn opcode_0xb7(&mut self) {
        // OR A,A
        self.or_with_accumulator(self.registers.accumulator);
    }

    pub(super) fn opcode_0xb8(&mut self) {
        // CP A,B
        self.cp_compare_with_accumulator(self.registers.b);
    }

    pub(super) fn opcode_0xb9(&mut self) {
        // CP A,C
        self.cp_compare_with_accumulator(self.registers.c);
    }

    pub(super) fn opcode_0xba(&mut self) {
        // CP A,D
        self.cp_compare_with_accumulator(self.registers.d);
    }

    pub(super) fn opcode_0xbb(&mut self) {
        // CP A,E
        self.cp_compare_with_accumulator(self.registers.e);
    }

    pub(super) fn opcode_0xbc(&mut self) {
        // CP A,H
        self.cp_compare_with_accumulator(self.registers.h);
    }

    pub(super) fn opcode_0xbd(&mut self) {
        // CP A,L
        self.cp_compare_with_accumulator(self.registers.l);
    }

    pub(super) fn opcode_0xbe(&mut self) {
        // CP A,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.cp_compare_with_accumulator(value);
    }

    pub(super) fn opcode_0xbf(&mut self) {
        // CP A,A
        self.cp_compare_with_accumulator(self.registers.accumulator);
    }

    pub(super) fn opcode_0xc6(&mut self) {
        // ADD A,u8
        let value = self.read_byte_operand();

        self.add_to_accumulator(value);
    }

    pub(super) fn opcode_0xce(&mut self) {
        // ADC A,u8
        let value = self.read_byte_operand();

        self.add_to_accumulator_with_carry(value);
    }

    pub(super) fn opcode_0xd6(&mut self) {
        // SUB A,u8
        let value = self.read_byte_operand();

        self.sub_from_accumulator(value);
    }

    pub(super) fn opcode_0xde(&mut self) {
        // SBC A,u8
        let value = self.read_byte_operand();

        self.sub_from_accumulator_with_carry(value);
    }

    pub(super) fn opcode_0xe6(&mut self) {
        // AND A,u8
        let value = self.read_byte_operand();

        self.and_with_accumulator(value);
    }

    pub(super) fn opcode_0xee(&mut self) {
        // XOR A,u8
        let value = self.read_byte_operand();

        self.xor_with_accumulator(value);
    }

    pub(super) fn opcode_0xf6(&mut self) {
        // OR A,u8
        let value = self.read_byte_operand();

        self.or_with_accumulator(value);
    }

    pub(super) fn opcode_0xfe(&mut self) {
        // CP A,u8
        let value = self.read_byte_operand();

        self.cp_compare_with_accumulator(value);
    }
}
