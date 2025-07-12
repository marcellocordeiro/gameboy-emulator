pub struct LengthTimer {
    pub enabled: bool,
    length: usize,
    counter: usize,
}

impl LengthTimer {
    pub fn new(length: usize) -> Self {
        Self {
            enabled: false,
            counter: 0,
            length,
        }
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

    pub fn reload(&mut self) {
        self.counter = self.length;
    }

    pub fn set_counter(&mut self, counter: u8) {
        self.counter = self.length - (counter as usize);
    }
}
