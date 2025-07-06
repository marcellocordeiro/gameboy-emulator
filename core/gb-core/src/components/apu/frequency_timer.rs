#[derive(Debug, Default)]
pub struct FrequencyTimer<T: Fn(u16) -> u16> {
    counter: u16,
    frequency: u16,

    calculate_frequency: T,
}

impl<T: Fn(u16) -> u16> FrequencyTimer<T> {
    pub fn new(calculate_frequency: T) -> Self {
        Self {
            counter: 0,
            frequency: 0,
            calculate_frequency,
        }
    }

    pub fn tick(&mut self) {
        if self.counter == 0 {
            return;
        }

        self.counter -= 1;
    }

    pub fn reload(&mut self) {
        let counter = (self.calculate_frequency)(self.frequency);

        self.counter = counter;
    }

    pub fn frequency(&self) -> u16 {
        self.frequency
    }

    pub fn set_frequency(&mut self, value: u16) {
        self.frequency = value;
    }

    pub fn set_frequency_low(&mut self, value: u8) {
        const MASK: u16 = 0xFF;
        let value = value as u16;

        self.frequency &= !MASK;
        self.frequency |= value;
    }

    pub fn set_frequency_high(&mut self, value: u8) {
        const MASK: u16 = 0b111;
        let value = ((value as u16) & MASK) << 8;

        self.frequency &= !(MASK << 8);
        self.frequency |= value;
    }

    pub fn expired(&self) -> bool {
        self.counter == 0
    }
}
