#[derive(Debug, Default)]
pub struct SpeedSwitch {
    key0: u8,

    cgb_mode: bool,
}

impl SpeedSwitch {
    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    pub fn in_double_speed(&self) -> bool {
        (self.key0 & 0b1000_0000) != 0
    }

    pub fn process_speed_switch(&mut self) {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return;
        }

        if self.key0 & 0b1 == 1 {
            self.key0 = !self.key0 & 0b1000_0000;
        }
    }

    pub fn read(&self) -> u8 {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return 0xFF;
        }

        0b0111_1110 | self.key0
    }

    pub fn write(&mut self, value: u8) {
        if !(cfg!(feature = "cgb") && self.cgb_mode) {
            return;
        }

        self.key0 |= value & 0b1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sanity() {
        let mut speed_switch = SpeedSwitch::default();

        if cfg!(feature = "cgb") {
            assert_eq!(speed_switch.read(), 0b0111_1110);

            speed_switch.write(0xFF);
            assert_eq!(speed_switch.read(), 0b0111_1111);
        } else {
            assert_eq!(speed_switch.read(), 0xFF);
            speed_switch.write(0xFF);
            assert_eq!(speed_switch.read(), 0xFF);
        }
    }

    #[test]
    #[cfg(feature = "cgb")]
    fn test_switch() {
        let mut speed_switch = SpeedSwitch::default();
        speed_switch.set_cgb_mode(true);

        assert_eq!(speed_switch.key0, 0);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert_eq!(speed_switch.in_double_speed(), false);

        // No changes.
        speed_switch.write(0b1111_1110);
        assert_eq!(speed_switch.key0, 0);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert_eq!(speed_switch.in_double_speed(), false);

        // No changes.
        speed_switch.process_speed_switch();
        assert_eq!(speed_switch.key0, 0);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert_eq!(speed_switch.in_double_speed(), false);

        // Request speed switch (Single -> Double).
        speed_switch.write(0b1111_1111);
        assert_eq!(speed_switch.key0, 0b1);
        assert_eq!(speed_switch.read(), 0b0111_1111);
        assert_eq!(speed_switch.in_double_speed(), false);

        // Process speed switch (Single -> Double).
        speed_switch.process_speed_switch();
        assert_eq!(speed_switch.key0, 0b1000_0000);
        assert_eq!(speed_switch.read(), 0b1111_1110);
        assert_eq!(speed_switch.in_double_speed(), true);

        // Request speed switch (Double -> Single).
        speed_switch.write(0b1111_1111);
        assert_eq!(speed_switch.key0, 0b1000_0001);
        assert_eq!(speed_switch.read(), 0b1111_1111);
        assert_eq!(speed_switch.in_double_speed(), true);

        // Process speed switch (Double -> Single).
        speed_switch.process_speed_switch();
        assert_eq!(speed_switch.key0, 0b0000_0000);
        assert_eq!(speed_switch.read(), 0b0111_1110);
        assert_eq!(speed_switch.in_double_speed(), false);
    }
}
