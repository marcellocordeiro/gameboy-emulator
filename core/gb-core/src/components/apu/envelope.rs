#[derive(Debug, Default)]
pub struct Envelope {
    initial_volume: u8,
    increasing: bool,
    sweep_pace: u8,

    counter: u8,
    current_volume: u8,
}

impl Envelope {
    pub fn reload(&mut self) {
        self.current_volume = self.initial_volume;
        self.counter = self.sweep_pace;
    }

    pub fn tick(&mut self) {
        if self.sweep_pace == 0 {
            return;
        }

        if self.counter == 0 {
            return;
        }

        self.counter -= 1;

        if self.counter == 0 {
            self.counter = self.sweep_pace;
            self.set_next_volume();
        }
    }

    pub fn set_next_volume(&mut self) {
        if self.increasing && self.current_volume < 0b1111 {
            self.current_volume += 1;
        } else if self.current_volume > 0 {
            self.current_volume -= 1;
        }
    }

    pub fn current_volume(&self) -> u8 {
        self.current_volume
    }

    pub fn read(&self) -> u8 {
        let increasing_bit = self.increasing as u8;

        (self.initial_volume << 4) | (increasing_bit << 3) | self.sweep_pace
    }

    pub fn write(&mut self, value: u8) {
        // TODO: only set after the next trigger?
        self.initial_volume = (value & 0b1111_0000) >> 4;
        self.increasing = (value & 0b1000) != 0;
        self.sweep_pace = value & 0b0111;
    }
}
