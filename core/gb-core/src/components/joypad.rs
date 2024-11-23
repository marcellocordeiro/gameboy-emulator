use self::line_selection::{JOYP_SELECTION_MASK, LineSelection};
use crate::utils::button::Button;

const JOYP_UNUSED_MASK: u8 = 0b1100_0000;
const JOYP_BUTTONS_MASK: u8 = 0b0000_1111;

pub struct Joypad {
    joyp: u8,
    buttons: u8,

    pub(crate) irq: bool,
}

impl Default for Joypad {
    fn default() -> Self {
        Self {
            joyp: 0b0000_1111,
            buttons: 0x00,

            irq: false,
        }
    }
}

impl Joypad {
    // 0xFF00

    pub(crate) fn read(&self) -> u8 {
        JOYP_UNUSED_MASK | self.joyp
    }

    pub(crate) fn write(&mut self, value: u8) {
        // Only bits 4 and 5 are writable.
        self.joyp &= !JOYP_SELECTION_MASK;
        self.joyp |= value & JOYP_SELECTION_MASK;

        self.update_joyp();
    }

    pub(crate) fn set_joypad_button(&mut self, button: Button, value: bool) {
        if value {
            self.joypad_button_down(button);
        } else {
            self.joypad_button_up(button);
        }
    }

    pub(crate) fn joypad_button_down(&mut self, button: Button) {
        let current_buttons = self.buttons;
        let new_buttons = self.buttons | (button as u8);

        if new_buttons != current_buttons {
            self.buttons = new_buttons;
            self.irq = true;

            self.update_joyp();
        }
    }

    pub(crate) fn joypad_button_up(&mut self, key: Button) {
        self.buttons &= !(key as u8);

        self.update_joyp();
    }

    fn update_joyp(&mut self) {
        let buttons_bits = match LineSelection::from_joyp_bits(self.joyp) {
            LineSelection::Both => (self.buttons | (self.buttons >> 4)) & 0b1111,
            LineSelection::Action => self.buttons & 0b1111,
            LineSelection::Direction => self.buttons >> 4,
            LineSelection::None => 0b0000,
        };

        self.joyp &= !JOYP_BUTTONS_MASK;
        self.joyp |= !buttons_bits & JOYP_BUTTONS_MASK;
    }
}

mod line_selection;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_read() {
        let joypad = Joypad::default();
        assert_eq!(joypad.read(), 0b1100_1111);
    }

    #[test]
    fn test_invalid_write() {
        let mut joypad = Joypad::default();

        let initial_state = (joypad.joyp, joypad.buttons, joypad.irq);

        joypad.write(0b1100_1111);

        let next_state = (joypad.joyp, joypad.buttons, joypad.irq);

        assert_eq!(initial_state, next_state);
    }

    #[test]
    fn test_a_button_press() {
        let mut joypad = Joypad::default();

        // All up
        assert_eq!(joypad.read(), 0b1100_1111);

        joypad.write(0b0001_0000);
        joypad.joypad_button_down(Button::A);

        // Only A is down
        assert_eq!(joypad.read(), 0b1101_1110);

        // Select `None`
        joypad.write(0b0011_0000);
        assert_eq!(joypad.read(), 0b1111_1111);
    }
}
