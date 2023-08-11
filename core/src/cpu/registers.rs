use std::fmt;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const ZERO = 1 << 7;
        const N_ADD_SUB = 1 << 6;
        const HALF_CARRY = 1 << 5;
        const CARRY = 1 << 4;

        // Bits 0-3 are unused.
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ImeState {
    #[default]
    Disabled,
    Enabled,
    Pending,
}

impl ImeState {
    pub fn get_status(self) -> bool {
        match self {
            Self::Disabled | Self::Pending => false,
            Self::Enabled => true,
        }
    }

    pub fn update_and_get_status(&mut self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
            Self::Pending => {
                *self = Self::Enabled;

                false
            }
        }
    }
}

#[derive(Default)]
pub struct Registers {
    pub accumulator: u8,
    pub flags: Flags,

    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub program_counter: u16,
    pub stack_pointer: u16,

    pub ime: ImeState, // Interrupt Master Enable (Write only).
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        let high = self.accumulator as u16;
        let low = self.flags.bits() as u16;

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

        self.accumulator = high;
        self.flags = Flags::from_bits_truncate(low); // Same as (low & 0xF0).
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

impl fmt::Display for ImeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Self::Disabled => "☐",
            Self::Enabled => "☑",
            Self::Pending => "~",
        };

        write!(f, "{str}")
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let af_line = format!(
            "AF: {:#06X}, A: {:#04X} | F: {:#04X}",
            self.get_af(),
            self.accumulator,
            self.flags.bits()
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

        let pc_line = format!("PC: {:#06X}", self.program_counter);
        let sp_line = format!("SP: {:#06X}", self.stack_pointer);

        let flags_line = format!(
            "Flags: Z: {} | N: {} | H: {} | C: {}",
            if self.flags.contains(Flags::ZERO) {
                "☑"
            } else {
                "☐"
            },
            if self.flags.contains(Flags::N_ADD_SUB) {
                "☑"
            } else {
                "☐"
            },
            if self.flags.contains(Flags::HALF_CARRY) {
                "☑"
            } else {
                "☐"
            },
            if self.flags.contains(Flags::CARRY) {
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
