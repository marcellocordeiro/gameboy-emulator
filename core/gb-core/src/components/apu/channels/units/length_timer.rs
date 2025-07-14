pub struct LengthTimer {
    length: usize,

    enabled: bool,
    counter: usize,
}

impl LengthTimer {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            enabled: false,
            counter: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn expired(&self) -> bool {
        self.counter == 0
    }

    pub fn tick(&mut self) {
        if !self.enabled || self.counter == 0 {
            return;
        }

        self.counter -= 1;
    }

    pub fn trigger(&mut self) {
        self.counter = self.length;
    }

    pub fn set_counter(&mut self, counter: u8) {
        self.counter = self.length - (counter as usize);
    }

    pub fn write(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
