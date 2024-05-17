use crate::{
    cpu::{alu, Cpu},
    memory::MemoryInterface,
};

// Completed, may need some refactoring.

macro_rules! alu_reg8 {
    ($self:ident, $memory:ident, $F:path,[hl]) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte($memory, address);

        let result = $F(&mut $self.registers.f, value);

        $self.write_byte($memory, address, result);
    };

    ($self:ident, $F:path, $reg:ident) => {
        $self.registers.$reg = $F(&mut $self.registers.f, $self.registers.$reg);
    };
}

macro_rules! alu_acc_reg8 {
    ($self:ident, $memory:ident, $F:path,[hl]) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte($memory, address);

        $self.registers.a = $F(&mut $self.registers.f, $self.registers.a, value);
    };

    ($self:ident, $F:path, $reg:ident) => {
        $self.registers.a = $F(
            &mut $self.registers.f,
            $self.registers.a,
            $self.registers.$reg,
        );
    };
}

macro_rules! alu_acc_imm {
    ($self:ident, $memory:ident, $F:path) => {
        let value = $self.read_byte_operand($memory);

        $self.registers.a = $F(&mut $self.registers.f, $self.registers.a, value);
    };
}

macro_rules! alu_flags {
    ($self:ident, $memory:ident, $F:path,[hl]) => {
        let address = $self.registers.get_hl();
        let value = $self.read_byte($memory, address);

        $F(&mut $self.registers.f, $self.registers.a, value);
    };

    ($self:ident, $F:path, $reg:ident) => {
        $F(
            &mut $self.registers.f,
            $self.registers.a,
            $self.registers.$reg,
        );
    };
}

impl Cpu {
    /// INC B
    pub(super) fn opcode_0x04(&mut self) {
        alu_reg8!(self, alu::inc, b);
    }

    /// DEC B
    pub(super) fn opcode_0x05(&mut self) {
        alu_reg8!(self, alu::dec, b);
    }

    /// INC C
    pub(super) fn opcode_0x0c(&mut self) {
        alu_reg8!(self, alu::inc, c);
    }

    /// DEC C
    pub(super) fn opcode_0x0d(&mut self) {
        alu_reg8!(self, alu::dec, c);
    }

    /// INC D
    pub(super) fn opcode_0x14(&mut self) {
        alu_reg8!(self, alu::inc, d);
    }

    /// DEC D
    pub(super) fn opcode_0x15(&mut self) {
        alu_reg8!(self, alu::dec, d);
    }

    /// INC E
    pub(super) fn opcode_0x1c(&mut self) {
        alu_reg8!(self, alu::inc, e);
    }

    /// DEC E
    pub(super) fn opcode_0x1d(&mut self) {
        alu_reg8!(self, alu::dec, e);
    }

    /// INC H
    pub(super) fn opcode_0x24(&mut self) {
        alu_reg8!(self, alu::inc, h);
    }

    /// DEC H
    pub(super) fn opcode_0x25(&mut self) {
        alu_reg8!(self, alu::dec, h);
    }

    /// DAA
    pub(super) fn opcode_0x27(&mut self) {
        self.registers.a = alu::daa(&mut self.registers.f, self.registers.a);
    }

    /// INC L
    pub(super) fn opcode_0x2c(&mut self) {
        alu_reg8!(self, alu::inc, l);
    }

    /// DEC L
    pub(super) fn opcode_0x2d(&mut self) {
        alu_reg8!(self, alu::dec, l);
    }

    /// CPL
    pub(super) fn opcode_0x2f(&mut self) {
        self.registers.a = alu::cpl(&mut self.registers.f, self.registers.a);
    }

    /// INC (HL)
    pub(super) fn opcode_0x34(&mut self, memory: &mut impl MemoryInterface) {
        alu_reg8!(self, memory, alu::inc, [hl]);
    }

    /// DEC (HL)
    pub(super) fn opcode_0x35(&mut self, memory: &mut impl MemoryInterface) {
        alu_reg8!(self, memory, alu::dec, [hl]);
    }

    /// SCF
    pub(super) fn opcode_0x37(&mut self) {
        alu::scf(&mut self.registers.f);
    }

    /// INC A
    pub(super) fn opcode_0x3c(&mut self) {
        alu_reg8!(self, alu::inc, a);
    }

    /// DEC A
    pub(super) fn opcode_0x3d(&mut self) {
        alu_reg8!(self, alu::dec, a);
    }

    /// CCF
    pub(super) fn opcode_0x3f(&mut self) {
        alu::ccf(&mut self.registers.f);
    }

    /// ADD A,B
    pub(super) fn opcode_0x80(&mut self) {
        alu_acc_reg8!(self, alu::add, b);
    }

    /// ADD A,C
    pub(super) fn opcode_0x81(&mut self) {
        alu_acc_reg8!(self, alu::add, c);
    }

    /// ADD A,D
    pub(super) fn opcode_0x82(&mut self) {
        alu_acc_reg8!(self, alu::add, d);
    }

    /// ADD A,E
    pub(super) fn opcode_0x83(&mut self) {
        alu_acc_reg8!(self, alu::add, e);
    }

    /// ADD A,H
    pub(super) fn opcode_0x84(&mut self) {
        alu_acc_reg8!(self, alu::add, h);
    }

    /// ADD A,L
    pub(super) fn opcode_0x85(&mut self) {
        alu_acc_reg8!(self, alu::add, l);
    }

    /// ADD A,(HL)
    pub(super) fn opcode_0x86(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::add, [hl]);
    }

    /// ADD A,A
    pub(super) fn opcode_0x87(&mut self) {
        alu_acc_reg8!(self, alu::add, a);
    }

    /// ADC A,B
    pub(super) fn opcode_0x88(&mut self) {
        alu_acc_reg8!(self, alu::adc, b);
    }

    /// ADC A,C
    pub(super) fn opcode_0x89(&mut self) {
        alu_acc_reg8!(self, alu::adc, c);
    }

    /// ADC A,D
    pub(super) fn opcode_0x8a(&mut self) {
        alu_acc_reg8!(self, alu::adc, d);
    }

    /// ADC A,E
    pub(super) fn opcode_0x8b(&mut self) {
        alu_acc_reg8!(self, alu::adc, e);
    }

    /// ADC A,H
    pub(super) fn opcode_0x8c(&mut self) {
        alu_acc_reg8!(self, alu::adc, h);
    }

    /// ADC A,L
    pub(super) fn opcode_0x8d(&mut self) {
        alu_acc_reg8!(self, alu::adc, l);
    }

    /// ADC A,(HL)
    pub(super) fn opcode_0x8e(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::adc, [hl]);
    }

    /// ADC A,A
    pub(super) fn opcode_0x8f(&mut self) {
        alu_acc_reg8!(self, alu::adc, a);
    }

    /// SUB A,B
    pub(super) fn opcode_0x90(&mut self) {
        alu_acc_reg8!(self, alu::sub, b);
    }

    /// SUB A,C
    pub(super) fn opcode_0x91(&mut self) {
        alu_acc_reg8!(self, alu::sub, c);
    }

    /// SUB A,D
    pub(super) fn opcode_0x92(&mut self) {
        alu_acc_reg8!(self, alu::sub, d);
    }

    /// SUB A,E
    pub(super) fn opcode_0x93(&mut self) {
        alu_acc_reg8!(self, alu::sub, e);
    }

    /// SUB A,H
    pub(super) fn opcode_0x94(&mut self) {
        alu_acc_reg8!(self, alu::sub, h);
    }

    /// SUB A,L
    pub(super) fn opcode_0x95(&mut self) {
        alu_acc_reg8!(self, alu::sub, l);
    }

    /// SUB A,(HL)
    pub(super) fn opcode_0x96(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::sub, [hl]);
    }

    /// SUB A,A
    pub(super) fn opcode_0x97(&mut self) {
        alu_acc_reg8!(self, alu::sub, a);
    }

    /// SBC A,B
    pub(super) fn opcode_0x98(&mut self) {
        alu_acc_reg8!(self, alu::sbc, b);
    }

    /// SBC A,C
    pub(super) fn opcode_0x99(&mut self) {
        alu_acc_reg8!(self, alu::sbc, c);
    }

    /// SBC A,D
    pub(super) fn opcode_0x9a(&mut self) {
        alu_acc_reg8!(self, alu::sbc, d);
    }

    /// SBC A,E
    pub(super) fn opcode_0x9b(&mut self) {
        alu_acc_reg8!(self, alu::sbc, e);
    }

    /// SBC A,H
    pub(super) fn opcode_0x9c(&mut self) {
        alu_acc_reg8!(self, alu::sbc, h);
    }

    /// SBC A,L
    pub(super) fn opcode_0x9d(&mut self) {
        alu_acc_reg8!(self, alu::sbc, l);
    }

    /// SBC A,(HL)
    pub(super) fn opcode_0x9e(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::sbc, [hl]);
    }

    /// SBC A,A
    pub(super) fn opcode_0x9f(&mut self) {
        alu_acc_reg8!(self, alu::sbc, a);
    }

    /// AND A,B
    pub(super) fn opcode_0xa0(&mut self) {
        alu_acc_reg8!(self, alu::and, b);
    }

    /// AND A,C
    pub(super) fn opcode_0xa1(&mut self) {
        alu_acc_reg8!(self, alu::and, c);
    }

    /// AND A,D
    pub(super) fn opcode_0xa2(&mut self) {
        alu_acc_reg8!(self, alu::and, d);
    }

    /// AND A,E
    pub(super) fn opcode_0xa3(&mut self) {
        alu_acc_reg8!(self, alu::and, e);
    }

    /// AND A,H
    pub(super) fn opcode_0xa4(&mut self) {
        alu_acc_reg8!(self, alu::and, h);
    }

    /// AND A,L
    pub(super) fn opcode_0xa5(&mut self) {
        alu_acc_reg8!(self, alu::and, l);
    }

    /// AND A,(HL)
    pub(super) fn opcode_0xa6(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::and, [hl]);
    }

    /// AND A,A
    pub(super) fn opcode_0xa7(&mut self) {
        alu_acc_reg8!(self, alu::and, a);
    }

    /// XOR A,B
    pub(super) fn opcode_0xa8(&mut self) {
        alu_acc_reg8!(self, alu::xor, b);
    }

    /// XOR A,C
    pub(super) fn opcode_0xa9(&mut self) {
        alu_acc_reg8!(self, alu::xor, c);
    }

    /// XOR A,D
    pub(super) fn opcode_0xaa(&mut self) {
        alu_acc_reg8!(self, alu::xor, d);
    }

    /// XOR A,E
    pub(super) fn opcode_0xab(&mut self) {
        alu_acc_reg8!(self, alu::xor, e);
    }

    /// XOR A,H
    pub(super) fn opcode_0xac(&mut self) {
        alu_acc_reg8!(self, alu::xor, h);
    }

    /// XOR A,L
    pub(super) fn opcode_0xad(&mut self) {
        alu_acc_reg8!(self, alu::xor, l);
    }

    /// XOR A,(HL)
    pub(super) fn opcode_0xae(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::xor, [hl]);
    }

    /// XOR A,A
    pub(super) fn opcode_0xaf(&mut self) {
        alu_acc_reg8!(self, alu::xor, a);
    }

    /// OR A,B
    pub(super) fn opcode_0xb0(&mut self) {
        alu_acc_reg8!(self, alu::or, b);
    }

    /// OR A,C
    pub(super) fn opcode_0xb1(&mut self) {
        alu_acc_reg8!(self, alu::or, c);
    }

    /// OR A,D
    pub(super) fn opcode_0xb2(&mut self) {
        alu_acc_reg8!(self, alu::or, d);
    }

    /// OR A,E
    pub(super) fn opcode_0xb3(&mut self) {
        alu_acc_reg8!(self, alu::or, e);
    }

    /// OR A,H
    pub(super) fn opcode_0xb4(&mut self) {
        alu_acc_reg8!(self, alu::or, h);
    }

    /// OR A,L
    pub(super) fn opcode_0xb5(&mut self) {
        alu_acc_reg8!(self, alu::or, l);
    }

    /// OR A,(HL)
    pub(super) fn opcode_0xb6(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_reg8!(self, memory, alu::or, [hl]);
    }

    /// OR A,A
    pub(super) fn opcode_0xb7(&mut self) {
        alu_acc_reg8!(self, alu::or, a);
    }

    /// CP A,B
    pub(super) fn opcode_0xb8(&mut self) {
        alu_flags!(self, alu::cp, b);
    }

    /// CP A,C
    pub(super) fn opcode_0xb9(&mut self) {
        alu_flags!(self, alu::cp, c);
    }

    /// CP A,D
    pub(super) fn opcode_0xba(&mut self) {
        alu_flags!(self, alu::cp, d);
    }

    /// CP A,E
    pub(super) fn opcode_0xbb(&mut self) {
        alu_flags!(self, alu::cp, e);
    }

    /// CP A,H
    pub(super) fn opcode_0xbc(&mut self) {
        alu_flags!(self, alu::cp, h);
    }

    /// CP A,L
    pub(super) fn opcode_0xbd(&mut self) {
        alu_flags!(self, alu::cp, l);
    }

    /// CP A,(HL)
    pub(super) fn opcode_0xbe(&mut self, memory: &mut impl MemoryInterface) {
        alu_flags!(self, memory, alu::cp, [hl]);
    }

    /// CP A,A
    pub(super) fn opcode_0xbf(&mut self) {
        alu_flags!(self, alu::cp, a);
    }

    /// ADD A,u8
    pub(super) fn opcode_0xc6(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::add);
    }

    /// ADC A,u8
    pub(super) fn opcode_0xce(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::adc);
    }

    /// SUB A,u8
    pub(super) fn opcode_0xd6(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::sub);
    }

    /// SBC A,u8
    pub(super) fn opcode_0xde(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::sbc);
    }

    /// AND A,u8
    pub(super) fn opcode_0xe6(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::and);
    }

    /// XOR A,u8
    pub(super) fn opcode_0xee(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::xor);
    }

    /// OR A,u8
    pub(super) fn opcode_0xf6(&mut self, memory: &mut impl MemoryInterface) {
        alu_acc_imm!(self, memory, alu::or);
    }

    /// CP A,u8
    pub(super) fn opcode_0xfe(&mut self, memory: &mut impl MemoryInterface) {
        let value = self.read_byte_operand(memory);

        alu::cp(&mut self.registers.f, self.registers.a, value);
    }
}
