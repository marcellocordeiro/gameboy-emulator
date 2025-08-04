use crate::components::apu::channels::units::period_divider::PeriodDivider;

#[derive(Debug, Default)]
pub struct Sweep {
    pace: u8,
    direction: Direction,
    individual_step: u8,

    enabled: bool,
    shadow_period: u16,
    counter: u8,
}

impl Sweep {
    pub fn tick(
        &mut self,
        channel_enabled: &mut bool,
        period_divider: &mut PeriodDivider<fn(u16) -> u16>,
    ) {
        if !self.enabled {
            return;
        }

        if self.counter == 0 {
            return;
        }

        self.counter -= 1;

        if self.expired() {
            self.reload();

            if self.pace == 0 {
                return;
            }

            let period = self.period();

            if period >= 0x7FF {
                *channel_enabled = false;
            } else if self.individual_step > 0 {
                // Set the period divider as well!
                period_divider.set_period(period);
                self.set_shadow_period(period);

                if self.period() > 0x7FF {
                    *channel_enabled = false;
                }
            }
        }
    }

    pub fn expired(&self) -> bool {
        self.counter == 0
    }

    pub fn reload(&mut self) {
        self.counter = if self.pace > 0 { self.pace } else { 8 };
    }

    pub fn trigger(
        &mut self,
        channel_enabled: &mut bool,
        period_divider: &PeriodDivider<fn(u16) -> u16>,
    ) {
        self.shadow_period = period_divider.period();
        self.enabled = self.pace > 0 || self.individual_step > 0;
        self.reload();

        // Should disable the channel
        *channel_enabled = !(self.individual_step > 0 && self.period() > 0x7FF);
    }

    pub fn set_shadow_period(&mut self, period: u16) {
        self.shadow_period = period;
    }

    pub fn period(&self) -> u16 {
        let delta = self.shadow_period >> self.individual_step;

        match self.direction {
            Direction::Increasing => self.shadow_period + delta,
            Direction::Decreasing => self.shadow_period - delta,
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
