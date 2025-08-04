use crate::components::{
    cpu::{Cpu, registers::Flags},
    memory::MemoryInterface,
};

impl Cpu {
    /// JR i8
    pub(super) fn opcode_0x18(&mut self, memory: &mut impl MemoryInterface) {
        let offset = self.read_byte_operand(memory) as i8;
        self.jr(memory, offset);
    }

    /// JR NZ,i8
    pub(super) fn opcode_0x20(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::ZERO);
        self.jr_cc(memory, condition);
    }

    /// JR Z,i8
    pub(super) fn opcode_0x28(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::ZERO);
        self.jr_cc(memory, condition);
    }

    /// JR NC,i8
    pub(super) fn opcode_0x30(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::CARRY);
        self.jr_cc(memory, condition);
    }

    /// JR C,i8
    pub(super) fn opcode_0x38(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::CARRY);
        self.jr_cc(memory, condition);
    }

    /// RET NZ
    pub(super) fn opcode_0xc0(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::ZERO);
        self.ret_cc(memory, condition);
    }

    /// JP NZ,u16
    pub(super) fn opcode_0xc2(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::ZERO);
        self.jp_cc(memory, condition);
    }

    /// JP u16
    pub(super) fn opcode_0xc3(&mut self, memory: &mut impl MemoryInterface) {
        let address = self.read_word_operand(memory);
        self.jp(memory, address);
    }

    /// CALL NZ,u16
    pub(super) fn opcode_0xc4(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::ZERO);
        self.call_cc(memory, condition);
    }

    /// RST 00h
    pub(super) fn opcode_0xc7(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x00);
    }

    /// RET Z
    pub(super) fn opcode_0xc8(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::ZERO);
        self.ret_cc(memory, condition);
    }

    /// RET
    pub(super) fn opcode_0xc9(&mut self, memory: &mut impl MemoryInterface) {
        self.ret(memory);
    }

    /// JP Z,u16
    pub(super) fn opcode_0xca(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::ZERO);
        self.jp_cc(memory, condition);
    }

    /// CALL Z,u16
    pub(super) fn opcode_0xcc(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::ZERO);
        self.call_cc(memory, condition);
    }

    /// CALL u16
    pub(super) fn opcode_0xcd(&mut self, memory: &mut impl MemoryInterface) {
        let address = self.read_word_operand(memory);
        self.call(memory, address);
    }

    /// RST 08h
    pub(super) fn opcode_0xcf(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x08);
    }

    /// RET NC
    pub(super) fn opcode_0xd0(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::CARRY);
        self.ret_cc(memory, condition);
    }

    /// JP NC,u16
    pub(super) fn opcode_0xd2(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::CARRY);
        self.jp_cc(memory, condition);
    }

    /// CALL NC,u16
    pub(super) fn opcode_0xd4(&mut self, memory: &mut impl MemoryInterface) {
        let condition = !self.registers.f.contains(Flags::CARRY);
        self.call_cc(memory, condition);
    }

    /// RST 10h
    pub(super) fn opcode_0xd7(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x10);
    }

    /// RET C
    pub(super) fn opcode_0xd8(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::CARRY);
        self.ret_cc(memory, condition);
    }

    /// RETI
    pub(super) fn opcode_0xd9(&mut self, memory: &mut impl MemoryInterface) {
        self.registers.ime.force_enable();
        self.ret(memory);
    }

    /// JP C,u16
    pub(super) fn opcode_0xda(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::CARRY);
        self.jp_cc(memory, condition);
    }

    /// CALL C,u16
    pub(super) fn opcode_0xdc(&mut self, memory: &mut impl MemoryInterface) {
        let condition = self.registers.f.contains(Flags::CARRY);
        self.call_cc(memory, condition);
    }

    /// RST 18h
    pub(super) fn opcode_0xdf(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x18);
    }

    /// RST 20h
    pub(super) fn opcode_0xe7(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x20);
    }

    /// JP HL
    pub(super) fn opcode_0xe9(&mut self) {
        let address = self.registers.get_hl();
        self.registers.pc = address;
    }

    /// RST 28h
    pub(super) fn opcode_0xef(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x28);
    }

    /// RST 30h
    pub(super) fn opcode_0xf7(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x30);
    }

    /// RST 38h
    pub(super) fn opcode_0xff(&mut self, memory: &mut impl MemoryInterface) {
        self.call(memory, 0x38);
    }
}
