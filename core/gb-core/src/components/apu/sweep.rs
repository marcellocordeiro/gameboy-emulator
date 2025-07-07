#[derive(Debug, Default)]
pub struct Sweep {
    pace: u8,
    increasing: bool,
    individual_step: u8,

    counter: u8,
}

impl Sweep {
    pub fn tick(&mut self) {
        if !self.enabled() {
            return;
        }

        if self.counter == 0 {
            return;
        }

        self.counter -= 1;
    }

    pub fn enabled(&self) -> bool {
        self.pace != 0
    }

    pub fn expired(&self) -> bool {
        self.counter == 0
    }

    pub fn reload(&mut self) {
        if self.pace > 0 {
            self.counter = self.pace;
        } else {
            self.counter = 8;
        }
    }

    pub fn get_new_frequency(&self, current_frequency: u16) -> (u16, bool) {
        let new_frequency = if self.increasing {
            current_frequency + (current_frequency >> self.individual_step)
        } else {
            current_frequency - (current_frequency >> self.individual_step)
        };

        let should_disable_channel = self.increasing && new_frequency > 0x07FF;

        if should_disable_channel {
            (current_frequency, true)
        } else {
            (new_frequency, false)
        }
    }

    pub fn read(&self) -> u8 {
        let increasing_bit = self.increasing as u8;

        (self.pace << 4) | (increasing_bit << 3) | self.individual_step
    }

    pub fn write(&mut self, value: u8) {
        let sweep_pace = (value & 0b0111_0000) >> 4;
        let increasing = (value & 0b1000) != 0;
        let individual_step = value & 0b0111;

        self.pace = sweep_pace;
        self.increasing = increasing;
        self.individual_step = individual_step;
    }
}
