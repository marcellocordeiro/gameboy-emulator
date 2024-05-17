pub use self::{flags::Flags, ime_state::ImeState};

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,

    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,

    pub ime: ImeState, // Interrupt Master Enable (Write only).
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        let high = self.a as u16;
        let low = self.f.bits() as u16;

        (high << 8) | low
    }

    pub fn get_bc(&self) -> u16 {
        let high = self.b as u16;
        let low = self.c as u16;

        (high << 8) | low
    }

    pub fn get_de(&self) -> u16 {
        let high = self.d as u16;
        let low = self.e as u16;

        (high << 8) | low
    }

    pub fn get_hl(&self) -> u16 {
        let high = self.h as u16;
        let low = self.l as u16;

        (high << 8) | low
    }

    pub fn set_af(&mut self, word: u16) {
        let high = ((word & 0xFF00) >> 8) as u8;
        let low = (word & 0x00FF) as u8;

        self.a = high;
        self.f = Flags::from_bits_truncate(low); // Same as (low & 0xF0).
    }

    pub fn set_bc(&mut self, word: u16) {
        let high = ((word & 0xFF00) >> 8) as u8;
        let low = (word & 0x00FF) as u8;

        self.b = high;
        self.c = low;
    }

    pub fn set_de(&mut self, word: u16) {
        let high = ((word & 0xFF00) >> 8) as u8;
        let low = (word & 0x00FF) as u8;

        self.d = high;
        self.e = low;
    }

    pub fn set_hl(&mut self, word: u16) {
        let high = ((word & 0xFF00) >> 8) as u8;
        let low = (word & 0x00FF) as u8;

        self.h = high;
        self.l = low;
    }
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let af_line = format!(
            "AF: {:#06X}, A: {:#04X} | F: {:#04X}",
            self.get_af(),
            self.a,
            self.f.bits()
        );

        let bc_line = format!(
            "BC: {:#06X}, B: {:#04X} | C: {:#04X}",
            self.get_bc(),
            self.b,
            self.c
        );

        let de_line = format!(
            "DE: {:#06X}, D: {:#04X} | E: {:#04X}",
            self.get_de(),
            self.d,
            self.e
        );

        let hl_line = format!(
            "HL: {:#06X}, H: {:#04X} | L: {:#04X}",
            self.get_hl(),
            self.h,
            self.l
        );

        let pc_line = format!("PC: {:#06X}", self.pc);
        let sp_line = format!("SP: {:#06X}", self.sp);

        let flags_line = format!(
            "Flags: Z: {} | N: {} | H: {} | C: {}",
            if self.f.contains(Flags::ZERO) {
                "☑"
            } else {
                "☐"
            },
            if self.f.contains(Flags::N_ADD_SUB) {
                "☑"
            } else {
                "☐"
            },
            if self.f.contains(Flags::HALF_CARRY) {
                "☑"
            } else {
                "☐"
            },
            if self.f.contains(Flags::CARRY) {
                "☑"
            } else {
                "☐"
            }
        );

        write!(
            f,
            "{af_line}\n{bc_line}\n{de_line}\n{hl_line}\n\n{pc_line}\n{sp_line}\n\n{flags_line}"
        )
    }
}

mod flags;
mod ime_state;
