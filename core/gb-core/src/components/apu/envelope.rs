#[derive(Debug, Default)]
pub struct Envelope {
    // Register values will only be applied in the next trigger
    initial_volume: u8,
    direction: Direction,
    sweep_pace: u8,

    state: State,
}

impl Envelope {
    pub fn reload(&mut self) {
        self.state = State {
            sweep_pace: self.sweep_pace,
            direction: self.direction,
            volume: self.initial_volume,
            counter: self.sweep_pace,
        }
    }

    pub fn tick(&mut self) {
        self.state.tick();
    }

    pub fn current_volume(&self) -> u8 {
        self.state.volume
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

#[derive(Debug, Default)]
struct State {
    sweep_pace: u8,
    direction: Direction,

    volume: u8,
    counter: u8,
}

impl State {
    fn tick(&mut self) {
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
}

#[derive(Debug, Default, Copy, Clone)]
enum Direction {
    #[default]
    Decreasing = 0,
    Increasing = 1,
}

impl From<bool> for Direction {
    fn from(value: bool) -> Self {
        if value {
            Self::Increasing
        } else {
            Self::Decreasing
        }
    }
}
