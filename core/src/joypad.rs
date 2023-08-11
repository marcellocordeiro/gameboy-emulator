// TODO: everything

use crate::constants::Button;

enum Selection {
    None,
    Action,
    Direction,
}

impl From<u8> for Selection {
    fn from(value: u8) -> Self {
        use Selection::*;

        match (value >> 4) & 0b11 {
            0b01 => Action,
            0b10 => Direction,
            _ => None,
        }
    }
}

pub struct Joypad {
    joyp: u8,
    buttons: u8,

    selection: Selection,

    pub irq: bool,
}

impl Default for Joypad {
    fn default() -> Self {
        Self {
            joyp: 0x0F,
            buttons: 0x00,

            selection: Selection::None,

            irq: false,
        }
    }
}

impl Joypad {
    // 0xFF00

    pub fn read(&self) -> u8 {
        0b1100_0000 | self.joyp
    }

    pub fn write(&mut self, value: u8) {
        // Only bits 4 and 5 are writable.
        self.selection = match (value >> 4) & 0b11 {
            0b01 => Selection::Action,
            0b10 => Selection::Direction,
            _ => Selection::None,
        };

        self.update_joyp();
    }

    fn update_joyp(&mut self) {
        let state = match self.selection {
            Selection::Action => self.buttons & 0b1111,
            Selection::Direction => self.buttons >> 4,
            Selection::None => 0b0000,
        };

        self.joyp = !state;
    }

    pub fn key_down(&mut self, key: Button) {
        self.buttons |= key as u8;
        self.irq = true;

        self.update_joyp();
    }

    pub fn key_up(&mut self, key: Button) {
        self.buttons &= !(key as u8);

        self.update_joyp();
    }
}
