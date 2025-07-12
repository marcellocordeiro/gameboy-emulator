#[derive(Debug, Default)]
pub struct PeriodDivider<T: Fn(u16) -> u16> {
    reload_period: u16,
    get_next_counter: T,

    current_period: u16,
    counter: u16,
}

impl<T: Fn(u16) -> u16> PeriodDivider<T> {
    pub fn new(get_next_counter: T) -> Self {
        Self {
            reload_period: 0,
            get_next_counter,
            current_period: 0,
            counter: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.counter == 0 {
            return;
        }

        self.counter -= 1;
    }

    pub fn reload(&mut self) {
        self.current_period = self.reload_period;
        self.counter = (self.get_next_counter)(self.current_period);
    }

    pub fn period(&self) -> u16 {
        self.reload_period
    }

    pub fn set_period(&mut self, value: u16) {
        self.reload_period = value;
    }

    pub fn set_period_low(&mut self, value: u8) {
        const MASK: u16 = 0xFF;
        let value = value as u16;

        self.reload_period &= !MASK;
        self.reload_period |= value;
    }

    pub fn set_period_high(&mut self, value: u8) {
        const MASK: u16 = 0b111;
        let value = ((value as u16) & MASK) << 8;

        self.reload_period &= !(MASK << 8);
        self.reload_period |= value;
    }

    pub fn expired(&self) -> bool {
        self.counter == 0
    }
}
