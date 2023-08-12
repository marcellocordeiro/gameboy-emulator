use super::Cpu;

// Completed, will definitely need some refactoring.

impl Cpu {
    pub(super) fn opcode_cb_0x00(&mut self) {
        // RLC B
        self.registers.b = self.bit_rotate_left_c(self.registers.b);
    }

    pub(super) fn opcode_cb_0x01(&mut self) {
        // RLC C
        self.registers.c = self.bit_rotate_left_c(self.registers.c);
    }

    pub(super) fn opcode_cb_0x02(&mut self) {
        // RLC D
        self.registers.d = self.bit_rotate_left_c(self.registers.d);
    }

    pub(super) fn opcode_cb_0x03(&mut self) {
        // RLC E
        self.registers.e = self.bit_rotate_left_c(self.registers.e);
    }

    pub(super) fn opcode_cb_0x04(&mut self) {
        // RLC H
        self.registers.h = self.bit_rotate_left_c(self.registers.h);
    }

    pub(super) fn opcode_cb_0x05(&mut self) {
        // RLC L
        self.registers.l = self.bit_rotate_left_c(self.registers.l);
    }

    pub(super) fn opcode_cb_0x06(&mut self) {
        // RLC (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_rotate_left_c(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x07(&mut self) {
        // RLC A
        self.registers.accumulator = self.bit_rotate_left_c(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x08(&mut self) {
        // RRC B
        self.registers.b = self.bit_rotate_right_c(self.registers.b);
    }

    pub(super) fn opcode_cb_0x09(&mut self) {
        // RRC C
        self.registers.c = self.bit_rotate_right_c(self.registers.c);
    }

    pub(super) fn opcode_cb_0x0a(&mut self) {
        // RRC D
        self.registers.d = self.bit_rotate_right_c(self.registers.d);
    }

    pub(super) fn opcode_cb_0x0b(&mut self) {
        // RRC E
        self.registers.e = self.bit_rotate_right_c(self.registers.e);
    }

    pub(super) fn opcode_cb_0x0c(&mut self) {
        // RRC H
        self.registers.h = self.bit_rotate_right_c(self.registers.h);
    }

    pub(super) fn opcode_cb_0x0d(&mut self) {
        // RRC L
        self.registers.l = self.bit_rotate_right_c(self.registers.l);
    }

    pub(super) fn opcode_cb_0x0e(&mut self) {
        // RRC (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_rotate_right_c(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x0f(&mut self) {
        // RRC A
        self.registers.accumulator = self.bit_rotate_right_c(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x10(&mut self) {
        // RL B
        self.registers.b = self.bit_rotate_left(self.registers.b);
    }

    pub(super) fn opcode_cb_0x11(&mut self) {
        // RL C
        self.registers.c = self.bit_rotate_left(self.registers.c);
    }

    pub(super) fn opcode_cb_0x12(&mut self) {
        // RL D
        self.registers.d = self.bit_rotate_left(self.registers.d);
    }

    pub(super) fn opcode_cb_0x13(&mut self) {
        // RL E
        self.registers.e = self.bit_rotate_left(self.registers.e);
    }

    pub(super) fn opcode_cb_0x14(&mut self) {
        // RL H
        self.registers.h = self.bit_rotate_left(self.registers.h);
    }

    pub(super) fn opcode_cb_0x15(&mut self) {
        // RL L
        self.registers.l = self.bit_rotate_left(self.registers.l);
    }

    pub(super) fn opcode_cb_0x16(&mut self) {
        // RL (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_rotate_left(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x17(&mut self) {
        // RL A
        self.registers.accumulator = self.bit_rotate_left(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x18(&mut self) {
        // RR B
        self.registers.b = self.bit_rotate_right(self.registers.b);
    }

    pub(super) fn opcode_cb_0x19(&mut self) {
        // RR C
        self.registers.c = self.bit_rotate_right(self.registers.c);
    }

    pub(super) fn opcode_cb_0x1a(&mut self) {
        // RR D
        self.registers.d = self.bit_rotate_right(self.registers.d);
    }

    pub(super) fn opcode_cb_0x1b(&mut self) {
        // RR E
        self.registers.e = self.bit_rotate_right(self.registers.e);
    }

    pub(super) fn opcode_cb_0x1c(&mut self) {
        // RR H
        self.registers.h = self.bit_rotate_right(self.registers.h);
    }

    pub(super) fn opcode_cb_0x1d(&mut self) {
        // RR L
        self.registers.l = self.bit_rotate_right(self.registers.l);
    }

    pub(super) fn opcode_cb_0x1e(&mut self) {
        // RR (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_rotate_right(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x1f(&mut self) {
        // RR A
        self.registers.accumulator = self.bit_rotate_right(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x20(&mut self) {
        // SLA B
        self.registers.b = self.bit_sla_arithmetic_shift_left(self.registers.b);
    }

    pub(super) fn opcode_cb_0x21(&mut self) {
        // SLA C
        self.registers.c = self.bit_sla_arithmetic_shift_left(self.registers.c);
    }

    pub(super) fn opcode_cb_0x22(&mut self) {
        // SLA D
        self.registers.d = self.bit_sla_arithmetic_shift_left(self.registers.d);
    }

    pub(super) fn opcode_cb_0x23(&mut self) {
        // SLA E
        self.registers.e = self.bit_sla_arithmetic_shift_left(self.registers.e);
    }

    pub(super) fn opcode_cb_0x24(&mut self) {
        // SLA H
        self.registers.h = self.bit_sla_arithmetic_shift_left(self.registers.h);
    }

    pub(super) fn opcode_cb_0x25(&mut self) {
        // SLA L
        self.registers.l = self.bit_sla_arithmetic_shift_left(self.registers.l);
    }

    pub(super) fn opcode_cb_0x26(&mut self) {
        // SLA (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_sla_arithmetic_shift_left(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x27(&mut self) {
        // SLA A
        self.registers.accumulator = self.bit_sla_arithmetic_shift_left(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x28(&mut self) {
        // SRA B
        self.registers.b = self.bit_sra_arithmetic_shift_right(self.registers.b);
    }

    pub(super) fn opcode_cb_0x29(&mut self) {
        // SRA C
        self.registers.c = self.bit_sra_arithmetic_shift_right(self.registers.c);
    }

    pub(super) fn opcode_cb_0x2a(&mut self) {
        // SRA D
        self.registers.d = self.bit_sra_arithmetic_shift_right(self.registers.d);
    }

    pub(super) fn opcode_cb_0x2b(&mut self) {
        // SRA E
        self.registers.e = self.bit_sra_arithmetic_shift_right(self.registers.e);
    }

    pub(super) fn opcode_cb_0x2c(&mut self) {
        // SRA H
        self.registers.h = self.bit_sra_arithmetic_shift_right(self.registers.h);
    }

    pub(super) fn opcode_cb_0x2d(&mut self) {
        // SRA L
        self.registers.l = self.bit_sra_arithmetic_shift_right(self.registers.l);
    }

    pub(super) fn opcode_cb_0x2e(&mut self) {
        // SRA (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_sra_arithmetic_shift_right(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x2f(&mut self) {
        // SRA A
        self.registers.accumulator =
            self.bit_sra_arithmetic_shift_right(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x30(&mut self) {
        // SWAP B
        self.registers.b = self.swap_nibbles(self.registers.b);
    }

    pub(super) fn opcode_cb_0x31(&mut self) {
        // SWAP C
        self.registers.c = self.swap_nibbles(self.registers.c);
    }

    pub(super) fn opcode_cb_0x32(&mut self) {
        // SWAP D
        self.registers.d = self.swap_nibbles(self.registers.d);
    }

    pub(super) fn opcode_cb_0x33(&mut self) {
        // SWAP E
        self.registers.e = self.swap_nibbles(self.registers.e);
    }

    pub(super) fn opcode_cb_0x34(&mut self) {
        // SWAP H
        self.registers.h = self.swap_nibbles(self.registers.h);
    }

    pub(super) fn opcode_cb_0x35(&mut self) {
        // SWAP L
        self.registers.l = self.swap_nibbles(self.registers.l);
    }

    pub(super) fn opcode_cb_0x36(&mut self) {
        // SWAP (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.swap_nibbles(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x37(&mut self) {
        // SWAP A
        self.registers.accumulator = self.swap_nibbles(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x38(&mut self) {
        // SRL B
        self.registers.b = self.bit_srl_logical_shift_right(self.registers.b);
    }

    pub(super) fn opcode_cb_0x39(&mut self) {
        // SRL C
        self.registers.c = self.bit_srl_logical_shift_right(self.registers.c);
    }

    pub(super) fn opcode_cb_0x3a(&mut self) {
        // SRL D
        self.registers.d = self.bit_srl_logical_shift_right(self.registers.d);
    }

    pub(super) fn opcode_cb_0x3b(&mut self) {
        // SRL E
        self.registers.e = self.bit_srl_logical_shift_right(self.registers.e);
    }

    pub(super) fn opcode_cb_0x3c(&mut self) {
        // SRL H
        self.registers.h = self.bit_srl_logical_shift_right(self.registers.h);
    }

    pub(super) fn opcode_cb_0x3d(&mut self) {
        // SRL L
        self.registers.l = self.bit_srl_logical_shift_right(self.registers.l);
    }

    pub(super) fn opcode_cb_0x3e(&mut self) {
        // SRL (HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_srl_logical_shift_right(value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x3f(&mut self) {
        // SRL A
        self.registers.accumulator = self.bit_srl_logical_shift_right(self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x40(&mut self) {
        // BIT 0,B
        self.bit_test_bit(0, self.registers.b);
    }

    pub(super) fn opcode_cb_0x41(&mut self) {
        // BIT 0,C
        self.bit_test_bit(0, self.registers.c);
    }

    pub(super) fn opcode_cb_0x42(&mut self) {
        // BIT 0,D
        self.bit_test_bit(0, self.registers.d);
    }

    pub(super) fn opcode_cb_0x43(&mut self) {
        // BIT 0,E
        self.bit_test_bit(0, self.registers.e);
    }

    pub(super) fn opcode_cb_0x44(&mut self) {
        // BIT 0,H
        self.bit_test_bit(0, self.registers.h);
    }

    pub(super) fn opcode_cb_0x45(&mut self) {
        // BIT 0,L
        self.bit_test_bit(0, self.registers.l);
    }

    pub(super) fn opcode_cb_0x46(&mut self) {
        // BIT 0,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(0, value);
    }

    pub(super) fn opcode_cb_0x47(&mut self) {
        // BIT 0,A
        self.bit_test_bit(0, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x48(&mut self) {
        // BIT 1,B
        self.bit_test_bit(1, self.registers.b);
    }

    pub(super) fn opcode_cb_0x49(&mut self) {
        // BIT 1,C
        self.bit_test_bit(1, self.registers.c);
    }

    pub(super) fn opcode_cb_0x4a(&mut self) {
        // BIT 1,D
        self.bit_test_bit(1, self.registers.d);
    }

    pub(super) fn opcode_cb_0x4b(&mut self) {
        // BIT 1,E
        self.bit_test_bit(1, self.registers.e);
    }

    pub(super) fn opcode_cb_0x4c(&mut self) {
        // BIT 1,H
        self.bit_test_bit(1, self.registers.h);
    }

    pub(super) fn opcode_cb_0x4d(&mut self) {
        // BIT 1,L
        self.bit_test_bit(1, self.registers.l);
    }

    pub(super) fn opcode_cb_0x4e(&mut self) {
        // BIT 1,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(1, value);
    }

    pub(super) fn opcode_cb_0x4f(&mut self) {
        // BIT 1,A
        self.bit_test_bit(1, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x50(&mut self) {
        // BIT 2,B
        self.bit_test_bit(2, self.registers.b);
    }

    pub(super) fn opcode_cb_0x51(&mut self) {
        // BIT 2,C
        self.bit_test_bit(2, self.registers.c);
    }

    pub(super) fn opcode_cb_0x52(&mut self) {
        // BIT 2,D
        self.bit_test_bit(2, self.registers.d);
    }

    pub(super) fn opcode_cb_0x53(&mut self) {
        // BIT 2,E
        self.bit_test_bit(2, self.registers.e);
    }

    pub(super) fn opcode_cb_0x54(&mut self) {
        // BIT 2,H
        self.bit_test_bit(2, self.registers.h);
    }

    pub(super) fn opcode_cb_0x55(&mut self) {
        // BIT 2,L
        self.bit_test_bit(2, self.registers.l);
    }

    pub(super) fn opcode_cb_0x56(&mut self) {
        // BIT 2,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(2, value);
    }

    pub(super) fn opcode_cb_0x57(&mut self) {
        // BIT 2,A
        self.bit_test_bit(2, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x58(&mut self) {
        // BIT 3,B
        self.bit_test_bit(3, self.registers.b);
    }

    pub(super) fn opcode_cb_0x59(&mut self) {
        // BIT 3,C
        self.bit_test_bit(3, self.registers.c);
    }

    pub(super) fn opcode_cb_0x5a(&mut self) {
        // BIT 3,D
        self.bit_test_bit(3, self.registers.d);
    }

    pub(super) fn opcode_cb_0x5b(&mut self) {
        // BIT 3,E
        self.bit_test_bit(3, self.registers.e);
    }

    pub(super) fn opcode_cb_0x5c(&mut self) {
        // BIT 3,H
        self.bit_test_bit(3, self.registers.h);
    }

    pub(super) fn opcode_cb_0x5d(&mut self) {
        // BIT 3,L
        self.bit_test_bit(3, self.registers.l);
    }

    pub(super) fn opcode_cb_0x5e(&mut self) {
        // BIT 3,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(3, value);
    }

    pub(super) fn opcode_cb_0x5f(&mut self) {
        // BIT 3,A
        self.bit_test_bit(3, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x60(&mut self) {
        // BIT 4,B
        self.bit_test_bit(4, self.registers.b);
    }

    pub(super) fn opcode_cb_0x61(&mut self) {
        // BIT 4,C
        self.bit_test_bit(4, self.registers.c);
    }

    pub(super) fn opcode_cb_0x62(&mut self) {
        // BIT 4,D
        self.bit_test_bit(4, self.registers.d);
    }

    pub(super) fn opcode_cb_0x63(&mut self) {
        // BIT 4,E
        self.bit_test_bit(4, self.registers.e);
    }

    pub(super) fn opcode_cb_0x64(&mut self) {
        // BIT 4,H
        self.bit_test_bit(4, self.registers.h);
    }

    pub(super) fn opcode_cb_0x65(&mut self) {
        // BIT 4,L
        self.bit_test_bit(4, self.registers.l);
    }

    pub(super) fn opcode_cb_0x66(&mut self) {
        // BIT 4,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(4, value);
    }

    pub(super) fn opcode_cb_0x67(&mut self) {
        // BIT 4,A
        self.bit_test_bit(4, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x68(&mut self) {
        // BIT 5,B
        self.bit_test_bit(5, self.registers.b);
    }

    pub(super) fn opcode_cb_0x69(&mut self) {
        // BIT 5,C
        self.bit_test_bit(5, self.registers.c);
    }

    pub(super) fn opcode_cb_0x6a(&mut self) {
        // BIT 5,D
        self.bit_test_bit(5, self.registers.d);
    }

    pub(super) fn opcode_cb_0x6b(&mut self) {
        // BIT 5,E
        self.bit_test_bit(5, self.registers.e);
    }

    pub(super) fn opcode_cb_0x6c(&mut self) {
        // BIT 5,H
        self.bit_test_bit(5, self.registers.h);
    }

    pub(super) fn opcode_cb_0x6d(&mut self) {
        // BIT 5,L
        self.bit_test_bit(5, self.registers.l);
    }

    pub(super) fn opcode_cb_0x6e(&mut self) {
        // BIT 5,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(5, value);
    }

    pub(super) fn opcode_cb_0x6f(&mut self) {
        // BIT 5,A
        self.bit_test_bit(5, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x70(&mut self) {
        // BIT 6,B
        self.bit_test_bit(6, self.registers.b);
    }

    pub(super) fn opcode_cb_0x71(&mut self) {
        // BIT 6,C
        self.bit_test_bit(6, self.registers.c);
    }

    pub(super) fn opcode_cb_0x72(&mut self) {
        // BIT 6,D
        self.bit_test_bit(6, self.registers.d);
    }

    pub(super) fn opcode_cb_0x73(&mut self) {
        // BIT 6,E
        self.bit_test_bit(6, self.registers.e);
    }

    pub(super) fn opcode_cb_0x74(&mut self) {
        // BIT 6,H
        self.bit_test_bit(6, self.registers.h);
    }

    pub(super) fn opcode_cb_0x75(&mut self) {
        // BIT 6,L
        self.bit_test_bit(6, self.registers.l);
    }

    pub(super) fn opcode_cb_0x76(&mut self) {
        // BIT 6,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(6, value);
    }

    pub(super) fn opcode_cb_0x77(&mut self) {
        // BIT 6,A
        self.bit_test_bit(6, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x78(&mut self) {
        // BIT 7,B
        self.bit_test_bit(7, self.registers.b);
    }

    pub(super) fn opcode_cb_0x79(&mut self) {
        // BIT 7,C
        self.bit_test_bit(7, self.registers.c);
    }

    pub(super) fn opcode_cb_0x7a(&mut self) {
        // BIT 7,D
        self.bit_test_bit(7, self.registers.d);
    }

    pub(super) fn opcode_cb_0x7b(&mut self) {
        // BIT 7,E
        self.bit_test_bit(7, self.registers.e);
    }

    pub(super) fn opcode_cb_0x7c(&mut self) {
        // BIT 7,H
        self.bit_test_bit(7, self.registers.h);
    }

    pub(super) fn opcode_cb_0x7d(&mut self) {
        // BIT 7,L
        self.bit_test_bit(7, self.registers.l);
    }

    pub(super) fn opcode_cb_0x7e(&mut self) {
        // BIT 7,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        self.bit_test_bit(7, value);
    }

    pub(super) fn opcode_cb_0x7f(&mut self) {
        // BIT 7,A
        self.bit_test_bit(7, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x80(&mut self) {
        // RES 0,B
        self.registers.b = self.bit_reset_bit(0, self.registers.b);
    }

    pub(super) fn opcode_cb_0x81(&mut self) {
        // RES 0,C
        self.registers.c = self.bit_reset_bit(0, self.registers.c);
    }

    pub(super) fn opcode_cb_0x82(&mut self) {
        // RES 0,D
        self.registers.d = self.bit_reset_bit(0, self.registers.d);
    }

    pub(super) fn opcode_cb_0x83(&mut self) {
        // RES 0,E
        self.registers.e = self.bit_reset_bit(0, self.registers.e);
    }

    pub(super) fn opcode_cb_0x84(&mut self) {
        // RES 0,H
        self.registers.h = self.bit_reset_bit(0, self.registers.h);
    }

    pub(super) fn opcode_cb_0x85(&mut self) {
        // RES 0,L
        self.registers.l = self.bit_reset_bit(0, self.registers.l);
    }

    pub(super) fn opcode_cb_0x86(&mut self) {
        // RES 0,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(0, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x87(&mut self) {
        // RES 0,A
        self.registers.accumulator = self.bit_reset_bit(0, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x88(&mut self) {
        // RES 1,B
        self.registers.b = self.bit_reset_bit(1, self.registers.b);
    }

    pub(super) fn opcode_cb_0x89(&mut self) {
        // RES 1,C
        self.registers.c = self.bit_reset_bit(1, self.registers.c);
    }

    pub(super) fn opcode_cb_0x8a(&mut self) {
        // RES 1,D
        self.registers.d = self.bit_reset_bit(1, self.registers.d);
    }

    pub(super) fn opcode_cb_0x8b(&mut self) {
        // RES 1,E
        self.registers.e = self.bit_reset_bit(1, self.registers.e);
    }

    pub(super) fn opcode_cb_0x8c(&mut self) {
        // RES 1,H
        self.registers.h = self.bit_reset_bit(1, self.registers.h);
    }

    pub(super) fn opcode_cb_0x8d(&mut self) {
        // RES 1,L
        self.registers.l = self.bit_reset_bit(1, self.registers.l);
    }

    pub(super) fn opcode_cb_0x8e(&mut self) {
        // RES 1,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(1, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x8f(&mut self) {
        // RES 1,A
        self.registers.accumulator = self.bit_reset_bit(1, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x90(&mut self) {
        // RES 2,B
        self.registers.b = self.bit_reset_bit(2, self.registers.b);
    }

    pub(super) fn opcode_cb_0x91(&mut self) {
        // RES 2,C
        self.registers.c = self.bit_reset_bit(2, self.registers.c);
    }

    pub(super) fn opcode_cb_0x92(&mut self) {
        // RES 2,D
        self.registers.d = self.bit_reset_bit(2, self.registers.d);
    }

    pub(super) fn opcode_cb_0x93(&mut self) {
        // RES 2,E
        self.registers.e = self.bit_reset_bit(2, self.registers.e);
    }

    pub(super) fn opcode_cb_0x94(&mut self) {
        // RES 2,H
        self.registers.h = self.bit_reset_bit(2, self.registers.h);
    }

    pub(super) fn opcode_cb_0x95(&mut self) {
        // RES 2,L
        self.registers.l = self.bit_reset_bit(2, self.registers.l);
    }

    pub(super) fn opcode_cb_0x96(&mut self) {
        // RES 2,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(2, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x97(&mut self) {
        // RES 2,A
        self.registers.accumulator = self.bit_reset_bit(2, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0x98(&mut self) {
        // RES 3,B
        self.registers.b = self.bit_reset_bit(3, self.registers.b);
    }

    pub(super) fn opcode_cb_0x99(&mut self) {
        // RES 3,C
        self.registers.c = self.bit_reset_bit(3, self.registers.c);
    }

    pub(super) fn opcode_cb_0x9a(&mut self) {
        // RES 3,D
        self.registers.d = self.bit_reset_bit(3, self.registers.d);
    }

    pub(super) fn opcode_cb_0x9b(&mut self) {
        // RES 3,E
        self.registers.e = self.bit_reset_bit(3, self.registers.e);
    }

    pub(super) fn opcode_cb_0x9c(&mut self) {
        // RES 3,H
        self.registers.h = self.bit_reset_bit(3, self.registers.h);
    }

    pub(super) fn opcode_cb_0x9d(&mut self) {
        // RES 3,L
        self.registers.l = self.bit_reset_bit(3, self.registers.l);
    }

    pub(super) fn opcode_cb_0x9e(&mut self) {
        // RES 3,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(3, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0x9f(&mut self) {
        // RES 3,A
        self.registers.accumulator = self.bit_reset_bit(3, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xa0(&mut self) {
        // RES 4,B
        self.registers.b = self.bit_reset_bit(4, self.registers.b);
    }

    pub(super) fn opcode_cb_0xa1(&mut self) {
        // RES 4,C
        self.registers.c = self.bit_reset_bit(4, self.registers.c);
    }

    pub(super) fn opcode_cb_0xa2(&mut self) {
        // RES 4,D
        self.registers.d = self.bit_reset_bit(4, self.registers.d);
    }

    pub(super) fn opcode_cb_0xa3(&mut self) {
        // RES 4,E
        self.registers.e = self.bit_reset_bit(4, self.registers.e);
    }

    pub(super) fn opcode_cb_0xa4(&mut self) {
        // RES 4,H
        self.registers.h = self.bit_reset_bit(4, self.registers.h);
    }

    pub(super) fn opcode_cb_0xa5(&mut self) {
        // RES 4,L
        self.registers.l = self.bit_reset_bit(4, self.registers.l);
    }

    pub(super) fn opcode_cb_0xa6(&mut self) {
        // RES 4,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(4, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xa7(&mut self) {
        // RES 4,A
        self.registers.accumulator = self.bit_reset_bit(4, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xa8(&mut self) {
        // RES 5,B
        self.registers.b = self.bit_reset_bit(5, self.registers.b);
    }

    pub(super) fn opcode_cb_0xa9(&mut self) {
        // RES 5,C
        self.registers.c = self.bit_reset_bit(5, self.registers.c);
    }

    pub(super) fn opcode_cb_0xaa(&mut self) {
        // RES 5,D
        self.registers.d = self.bit_reset_bit(5, self.registers.d);
    }

    pub(super) fn opcode_cb_0xab(&mut self) {
        // RES 5,E
        self.registers.e = self.bit_reset_bit(5, self.registers.e);
    }

    pub(super) fn opcode_cb_0xac(&mut self) {
        // RES 5,H
        self.registers.h = self.bit_reset_bit(5, self.registers.h);
    }

    pub(super) fn opcode_cb_0xad(&mut self) {
        // RES 5,L
        self.registers.l = self.bit_reset_bit(5, self.registers.l);
    }

    pub(super) fn opcode_cb_0xae(&mut self) {
        // RES 5,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(5, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xaf(&mut self) {
        // RES 5,A
        self.registers.accumulator = self.bit_reset_bit(5, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xb0(&mut self) {
        // RES 6,B
        self.registers.b = self.bit_reset_bit(6, self.registers.b);
    }

    pub(super) fn opcode_cb_0xb1(&mut self) {
        // RES 6,C
        self.registers.c = self.bit_reset_bit(6, self.registers.c);
    }

    pub(super) fn opcode_cb_0xb2(&mut self) {
        // RES 6,D
        self.registers.d = self.bit_reset_bit(6, self.registers.d);
    }

    pub(super) fn opcode_cb_0xb3(&mut self) {
        // RES 6,E
        self.registers.e = self.bit_reset_bit(6, self.registers.e);
    }

    pub(super) fn opcode_cb_0xb4(&mut self) {
        // RES 6,H
        self.registers.h = self.bit_reset_bit(6, self.registers.h);
    }

    pub(super) fn opcode_cb_0xb5(&mut self) {
        // RES 6,L
        self.registers.l = self.bit_reset_bit(6, self.registers.l);
    }

    pub(super) fn opcode_cb_0xb6(&mut self) {
        // RES 6,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(6, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xb7(&mut self) {
        // RES 6,A
        self.registers.accumulator = self.bit_reset_bit(6, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xb8(&mut self) {
        // RES 7,B
        self.registers.b = self.bit_reset_bit(7, self.registers.b);
    }

    pub(super) fn opcode_cb_0xb9(&mut self) {
        // RES 7,C
        self.registers.c = self.bit_reset_bit(7, self.registers.c);
    }

    pub(super) fn opcode_cb_0xba(&mut self) {
        // RES 7,D
        self.registers.d = self.bit_reset_bit(7, self.registers.d);
    }

    pub(super) fn opcode_cb_0xbb(&mut self) {
        // RES 7,E
        self.registers.e = self.bit_reset_bit(7, self.registers.e);
    }

    pub(super) fn opcode_cb_0xbc(&mut self) {
        // RES 7,H
        self.registers.h = self.bit_reset_bit(7, self.registers.h);
    }

    pub(super) fn opcode_cb_0xbd(&mut self) {
        // RES 7,L
        self.registers.l = self.bit_reset_bit(7, self.registers.l);
    }

    pub(super) fn opcode_cb_0xbe(&mut self) {
        // RES 7,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_reset_bit(7, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xbf(&mut self) {
        // RES 7,A
        self.registers.accumulator = self.bit_reset_bit(7, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xc0(&mut self) {
        // SET 0,B
        self.registers.b = self.bit_set_bit(0, self.registers.b);
    }

    pub(super) fn opcode_cb_0xc1(&mut self) {
        // SET 0,C
        self.registers.c = self.bit_set_bit(0, self.registers.c);
    }

    pub(super) fn opcode_cb_0xc2(&mut self) {
        // SET 0,D
        self.registers.d = self.bit_set_bit(0, self.registers.d);
    }

    pub(super) fn opcode_cb_0xc3(&mut self) {
        // SET 0,E
        self.registers.e = self.bit_set_bit(0, self.registers.e);
    }

    pub(super) fn opcode_cb_0xc4(&mut self) {
        // SET 0,H
        self.registers.h = self.bit_set_bit(0, self.registers.h);
    }

    pub(super) fn opcode_cb_0xc5(&mut self) {
        // SET 0,L
        self.registers.l = self.bit_set_bit(0, self.registers.l);
    }

    pub(super) fn opcode_cb_0xc6(&mut self) {
        // SET 0,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(0, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xc7(&mut self) {
        // SET 0,A
        self.registers.accumulator = self.bit_set_bit(0, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xc8(&mut self) {
        // SET 1,B
        self.registers.b = self.bit_set_bit(1, self.registers.b);
    }

    pub(super) fn opcode_cb_0xc9(&mut self) {
        // SET 1,C
        self.registers.c = self.bit_set_bit(1, self.registers.c);
    }

    pub(super) fn opcode_cb_0xca(&mut self) {
        // SET 1,D
        self.registers.d = self.bit_set_bit(1, self.registers.d);
    }

    pub(super) fn opcode_cb_0xcb(&mut self) {
        // SET 1,E
        self.registers.e = self.bit_set_bit(1, self.registers.e);
    }

    pub(super) fn opcode_cb_0xcc(&mut self) {
        // SET 1,H
        self.registers.h = self.bit_set_bit(1, self.registers.h);
    }

    pub(super) fn opcode_cb_0xcd(&mut self) {
        // SET 1,L
        self.registers.l = self.bit_set_bit(1, self.registers.l);
    }

    pub(super) fn opcode_cb_0xce(&mut self) {
        // SET 1,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(1, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xcf(&mut self) {
        // SET 1,A
        self.registers.accumulator = self.bit_set_bit(1, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xd0(&mut self) {
        // SET 2,B
        self.registers.b = self.bit_set_bit(2, self.registers.b);
    }

    pub(super) fn opcode_cb_0xd1(&mut self) {
        // SET 2,C
        self.registers.c = self.bit_set_bit(2, self.registers.c);
    }

    pub(super) fn opcode_cb_0xd2(&mut self) {
        // SET 2,D
        self.registers.d = self.bit_set_bit(2, self.registers.d);
    }

    pub(super) fn opcode_cb_0xd3(&mut self) {
        // SET 2,E
        self.registers.e = self.bit_set_bit(2, self.registers.e);
    }

    pub(super) fn opcode_cb_0xd4(&mut self) {
        // SET 2,H
        self.registers.h = self.bit_set_bit(2, self.registers.h);
    }

    pub(super) fn opcode_cb_0xd5(&mut self) {
        // SET 2,L
        self.registers.l = self.bit_set_bit(2, self.registers.l);
    }

    pub(super) fn opcode_cb_0xd6(&mut self) {
        // SET 2,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(2, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xd7(&mut self) {
        // SET 2,A
        self.registers.accumulator = self.bit_set_bit(2, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xd8(&mut self) {
        // SET 3,B
        self.registers.b = self.bit_set_bit(3, self.registers.b);
    }

    pub(super) fn opcode_cb_0xd9(&mut self) {
        // SET 3,C
        self.registers.c = self.bit_set_bit(3, self.registers.c);
    }

    pub(super) fn opcode_cb_0xda(&mut self) {
        // SET 3,D
        self.registers.d = self.bit_set_bit(3, self.registers.d);
    }

    pub(super) fn opcode_cb_0xdb(&mut self) {
        // SET 3,E
        self.registers.e = self.bit_set_bit(3, self.registers.e);
    }

    pub(super) fn opcode_cb_0xdc(&mut self) {
        // SET 3,H
        self.registers.h = self.bit_set_bit(3, self.registers.h);
    }

    pub(super) fn opcode_cb_0xdd(&mut self) {
        // SET 3,L
        self.registers.l = self.bit_set_bit(3, self.registers.l);
    }

    pub(super) fn opcode_cb_0xde(&mut self) {
        // SET 3,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(3, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xdf(&mut self) {
        // SET 3,A
        self.registers.accumulator = self.bit_set_bit(3, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xe0(&mut self) {
        // SET 4,B
        self.registers.b = self.bit_set_bit(4, self.registers.b);
    }

    pub(super) fn opcode_cb_0xe1(&mut self) {
        // SET 4,C
        self.registers.c = self.bit_set_bit(4, self.registers.c);
    }

    pub(super) fn opcode_cb_0xe2(&mut self) {
        // SET 4,D
        self.registers.d = self.bit_set_bit(4, self.registers.d);
    }

    pub(super) fn opcode_cb_0xe3(&mut self) {
        // SET 4,E
        self.registers.e = self.bit_set_bit(4, self.registers.e);
    }

    pub(super) fn opcode_cb_0xe4(&mut self) {
        // SET 4,H
        self.registers.h = self.bit_set_bit(4, self.registers.h);
    }

    pub(super) fn opcode_cb_0xe5(&mut self) {
        // SET 4,L
        self.registers.l = self.bit_set_bit(4, self.registers.l);
    }

    pub(super) fn opcode_cb_0xe6(&mut self) {
        // SET 4,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(4, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xe7(&mut self) {
        // SET 4,A
        self.registers.accumulator = self.bit_set_bit(4, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xe8(&mut self) {
        // SET 5,B
        self.registers.b = self.bit_set_bit(5, self.registers.b);
    }

    pub(super) fn opcode_cb_0xe9(&mut self) {
        // SET 5,C
        self.registers.c = self.bit_set_bit(5, self.registers.c);
    }

    pub(super) fn opcode_cb_0xea(&mut self) {
        // SET 5,D
        self.registers.d = self.bit_set_bit(5, self.registers.d);
    }

    pub(super) fn opcode_cb_0xeb(&mut self) {
        // SET 5,E
        self.registers.e = self.bit_set_bit(5, self.registers.e);
    }

    pub(super) fn opcode_cb_0xec(&mut self) {
        // SET 5,H
        self.registers.h = self.bit_set_bit(5, self.registers.h);
    }

    pub(super) fn opcode_cb_0xed(&mut self) {
        // SET 5,L
        self.registers.l = self.bit_set_bit(5, self.registers.l);
    }

    pub(super) fn opcode_cb_0xee(&mut self) {
        // SET 5,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(5, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xef(&mut self) {
        // SET 5,A
        self.registers.accumulator = self.bit_set_bit(5, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xf0(&mut self) {
        // SET 6,B
        self.registers.b = self.bit_set_bit(6, self.registers.b);
    }

    pub(super) fn opcode_cb_0xf1(&mut self) {
        // SET 6,C
        self.registers.c = self.bit_set_bit(6, self.registers.c);
    }

    pub(super) fn opcode_cb_0xf2(&mut self) {
        // SET 6,D
        self.registers.d = self.bit_set_bit(6, self.registers.d);
    }

    pub(super) fn opcode_cb_0xf3(&mut self) {
        // SET 6,E
        self.registers.e = self.bit_set_bit(6, self.registers.e);
    }

    pub(super) fn opcode_cb_0xf4(&mut self) {
        // SET 6,H
        self.registers.h = self.bit_set_bit(6, self.registers.h);
    }

    pub(super) fn opcode_cb_0xf5(&mut self) {
        // SET 6,L
        self.registers.l = self.bit_set_bit(6, self.registers.l);
    }

    pub(super) fn opcode_cb_0xf6(&mut self) {
        // SET 6,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(6, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xf7(&mut self) {
        // SET 6,A
        self.registers.accumulator = self.bit_set_bit(6, self.registers.accumulator);
    }

    pub(super) fn opcode_cb_0xf8(&mut self) {
        // SET 7,B
        self.registers.b = self.bit_set_bit(7, self.registers.b);
    }

    pub(super) fn opcode_cb_0xf9(&mut self) {
        // SET 7,C
        self.registers.c = self.bit_set_bit(7, self.registers.c);
    }

    pub(super) fn opcode_cb_0xfa(&mut self) {
        // SET 7,D
        self.registers.d = self.bit_set_bit(7, self.registers.d);
    }

    pub(super) fn opcode_cb_0xfb(&mut self) {
        // SET 7,E
        self.registers.e = self.bit_set_bit(7, self.registers.e);
    }

    pub(super) fn opcode_cb_0xfc(&mut self) {
        // SET 7,H
        self.registers.h = self.bit_set_bit(7, self.registers.h);
    }

    pub(super) fn opcode_cb_0xfd(&mut self) {
        // SET 7,L
        self.registers.l = self.bit_set_bit(7, self.registers.l);
    }

    pub(super) fn opcode_cb_0xfe(&mut self) {
        // SET 7,(HL)
        let address = self.registers.get_hl();
        let value = self.read_byte(address);

        let result = self.bit_set_bit(7, value);

        self.write_byte(address, result);
    }

    pub(super) fn opcode_cb_0xff(&mut self) {
        // SET 7,A
        self.registers.accumulator = self.bit_set_bit(7, self.registers.accumulator);
    }
}
