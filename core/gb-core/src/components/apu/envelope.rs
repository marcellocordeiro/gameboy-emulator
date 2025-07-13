#[derive(Debug, Default)]
pub struct Envelope {
    initial_volume: u8,
    direction: Direction,
    sweep_pace: u8,

    volume: u8,
    counter: u8,
}

impl Envelope {
    pub fn tick(&mut self) {
        if self.sweep_pace == 0 {
            return;
        }

        if self.counter > 0 {
            self.counter -= 1;

            if self.counter != 0 {
                return;
            }
        }

        self.counter = self.sweep_pace;
        self.next_volume();
    }

    pub fn trigger(&mut self) {
        self.volume = self.initial_volume;
        self.counter = self.sweep_pace;
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn next_volume(&mut self) {
        match self.direction {
            Direction::Decreasing => {
                if self.volume > 0 {
                    self.volume -= 1;
                }
            }

            Direction::Increasing => {
                if self.volume < 0b1111 {
                    self.volume += 1;
                }
            }
        }
    }

    pub fn read(&self) -> u8 {
        let direction_bit = self.direction as u8;

        (self.initial_volume << 4) | (direction_bit << 3) | self.sweep_pace
    }

    pub fn write(&mut self, value: u8) {
        self.initial_volume = (value & 0b1111_0000) >> 4;
        self.direction = Direction::from((value & 0b1000) != 0);
        self.sweep_pace = value & 0b0111;
    }
}

#[derive(Debug, Default, Copy, Clone)]
enum Direction {
    #[default]
    Decreasing = 0,
    Increasing = 1,
}

impl From<bool> for Direction {
    /// Unlike the sweep direction,
    /// the envelope direction is `Increasing` when the bit is set.
    fn from(value: bool) -> Self {
        if value {
            Self::Increasing
        } else {
            Self::Decreasing
        }
    }
}
