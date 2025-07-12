#[derive(Debug, Default)]
pub struct Sweep {
    pace: u8,
    direction: Direction,
    individual_step: u8,

    shadow_frequency: u16,
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
        self.counter = if self.pace > 0 { self.pace } else { 8 };
    }

    pub fn set_shadow_frequency(&mut self, frequency: u16) {
        self.shadow_frequency = frequency;
    }

    pub fn get_new_period(&self) -> (u16, bool) {
        let delta = self.shadow_frequency >> self.individual_step;
        let new_period = match self.direction {
            Direction::Increasing => self.shadow_frequency + delta,
            Direction::Decreasing => self.shadow_frequency - delta,
        };

        let should_disable_channel = self.direction == Direction::Increasing && new_period > 0x07FF;

        if should_disable_channel {
            (self.shadow_frequency, true)
        } else {
            (new_period, false)
        }
    }

    pub fn read(&self) -> u8 {
        let direction_bit = self.direction as u8;

        (self.pace << 4) | (direction_bit << 3) | self.individual_step
    }

    pub fn write(&mut self, value: u8) {
        let pace = (value & 0b0111_0000) >> 4;
        let direction = Direction::from((value & 0b1000) != 0);
        let individual_step = value & 0b0111;

        self.pace = pace;
        self.direction = direction;
        self.individual_step = individual_step;
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
enum Direction {
    #[default]
    Increasing = 0,
    Decreasing = 1,
}

impl From<bool> for Direction {
    /// Unlike the envelope direction,
    /// the sweep direction is `Decreasing` when the bit is set.
    fn from(value: bool) -> Self {
        if value {
            Self::Decreasing
        } else {
            Self::Increasing
        }
    }
}
