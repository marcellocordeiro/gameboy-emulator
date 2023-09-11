#[derive(Debug, Default)]
pub struct Speed {
    key0: u8,

    cgb_mode: bool,
}

impl Speed {
    pub fn set_cgb_mode(&mut self, value: bool) {
        self.key0 = 0;
        self.cgb_mode = value;
    }

    pub fn in_double_speed(&self) -> bool {
        (self.key0 & 0b1000_0000) != 0
    }

    pub fn process_speed_switch(&mut self) {
        if self.key0 & 0b1 == 1 {
            self.key0 = !self.key0 & 0b1000_0000;
        }
    }

    pub fn read(&self) -> u8 {
        0b0111_1110 | self.key0
    }

    pub fn write(&mut self, value: u8) {
        self.key0 |= value & 0b1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch() {
        let mut speed = Speed::default();
        speed.set_cgb_mode(true);

        assert_eq!(speed.key0, 0);
        assert_eq!(speed.read(), 0b0111_1110);
        assert_eq!(speed.in_double_speed(), false);

        // No changes.
        speed.write(0b1111_1110);
        assert_eq!(speed.key0, 0);
        assert_eq!(speed.read(), 0b0111_1110);
        assert_eq!(speed.in_double_speed(), false);

        // No changes.
        speed.process_speed_switch();
        assert_eq!(speed.key0, 0);
        assert_eq!(speed.read(), 0b0111_1110);
        assert_eq!(speed.in_double_speed(), false);

        // Request speed switch (Single -> Double).
        speed.write(0b1111_1111);
        assert_eq!(speed.key0, 0b1);
        assert_eq!(speed.read(), 0b0111_1111);
        assert_eq!(speed.in_double_speed(), false);

        // Process speed switch (Single -> Double).
        speed.process_speed_switch();
        assert_eq!(speed.key0, 0b1000_0000);
        assert_eq!(speed.read(), 0b1111_1110);
        assert_eq!(speed.in_double_speed(), true);

        // Request speed switch (Double -> Single).
        speed.write(0b1111_1111);
        assert_eq!(speed.key0, 0b1000_0001);
        assert_eq!(speed.read(), 0b1111_1111);
        assert_eq!(speed.in_double_speed(), true);

        // Process speed switch (Double -> Single).
        speed.process_speed_switch();
        assert_eq!(speed.key0, 0b0000_0000);
        assert_eq!(speed.read(), 0b0111_1110);
        assert_eq!(speed.in_double_speed(), false);
    }
}
