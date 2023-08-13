// TODO: IRQ

use crate::constants::Button;

use self::line_selection::LineSelection;

pub struct Joypad {
    joyp: u8,
    buttons: u8,

    line_selection: LineSelection,

    pub irq: bool,
}

impl Default for Joypad {
    fn default() -> Self {
        Self {
            joyp: 0b1100_1111,
            buttons: 0x00,

            line_selection: LineSelection::Both,

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
        self.line_selection = LineSelection::from_joyp_bits(value);

        self.update_joyp();
    }

    fn update_joyp(&mut self) {
        self.joyp &= 0b1100_0000;

        let buttons_bits = {
            use LineSelection::*;

            let bits = match self.line_selection {
                Both => (self.buttons | (self.buttons >> 4)) & 0b1111,
                Action => self.buttons & 0b1111,
                Direction => self.buttons >> 4,
                None => 0b0000,
            };

            !bits & 0b1111
        };

        self.joyp |= self.line_selection.to_joyp_bits();
        self.joyp |= buttons_bits;
    }

    pub fn key_down(&mut self, key: Button) {
        self.buttons |= key as u8;
        self.irq = true; // this is wrong.

        self.update_joyp();
    }

    pub fn key_up(&mut self, key: Button) {
        self.buttons &= !(key as u8);

        self.update_joyp();
    }
}

mod line_selection;

#[cfg(test)]
mod tests {
    use super::*;
    use Button::*;

    #[test]
    fn test_initial_read() {
        let joypad = Joypad::default();
        assert_eq!(joypad.read(), 0b1100_1111);
    }

    #[test]
    fn test_invalid_write() {
        let mut joypad = Joypad::default();

        let initial_state = (
            joypad.joyp,
            joypad.buttons,
            joypad.line_selection,
            joypad.irq,
        );

        joypad.write(0b1100_1111);

        let next_state = (
            joypad.joyp,
            joypad.buttons,
            joypad.line_selection,
            joypad.irq,
        );

        assert_eq!(initial_state, next_state);
    }

    #[test]
    fn test_a_button_press() {
        let mut joypad = Joypad::default();

        // All up
        assert_eq!(joypad.read(), 0b1100_1111);

        joypad.write(0b0001_0000);
        joypad.key_down(A);

        // Only A is down
        assert_eq!(joypad.read(), 0b1101_1110);

        // Select `None`
        joypad.write(0b0011_0000);
        assert_eq!(joypad.read(), 0b1111_1111);
    }
}
