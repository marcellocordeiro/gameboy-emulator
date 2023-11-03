use crate::{
    cpu::{alu, Cpu},
    memory::Memory,
};

// Completed, will definitely need some refactoring.

macro_rules! alu_op_r8 {
    ($self:ident, $memory:ident, $F:ident, [hl]) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte($memory, address);

        let result = alu::$F(&mut $self.registers.f, value);

        $self.write_byte($memory, address, result)
    };

    ($self:ident, $memory:ident, $F:ident, $bit:literal, [hl]) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte($memory, address);

        let result = alu::$F(&mut $self.registers.f, $bit, value);

        $self.write_byte($memory, address, result)
    };

    ($self:ident, $F:ident, $reg:ident) => {
        $self.registers.$reg = alu::$F(&mut $self.registers.f, $self.registers.$reg)
    };

    ($self:ident, $F:ident, $bit:literal, $reg:ident) => {
        $self.registers.$reg = alu::$F(&mut $self.registers.f, $bit, $self.registers.$reg)
    };
}

macro_rules! alu_op_bit_test {
    ($self:ident, $memory:ident, $F:ident, $bit:literal, [hl]) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte($memory, address);

        alu::$F(&mut $self.registers.f, $bit, value)
    };

    ($self:ident, $F:ident, $bit:literal, $reg:ident) => {
        alu::$F(&mut $self.registers.f, $bit, $self.registers.$reg)
    };
}

impl Cpu {
    /// RLC B
    pub(super) fn opcode_cb_0x00(&mut self) {
        alu_op_r8!(self, rlc, b);
    }

    /// RLC C
    pub(super) fn opcode_cb_0x01(&mut self) {
        alu_op_r8!(self, rlc, c);
    }

    /// RLC D
    pub(super) fn opcode_cb_0x02(&mut self) {
        alu_op_r8!(self, rlc, d);
    }

    /// RLC E
    pub(super) fn opcode_cb_0x03(&mut self) {
        alu_op_r8!(self, rlc, e);
    }

    /// RLC H
    pub(super) fn opcode_cb_0x04(&mut self) {
        alu_op_r8!(self, rlc, h);
    }

    /// RLC L
    pub(super) fn opcode_cb_0x05(&mut self) {
        alu_op_r8!(self, rlc, l);
    }

    /// RLC (HL)
    pub(super) fn opcode_cb_0x06(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, rlc, [hl]);
    }

    /// RLC A
    pub(super) fn opcode_cb_0x07(&mut self) {
        alu_op_r8!(self, rlc, a);
    }

    /// RRC B
    pub(super) fn opcode_cb_0x08(&mut self) {
        alu_op_r8!(self, rrc, b);
    }

    /// RRC C
    pub(super) fn opcode_cb_0x09(&mut self) {
        alu_op_r8!(self, rrc, c);
    }

    /// RRC D
    pub(super) fn opcode_cb_0x0a(&mut self) {
        alu_op_r8!(self, rrc, d);
    }

    /// RRC E
    pub(super) fn opcode_cb_0x0b(&mut self) {
        alu_op_r8!(self, rrc, e);
    }

    /// RRC H
    pub(super) fn opcode_cb_0x0c(&mut self) {
        alu_op_r8!(self, rrc, h);
    }

    /// RRC L
    pub(super) fn opcode_cb_0x0d(&mut self) {
        alu_op_r8!(self, rrc, l);
    }

    /// RRC (HL)
    pub(super) fn opcode_cb_0x0e(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, rrc, [hl]);
    }

    /// RRC A
    pub(super) fn opcode_cb_0x0f(&mut self) {
        alu_op_r8!(self, rrc, a);
    }

    /// RL B
    pub(super) fn opcode_cb_0x10(&mut self) {
        alu_op_r8!(self, rl, b);
    }

    /// RL C
    pub(super) fn opcode_cb_0x11(&mut self) {
        alu_op_r8!(self, rl, c);
    }

    /// RL D
    pub(super) fn opcode_cb_0x12(&mut self) {
        alu_op_r8!(self, rl, d);
    }

    /// RL E
    pub(super) fn opcode_cb_0x13(&mut self) {
        alu_op_r8!(self, rl, e);
    }

    /// RL H
    pub(super) fn opcode_cb_0x14(&mut self) {
        alu_op_r8!(self, rl, h);
    }

    /// RL L
    pub(super) fn opcode_cb_0x15(&mut self) {
        alu_op_r8!(self, rl, l);
    }

    /// RL (HL)
    pub(super) fn opcode_cb_0x16(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, rl, [hl]);
    }

    /// RL A
    pub(super) fn opcode_cb_0x17(&mut self) {
        alu_op_r8!(self, rl, a);
    }

    /// RR B
    pub(super) fn opcode_cb_0x18(&mut self) {
        alu_op_r8!(self, rr, b);
    }

    /// RR C
    pub(super) fn opcode_cb_0x19(&mut self) {
        alu_op_r8!(self, rr, c);
    }

    /// RR D
    pub(super) fn opcode_cb_0x1a(&mut self) {
        alu_op_r8!(self, rr, d);
    }

    /// RR E
    pub(super) fn opcode_cb_0x1b(&mut self) {
        alu_op_r8!(self, rr, e);
    }

    /// RR H
    pub(super) fn opcode_cb_0x1c(&mut self) {
        alu_op_r8!(self, rr, h);
    }

    /// RR L
    pub(super) fn opcode_cb_0x1d(&mut self) {
        alu_op_r8!(self, rr, l);
    }

    /// RR (HL)
    pub(super) fn opcode_cb_0x1e(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, rr, [hl]);
    }

    /// RR A
    pub(super) fn opcode_cb_0x1f(&mut self) {
        alu_op_r8!(self, rr, a);
    }

    /// SLA B
    pub(super) fn opcode_cb_0x20(&mut self) {
        alu_op_r8!(self, sla, b);
    }

    /// SLA C
    pub(super) fn opcode_cb_0x21(&mut self) {
        alu_op_r8!(self, sla, c);
    }

    /// SLA D
    pub(super) fn opcode_cb_0x22(&mut self) {
        alu_op_r8!(self, sla, d);
    }

    /// SLA E
    pub(super) fn opcode_cb_0x23(&mut self) {
        alu_op_r8!(self, sla, e);
    }

    /// SLA H
    pub(super) fn opcode_cb_0x24(&mut self) {
        alu_op_r8!(self, sla, h);
    }

    /// SLA L
    pub(super) fn opcode_cb_0x25(&mut self) {
        alu_op_r8!(self, sla, l);
    }

    /// SLA (HL)
    pub(super) fn opcode_cb_0x26(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, sla, [hl]);
    }

    /// SLA A
    pub(super) fn opcode_cb_0x27(&mut self) {
        alu_op_r8!(self, sla, a);
    }

    /// SRA B
    pub(super) fn opcode_cb_0x28(&mut self) {
        alu_op_r8!(self, sra, b);
    }

    /// SRA C
    pub(super) fn opcode_cb_0x29(&mut self) {
        alu_op_r8!(self, sra, c);
    }

    /// SRA D
    pub(super) fn opcode_cb_0x2a(&mut self) {
        alu_op_r8!(self, sra, d);
    }

    /// SRA E
    pub(super) fn opcode_cb_0x2b(&mut self) {
        alu_op_r8!(self, sra, e);
    }

    /// SRA H
    pub(super) fn opcode_cb_0x2c(&mut self) {
        alu_op_r8!(self, sra, h);
    }

    /// SRA L
    pub(super) fn opcode_cb_0x2d(&mut self) {
        alu_op_r8!(self, sra, l);
    }

    /// SRA (HL)
    pub(super) fn opcode_cb_0x2e(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, sra, [hl]);
    }

    /// SRA A
    pub(super) fn opcode_cb_0x2f(&mut self) {
        alu_op_r8!(self, sra, a);
    }

    /// SWAP B
    pub(super) fn opcode_cb_0x30(&mut self) {
        alu_op_r8!(self, swap, b);
    }

    /// SWAP C
    pub(super) fn opcode_cb_0x31(&mut self) {
        alu_op_r8!(self, swap, c);
    }

    /// SWAP D
    pub(super) fn opcode_cb_0x32(&mut self) {
        alu_op_r8!(self, swap, d);
    }

    /// SWAP E
    pub(super) fn opcode_cb_0x33(&mut self) {
        alu_op_r8!(self, swap, e);
    }

    /// SWAP H
    pub(super) fn opcode_cb_0x34(&mut self) {
        alu_op_r8!(self, swap, h);
    }

    /// SWAP L
    pub(super) fn opcode_cb_0x35(&mut self) {
        alu_op_r8!(self, swap, l);
    }

    /// SWAP (HL)
    pub(super) fn opcode_cb_0x36(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, swap, [hl]);
    }

    /// SWAP A
    pub(super) fn opcode_cb_0x37(&mut self) {
        alu_op_r8!(self, swap, a);
    }

    /// SRL B
    pub(super) fn opcode_cb_0x38(&mut self) {
        alu_op_r8!(self, srl, b);
    }

    /// SRL C
    pub(super) fn opcode_cb_0x39(&mut self) {
        alu_op_r8!(self, srl, c);
    }

    /// SRL D
    pub(super) fn opcode_cb_0x3a(&mut self) {
        alu_op_r8!(self, srl, d);
    }

    /// SRL E
    pub(super) fn opcode_cb_0x3b(&mut self) {
        alu_op_r8!(self, srl, e);
    }

    /// SRL H
    pub(super) fn opcode_cb_0x3c(&mut self) {
        alu_op_r8!(self, srl, h);
    }

    /// SRL L
    pub(super) fn opcode_cb_0x3d(&mut self) {
        alu_op_r8!(self, srl, l);
    }

    /// SRL (HL)
    pub(super) fn opcode_cb_0x3e(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, srl, [hl]);
    }

    /// SRL A
    pub(super) fn opcode_cb_0x3f(&mut self) {
        alu_op_r8!(self, srl, a);
    }

    /// BIT 0,B
    pub(super) fn opcode_cb_0x40(&mut self) {
        alu_op_bit_test!(self, bit, 0, b);
    }

    /// BIT 0,C
    pub(super) fn opcode_cb_0x41(&mut self) {
        alu_op_bit_test!(self, bit, 0, c);
    }

    /// BIT 0,D
    pub(super) fn opcode_cb_0x42(&mut self) {
        alu_op_bit_test!(self, bit, 0, d);
    }

    /// BIT 0,E
    pub(super) fn opcode_cb_0x43(&mut self) {
        alu_op_bit_test!(self, bit, 0, e);
    }

    /// BIT 0,H
    pub(super) fn opcode_cb_0x44(&mut self) {
        alu_op_bit_test!(self, bit, 0, h);
    }

    /// BIT 0,L
    pub(super) fn opcode_cb_0x45(&mut self) {
        alu_op_bit_test!(self, bit, 0, l);
    }

    /// BIT 0,(HL)
    pub(super) fn opcode_cb_0x46(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 0, [hl]);
    }

    /// BIT 0,A
    pub(super) fn opcode_cb_0x47(&mut self) {
        alu_op_bit_test!(self, bit, 0, a);
    }

    /// BIT 1,B
    pub(super) fn opcode_cb_0x48(&mut self) {
        alu_op_bit_test!(self, bit, 1, b);
    }

    /// BIT 1,C
    pub(super) fn opcode_cb_0x49(&mut self) {
        alu_op_bit_test!(self, bit, 1, c);
    }

    /// BIT 1,D
    pub(super) fn opcode_cb_0x4a(&mut self) {
        alu_op_bit_test!(self, bit, 1, d);
    }

    /// BIT 1,E
    pub(super) fn opcode_cb_0x4b(&mut self) {
        alu_op_bit_test!(self, bit, 1, e);
    }

    /// BIT 1,H
    pub(super) fn opcode_cb_0x4c(&mut self) {
        alu_op_bit_test!(self, bit, 1, h);
    }

    /// BIT 1,L
    pub(super) fn opcode_cb_0x4d(&mut self) {
        alu_op_bit_test!(self, bit, 1, l);
    }

    /// BIT 1,(HL)
    pub(super) fn opcode_cb_0x4e(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 1, [hl]);
    }

    /// BIT 1,A
    pub(super) fn opcode_cb_0x4f(&mut self) {
        alu_op_bit_test!(self, bit, 1, a);
    }

    /// BIT 2,B
    pub(super) fn opcode_cb_0x50(&mut self) {
        alu_op_bit_test!(self, bit, 2, b);
    }

    /// BIT 2,C
    pub(super) fn opcode_cb_0x51(&mut self) {
        alu_op_bit_test!(self, bit, 2, c);
    }

    /// BIT 2,D
    pub(super) fn opcode_cb_0x52(&mut self) {
        alu_op_bit_test!(self, bit, 2, d);
    }

    /// BIT 2,E
    pub(super) fn opcode_cb_0x53(&mut self) {
        alu_op_bit_test!(self, bit, 2, e);
    }

    /// BIT 2,H
    pub(super) fn opcode_cb_0x54(&mut self) {
        alu_op_bit_test!(self, bit, 2, h);
    }

    /// BIT 2,L
    pub(super) fn opcode_cb_0x55(&mut self) {
        alu_op_bit_test!(self, bit, 2, l);
    }

    /// BIT 2,(HL)
    pub(super) fn opcode_cb_0x56(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 2, [hl]);
    }

    /// BIT 2,A
    pub(super) fn opcode_cb_0x57(&mut self) {
        alu_op_bit_test!(self, bit, 2, a);
    }

    /// BIT 3,B
    pub(super) fn opcode_cb_0x58(&mut self) {
        alu_op_bit_test!(self, bit, 3, b);
    }

    /// BIT 3,C
    pub(super) fn opcode_cb_0x59(&mut self) {
        alu_op_bit_test!(self, bit, 3, c);
    }

    /// BIT 3,D
    pub(super) fn opcode_cb_0x5a(&mut self) {
        alu_op_bit_test!(self, bit, 3, d);
    }

    /// BIT 3,E
    pub(super) fn opcode_cb_0x5b(&mut self) {
        alu_op_bit_test!(self, bit, 3, e);
    }

    /// BIT 3,H
    pub(super) fn opcode_cb_0x5c(&mut self) {
        alu_op_bit_test!(self, bit, 3, h);
    }

    /// BIT 3,L
    pub(super) fn opcode_cb_0x5d(&mut self) {
        alu_op_bit_test!(self, bit, 3, l);
    }

    /// BIT 3,(HL)
    pub(super) fn opcode_cb_0x5e(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 3, [hl]);
    }

    /// BIT 3,A
    pub(super) fn opcode_cb_0x5f(&mut self) {
        alu_op_bit_test!(self, bit, 3, a);
    }

    /// BIT 4,B
    pub(super) fn opcode_cb_0x60(&mut self) {
        alu_op_bit_test!(self, bit, 4, b);
    }

    /// BIT 4,C
    pub(super) fn opcode_cb_0x61(&mut self) {
        alu_op_bit_test!(self, bit, 4, c);
    }

    /// BIT 4,D
    pub(super) fn opcode_cb_0x62(&mut self) {
        alu_op_bit_test!(self, bit, 4, d);
    }

    /// BIT 4,E
    pub(super) fn opcode_cb_0x63(&mut self) {
        alu_op_bit_test!(self, bit, 4, e);
    }

    /// BIT 4,H
    pub(super) fn opcode_cb_0x64(&mut self) {
        alu_op_bit_test!(self, bit, 4, h);
    }

    /// BIT 4,L
    pub(super) fn opcode_cb_0x65(&mut self) {
        alu_op_bit_test!(self, bit, 4, l);
    }

    /// BIT 4,(HL)
    pub(super) fn opcode_cb_0x66(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 4, [hl]);
    }

    /// BIT 4,A
    pub(super) fn opcode_cb_0x67(&mut self) {
        alu_op_bit_test!(self, bit, 4, a);
    }

    /// BIT 5,B
    pub(super) fn opcode_cb_0x68(&mut self) {
        alu_op_bit_test!(self, bit, 5, b);
    }

    /// BIT 5,C
    pub(super) fn opcode_cb_0x69(&mut self) {
        alu_op_bit_test!(self, bit, 5, c);
    }

    /// BIT 5,D
    pub(super) fn opcode_cb_0x6a(&mut self) {
        alu_op_bit_test!(self, bit, 5, d);
    }

    /// BIT 5,E
    pub(super) fn opcode_cb_0x6b(&mut self) {
        alu_op_bit_test!(self, bit, 5, e);
    }

    /// BIT 5,H
    pub(super) fn opcode_cb_0x6c(&mut self) {
        alu_op_bit_test!(self, bit, 5, h);
    }

    /// BIT 5,L
    pub(super) fn opcode_cb_0x6d(&mut self) {
        alu_op_bit_test!(self, bit, 5, l);
    }

    /// BIT 5,(HL)
    pub(super) fn opcode_cb_0x6e(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 5, [hl]);
    }

    /// BIT 5,A
    pub(super) fn opcode_cb_0x6f(&mut self) {
        alu_op_bit_test!(self, bit, 5, a);
    }

    /// BIT 6,B
    pub(super) fn opcode_cb_0x70(&mut self) {
        alu_op_bit_test!(self, bit, 6, b);
    }

    /// BIT 6,C
    pub(super) fn opcode_cb_0x71(&mut self) {
        alu_op_bit_test!(self, bit, 6, c);
    }

    /// BIT 6,D
    pub(super) fn opcode_cb_0x72(&mut self) {
        alu_op_bit_test!(self, bit, 6, d);
    }

    /// BIT 6,E
    pub(super) fn opcode_cb_0x73(&mut self) {
        alu_op_bit_test!(self, bit, 6, e);
    }

    /// BIT 6,H
    pub(super) fn opcode_cb_0x74(&mut self) {
        alu_op_bit_test!(self, bit, 6, h);
    }

    /// BIT 6,L
    pub(super) fn opcode_cb_0x75(&mut self) {
        alu_op_bit_test!(self, bit, 6, l);
    }

    /// BIT 6,(HL)
    pub(super) fn opcode_cb_0x76(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 6, [hl]);
    }

    /// BIT 6,A
    pub(super) fn opcode_cb_0x77(&mut self) {
        alu_op_bit_test!(self, bit, 6, a);
    }

    /// BIT 7,B
    pub(super) fn opcode_cb_0x78(&mut self) {
        alu_op_bit_test!(self, bit, 7, b);
    }

    /// BIT 7,C
    pub(super) fn opcode_cb_0x79(&mut self) {
        alu_op_bit_test!(self, bit, 7, c);
    }

    /// BIT 7,D
    pub(super) fn opcode_cb_0x7a(&mut self) {
        alu_op_bit_test!(self, bit, 7, d);
    }

    /// BIT 7,E
    pub(super) fn opcode_cb_0x7b(&mut self) {
        alu_op_bit_test!(self, bit, 7, e);
    }

    /// BIT 7,H
    pub(super) fn opcode_cb_0x7c(&mut self) {
        alu_op_bit_test!(self, bit, 7, h);
    }

    /// BIT 7,L
    pub(super) fn opcode_cb_0x7d(&mut self) {
        alu_op_bit_test!(self, bit, 7, l);
    }

    /// BIT 7,(HL)
    pub(super) fn opcode_cb_0x7e(&mut self, memory: &mut Memory) {
        alu_op_bit_test!(self, memory, bit, 7, [hl]);
    }

    /// BIT 7,A
    pub(super) fn opcode_cb_0x7f(&mut self) {
        alu_op_bit_test!(self, bit, 7, a);
    }

    /// RES 0,B
    pub(super) fn opcode_cb_0x80(&mut self) {
        alu_op_r8!(self, res, 0, b);
    }

    /// RES 0,C
    pub(super) fn opcode_cb_0x81(&mut self) {
        alu_op_r8!(self, res, 0, c);
    }

    /// RES 0,D
    pub(super) fn opcode_cb_0x82(&mut self) {
        alu_op_r8!(self, res, 0, d);
    }

    /// RES 0,E
    pub(super) fn opcode_cb_0x83(&mut self) {
        alu_op_r8!(self, res, 0, e);
    }

    /// RES 0,H
    pub(super) fn opcode_cb_0x84(&mut self) {
        alu_op_r8!(self, res, 0, h);
    }

    /// RES 0,L
    pub(super) fn opcode_cb_0x85(&mut self) {
        alu_op_r8!(self, res, 0, l);
    }

    /// RES 0,(HL)
    pub(super) fn opcode_cb_0x86(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 0, [hl]);
    }

    /// RES 0,A
    pub(super) fn opcode_cb_0x87(&mut self) {
        alu_op_r8!(self, res, 0, a);
    }

    /// RES 1,B
    pub(super) fn opcode_cb_0x88(&mut self) {
        alu_op_r8!(self, res, 1, b);
    }

    /// RES 1,C
    pub(super) fn opcode_cb_0x89(&mut self) {
        alu_op_r8!(self, res, 1, c);
    }

    /// RES 1,D
    pub(super) fn opcode_cb_0x8a(&mut self) {
        alu_op_r8!(self, res, 1, d);
    }

    /// RES 1,E
    pub(super) fn opcode_cb_0x8b(&mut self) {
        alu_op_r8!(self, res, 1, e);
    }

    /// RES 1,H
    pub(super) fn opcode_cb_0x8c(&mut self) {
        alu_op_r8!(self, res, 1, h);
    }

    /// RES 1,L
    pub(super) fn opcode_cb_0x8d(&mut self) {
        alu_op_r8!(self, res, 1, l);
    }

    /// RES 1,(HL)
    pub(super) fn opcode_cb_0x8e(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 1, [hl]);
    }

    /// RES 1,A
    pub(super) fn opcode_cb_0x8f(&mut self) {
        alu_op_r8!(self, res, 1, a);
    }

    /// RES 2,B
    pub(super) fn opcode_cb_0x90(&mut self) {
        alu_op_r8!(self, res, 2, b);
    }

    /// RES 2,C
    pub(super) fn opcode_cb_0x91(&mut self) {
        alu_op_r8!(self, res, 2, c);
    }

    /// RES 2,D
    pub(super) fn opcode_cb_0x92(&mut self) {
        alu_op_r8!(self, res, 2, d);
    }

    /// RES 2,E
    pub(super) fn opcode_cb_0x93(&mut self) {
        alu_op_r8!(self, res, 2, e);
    }

    /// RES 2,H
    pub(super) fn opcode_cb_0x94(&mut self) {
        alu_op_r8!(self, res, 2, h);
    }

    /// RES 2,L
    pub(super) fn opcode_cb_0x95(&mut self) {
        alu_op_r8!(self, res, 2, l);
    }

    /// RES 2,(HL)
    pub(super) fn opcode_cb_0x96(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 2, [hl]);
    }

    /// RES 2,A
    pub(super) fn opcode_cb_0x97(&mut self) {
        alu_op_r8!(self, res, 2, a);
    }

    /// RES 3,B
    pub(super) fn opcode_cb_0x98(&mut self) {
        alu_op_r8!(self, res, 3, b);
    }

    /// RES 3,C
    pub(super) fn opcode_cb_0x99(&mut self) {
        alu_op_r8!(self, res, 3, c);
    }

    /// RES 3,D
    pub(super) fn opcode_cb_0x9a(&mut self) {
        alu_op_r8!(self, res, 3, d);
    }

    /// RES 3,E
    pub(super) fn opcode_cb_0x9b(&mut self) {
        alu_op_r8!(self, res, 3, e);
    }

    /// RES 3,H
    pub(super) fn opcode_cb_0x9c(&mut self) {
        alu_op_r8!(self, res, 3, h);
    }

    /// RES 3,L
    pub(super) fn opcode_cb_0x9d(&mut self) {
        alu_op_r8!(self, res, 3, l);
    }

    /// RES 3,(HL)
    pub(super) fn opcode_cb_0x9e(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 3, [hl]);
    }

    /// RES 3,A
    pub(super) fn opcode_cb_0x9f(&mut self) {
        alu_op_r8!(self, res, 3, a);
    }

    /// RES 4,B
    pub(super) fn opcode_cb_0xa0(&mut self) {
        alu_op_r8!(self, res, 4, b);
    }

    /// RES 4,C
    pub(super) fn opcode_cb_0xa1(&mut self) {
        alu_op_r8!(self, res, 4, c);
    }

    /// RES 4,D
    pub(super) fn opcode_cb_0xa2(&mut self) {
        alu_op_r8!(self, res, 4, d);
    }

    /// RES 4,E
    pub(super) fn opcode_cb_0xa3(&mut self) {
        alu_op_r8!(self, res, 4, e);
    }

    /// RES 4,H
    pub(super) fn opcode_cb_0xa4(&mut self) {
        alu_op_r8!(self, res, 4, h);
    }

    /// RES 4,L
    pub(super) fn opcode_cb_0xa5(&mut self) {
        alu_op_r8!(self, res, 4, l);
    }

    /// RES 4,(HL)
    pub(super) fn opcode_cb_0xa6(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 4, [hl]);
    }

    /// RES 4,A
    pub(super) fn opcode_cb_0xa7(&mut self) {
        alu_op_r8!(self, res, 4, a);
    }

    /// RES 5,B
    pub(super) fn opcode_cb_0xa8(&mut self) {
        alu_op_r8!(self, res, 5, b);
    }

    /// RES 5,C
    pub(super) fn opcode_cb_0xa9(&mut self) {
        alu_op_r8!(self, res, 5, c);
    }

    /// RES 5,D
    pub(super) fn opcode_cb_0xaa(&mut self) {
        alu_op_r8!(self, res, 5, d);
    }

    /// RES 5,E
    pub(super) fn opcode_cb_0xab(&mut self) {
        alu_op_r8!(self, res, 5, e);
    }

    /// RES 5,H
    pub(super) fn opcode_cb_0xac(&mut self) {
        alu_op_r8!(self, res, 5, h);
    }

    /// RES 5,L
    pub(super) fn opcode_cb_0xad(&mut self) {
        alu_op_r8!(self, res, 5, l);
    }

    /// RES 5,(HL)
    pub(super) fn opcode_cb_0xae(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 5, [hl]);
    }

    /// RES 5,A
    pub(super) fn opcode_cb_0xaf(&mut self) {
        alu_op_r8!(self, res, 5, a);
    }

    /// RES 6,B
    pub(super) fn opcode_cb_0xb0(&mut self) {
        alu_op_r8!(self, res, 6, b);
    }

    /// RES 6,C
    pub(super) fn opcode_cb_0xb1(&mut self) {
        alu_op_r8!(self, res, 6, c);
    }

    /// RES 6,D
    pub(super) fn opcode_cb_0xb2(&mut self) {
        alu_op_r8!(self, res, 6, d);
    }

    /// RES 6,E
    pub(super) fn opcode_cb_0xb3(&mut self) {
        alu_op_r8!(self, res, 6, e);
    }

    /// RES 6,H
    pub(super) fn opcode_cb_0xb4(&mut self) {
        alu_op_r8!(self, res, 6, h);
    }

    /// RES 6,L
    pub(super) fn opcode_cb_0xb5(&mut self) {
        alu_op_r8!(self, res, 6, l);
    }

    /// RES 6,(HL)
    pub(super) fn opcode_cb_0xb6(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 6, [hl]);
    }

    /// RES 6,A
    pub(super) fn opcode_cb_0xb7(&mut self) {
        alu_op_r8!(self, res, 6, a);
    }

    /// RES 7,B
    pub(super) fn opcode_cb_0xb8(&mut self) {
        alu_op_r8!(self, res, 7, b);
    }

    /// RES 7,C
    pub(super) fn opcode_cb_0xb9(&mut self) {
        alu_op_r8!(self, res, 7, c);
    }

    /// RES 7,D
    pub(super) fn opcode_cb_0xba(&mut self) {
        alu_op_r8!(self, res, 7, d);
    }

    /// RES 7,E
    pub(super) fn opcode_cb_0xbb(&mut self) {
        alu_op_r8!(self, res, 7, e);
    }

    /// RES 7,H
    pub(super) fn opcode_cb_0xbc(&mut self) {
        alu_op_r8!(self, res, 7, h);
    }

    /// RES 7,L
    pub(super) fn opcode_cb_0xbd(&mut self) {
        alu_op_r8!(self, res, 7, l);
    }

    /// RES 7,(HL)
    pub(super) fn opcode_cb_0xbe(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, res, 7, [hl]);
    }

    /// RES 7,A
    pub(super) fn opcode_cb_0xbf(&mut self) {
        alu_op_r8!(self, res, 7, a);
    }

    /// SET 0,B
    pub(super) fn opcode_cb_0xc0(&mut self) {
        alu_op_r8!(self, set, 0, b);
    }

    /// SET 0,C
    pub(super) fn opcode_cb_0xc1(&mut self) {
        alu_op_r8!(self, set, 0, c);
    }

    /// SET 0,D
    pub(super) fn opcode_cb_0xc2(&mut self) {
        alu_op_r8!(self, set, 0, d);
    }

    /// SET 0,E
    pub(super) fn opcode_cb_0xc3(&mut self) {
        alu_op_r8!(self, set, 0, e);
    }

    /// SET 0,H
    pub(super) fn opcode_cb_0xc4(&mut self) {
        alu_op_r8!(self, set, 0, h);
    }

    /// SET 0,L
    pub(super) fn opcode_cb_0xc5(&mut self) {
        alu_op_r8!(self, set, 0, l);
    }

    /// SET 0,(HL)
    pub(super) fn opcode_cb_0xc6(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 0, [hl]);
    }

    /// SET 0,A
    pub(super) fn opcode_cb_0xc7(&mut self) {
        alu_op_r8!(self, set, 0, a);
    }

    /// SET 1,B
    pub(super) fn opcode_cb_0xc8(&mut self) {
        alu_op_r8!(self, set, 1, b);
    }

    /// SET 1,C
    pub(super) fn opcode_cb_0xc9(&mut self) {
        alu_op_r8!(self, set, 1, c);
    }

    /// SET 1,D
    pub(super) fn opcode_cb_0xca(&mut self) {
        alu_op_r8!(self, set, 1, d);
    }

    /// SET 1,E
    pub(super) fn opcode_cb_0xcb(&mut self) {
        alu_op_r8!(self, set, 1, e);
    }

    /// SET 1,H
    pub(super) fn opcode_cb_0xcc(&mut self) {
        alu_op_r8!(self, set, 1, h);
    }

    /// SET 1,L
    pub(super) fn opcode_cb_0xcd(&mut self) {
        alu_op_r8!(self, set, 1, l);
    }

    /// SET 1,(HL)
    pub(super) fn opcode_cb_0xce(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 1, [hl]);
    }

    /// SET 1,A
    pub(super) fn opcode_cb_0xcf(&mut self) {
        alu_op_r8!(self, set, 1, a);
    }

    /// SET 2,B
    pub(super) fn opcode_cb_0xd0(&mut self) {
        alu_op_r8!(self, set, 2, b);
    }

    /// SET 2,C
    pub(super) fn opcode_cb_0xd1(&mut self) {
        alu_op_r8!(self, set, 2, c);
    }

    /// SET 2,D
    pub(super) fn opcode_cb_0xd2(&mut self) {
        alu_op_r8!(self, set, 2, d);
    }

    /// SET 2,E
    pub(super) fn opcode_cb_0xd3(&mut self) {
        alu_op_r8!(self, set, 2, e);
    }

    /// SET 2,H
    pub(super) fn opcode_cb_0xd4(&mut self) {
        alu_op_r8!(self, set, 2, h);
    }

    /// SET 2,L
    pub(super) fn opcode_cb_0xd5(&mut self) {
        alu_op_r8!(self, set, 2, l);
    }

    /// SET 2,(HL)
    pub(super) fn opcode_cb_0xd6(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 2, [hl]);
    }

    /// SET 2,A
    pub(super) fn opcode_cb_0xd7(&mut self) {
        alu_op_r8!(self, set, 2, a);
    }

    /// SET 3,B
    pub(super) fn opcode_cb_0xd8(&mut self) {
        alu_op_r8!(self, set, 3, b);
    }

    /// SET 3,C
    pub(super) fn opcode_cb_0xd9(&mut self) {
        alu_op_r8!(self, set, 3, c);
    }

    /// SET 3,D
    pub(super) fn opcode_cb_0xda(&mut self) {
        alu_op_r8!(self, set, 3, d);
    }

    /// SET 3,E
    pub(super) fn opcode_cb_0xdb(&mut self) {
        alu_op_r8!(self, set, 3, e);
    }

    /// SET 3,H
    pub(super) fn opcode_cb_0xdc(&mut self) {
        alu_op_r8!(self, set, 3, h);
    }

    /// SET 3,L
    pub(super) fn opcode_cb_0xdd(&mut self) {
        alu_op_r8!(self, set, 3, l);
    }

    /// SET 3,(HL)
    pub(super) fn opcode_cb_0xde(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 3, [hl]);
    }

    /// SET 3,A
    pub(super) fn opcode_cb_0xdf(&mut self) {
        alu_op_r8!(self, set, 3, a);
    }

    /// SET 4,B
    pub(super) fn opcode_cb_0xe0(&mut self) {
        alu_op_r8!(self, set, 4, b);
    }

    /// SET 4,C
    pub(super) fn opcode_cb_0xe1(&mut self) {
        alu_op_r8!(self, set, 4, c);
    }

    /// SET 4,D
    pub(super) fn opcode_cb_0xe2(&mut self) {
        alu_op_r8!(self, set, 4, d);
    }

    /// SET 4,E
    pub(super) fn opcode_cb_0xe3(&mut self) {
        alu_op_r8!(self, set, 4, e);
    }

    /// SET 4,H
    pub(super) fn opcode_cb_0xe4(&mut self) {
        alu_op_r8!(self, set, 4, h);
    }

    /// SET 4,L
    pub(super) fn opcode_cb_0xe5(&mut self) {
        alu_op_r8!(self, set, 4, l);
    }

    /// SET 4,(HL)
    pub(super) fn opcode_cb_0xe6(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 4, [hl]);
    }

    /// SET 4,A
    pub(super) fn opcode_cb_0xe7(&mut self) {
        alu_op_r8!(self, set, 4, a);
    }

    /// SET 5,B
    pub(super) fn opcode_cb_0xe8(&mut self) {
        alu_op_r8!(self, set, 5, b);
    }

    /// SET 5,C
    pub(super) fn opcode_cb_0xe9(&mut self) {
        alu_op_r8!(self, set, 5, c);
    }

    /// SET 5,D
    pub(super) fn opcode_cb_0xea(&mut self) {
        alu_op_r8!(self, set, 5, d);
    }

    /// SET 5,E
    pub(super) fn opcode_cb_0xeb(&mut self) {
        alu_op_r8!(self, set, 5, e);
    }

    /// SET 5,H
    pub(super) fn opcode_cb_0xec(&mut self) {
        alu_op_r8!(self, set, 5, h);
    }

    /// SET 5,L
    pub(super) fn opcode_cb_0xed(&mut self) {
        alu_op_r8!(self, set, 5, l);
    }

    /// SET 5,(HL)
    pub(super) fn opcode_cb_0xee(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 5, [hl]);
    }

    /// SET 5,A
    pub(super) fn opcode_cb_0xef(&mut self) {
        alu_op_r8!(self, set, 5, a);
    }

    /// SET 6,B
    pub(super) fn opcode_cb_0xf0(&mut self) {
        alu_op_r8!(self, set, 6, b);
    }

    /// SET 6,C
    pub(super) fn opcode_cb_0xf1(&mut self) {
        alu_op_r8!(self, set, 6, c);
    }

    /// SET 6,D
    pub(super) fn opcode_cb_0xf2(&mut self) {
        alu_op_r8!(self, set, 6, d);
    }

    /// SET 6,E
    pub(super) fn opcode_cb_0xf3(&mut self) {
        alu_op_r8!(self, set, 6, e);
    }

    /// SET 6,H
    pub(super) fn opcode_cb_0xf4(&mut self) {
        alu_op_r8!(self, set, 6, h);
    }

    /// SET 6,L
    pub(super) fn opcode_cb_0xf5(&mut self) {
        alu_op_r8!(self, set, 6, l);
    }

    /// SET 6,(HL)
    pub(super) fn opcode_cb_0xf6(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 6, [hl]);
    }

    /// SET 6,A
    pub(super) fn opcode_cb_0xf7(&mut self) {
        alu_op_r8!(self, set, 6, a);
    }

    /// SET 7,B
    pub(super) fn opcode_cb_0xf8(&mut self) {
        alu_op_r8!(self, set, 7, b);
    }

    /// SET 7,C
    pub(super) fn opcode_cb_0xf9(&mut self) {
        alu_op_r8!(self, set, 7, c);
    }

    /// SET 7,D
    pub(super) fn opcode_cb_0xfa(&mut self) {
        alu_op_r8!(self, set, 7, d);
    }

    /// SET 7,E
    pub(super) fn opcode_cb_0xfb(&mut self) {
        alu_op_r8!(self, set, 7, e);
    }

    /// SET 7,H
    pub(super) fn opcode_cb_0xfc(&mut self) {
        alu_op_r8!(self, set, 7, h);
    }

    /// SET 7,L
    pub(super) fn opcode_cb_0xfd(&mut self) {
        alu_op_r8!(self, set, 7, l);
    }

    /// SET 7,(HL)
    pub(super) fn opcode_cb_0xfe(&mut self, memory: &mut Memory) {
        alu_op_r8!(self, memory, set, 7, [hl]);
    }

    /// SET 7,A
    pub(super) fn opcode_cb_0xff(&mut self) {
        alu_op_r8!(self, set, 7, a);
    }
}
