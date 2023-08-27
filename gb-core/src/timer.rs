#[derive(Debug, Default, PartialEq, Eq)]
enum TimaState {
    #[default]
    Running,
    Overflow(u8),
    Loading(u8),
}

#[derive(Debug, Default)]
pub struct Timer {
    system_counter: u16, // [15-8] Divider (R/W).
    tima: u8,            // Timer counter (R/W).
    tma: u8,             // Timer modulo (R/W).
    tac: u8,             // Timer Control (R/W).
    tima_state: TimaState,

    pub irq: bool,
}

impl Timer {
    // 0xFF04 ~ 0xFF07

    // 0xFF04: DIV
    // 0xFF05: TIMA
    // 0xFF06: TMA
    // 0xFF07: TAC

    pub fn skip_bootrom(&mut self) {
        self.system_counter = 0xABCC;
    }

    pub fn tick(&mut self) {
        self.update_tima_state();

        if !self.timer_enable() {
            self.increment_system_counter();
            return;
        }

        let clock = self.input_clock();

        let old_rate_bit = self.system_counter & (clock / 2);

        self.increment_system_counter();

        let new_rate_bit = self.system_counter & (clock / 2);

        if (old_rate_bit != 0) && (new_rate_bit == 0) {
            self.increment_tima();
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.read_div(),
            0xFF05 => self.read_tima(),
            0xFF06 => self.read_tma(),
            0xFF07 => self.read_tac(),

            _ => unreachable!("[timer.rs] Invalid read: {:#06x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.write_div(),
            0xFF05 => self.write_tima(value),
            0xFF06 => self.write_tma(value),
            0xFF07 => self.write_tac(value),

            _ => unreachable!(
                "[timer.rs] Invalid write: ({:#06x}) = {:#04x}",
                address, value
            ),
        }
    }

    fn read_div(&self) -> u8 {
        (self.system_counter >> 8) as u8
    }

    fn read_tima(&self) -> u8 {
        match self.tima_state {
            TimaState::Loading(_) => self.tma,
            _ => self.tima,
        }
    }

    fn read_tma(&self) -> u8 {
        self.tma
    }

    fn read_tac(&self) -> u8 {
        0b1111_1000 | self.tac
    }

    fn write_div(&mut self) {
        let clock = self.input_clock();
        let rate_bit = self.system_counter & (clock / 2) != 0;

        self.system_counter = 0;

        if self.timer_enable() && rate_bit {
            self.increment_tima();
        }
    }

    fn write_tima(&mut self, value: u8) {
        match self.tima_state {
            TimaState::Running => self.tima = value,
            TimaState::Overflow(_) => {
                self.tima = value;
                self.tima_state = TimaState::Running;
            }
            TimaState::Loading(_) => {}
        }
    }

    fn write_tma(&mut self, value: u8) {
        self.tma = value;
    }

    pub fn write_tac(&mut self, value: u8) {
        // https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register

        let old_clock = self.input_clock();
        let old_enable = self.timer_enable();

        self.tac = value;

        let new_clock = self.input_clock();
        let new_enable = self.timer_enable();

        if (old_enable && (self.system_counter & (old_clock / 2) != 0))
            && !(new_enable && (self.system_counter & (new_clock / 2) != 0))
        {
            self.increment_tima();
        }
    }

    // Other helpers.
    fn increment_system_counter(&mut self) {
        self.system_counter = self.system_counter.wrapping_add(1);
    }

    fn increment_tima(&mut self) {
        self.tima = self.tima.wrapping_add(1);

        if self.tima == 0 {
            self.tima_state = TimaState::Overflow(3);
        }
    }

    fn update_tima_state(&mut self) {
        match self.tima_state {
            TimaState::Running => {}

            // IRQ is delayed by 4 cycles.
            TimaState::Overflow(count) => match count {
                0 => {
                    self.irq = true;
                    self.tima_state = TimaState::Loading(3);
                }

                value => self.tima_state = TimaState::Overflow(value - 1),
            },

            // After an overflow and requesting an interruption,
            // the `tima = tma` load is delayed by 4 cycles.
            TimaState::Loading(count) => match count {
                0 => {
                    self.tima = self.tma;
                    self.tima_state = TimaState::Running;
                }

                value => self.tima_state = TimaState::Loading(value - 1),
            },
        }
    }

    fn timer_enable(&self) -> bool {
        (self.tac & 0b100) != 0
    }

    fn input_clock(&self) -> u16 {
        match self.tac & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,

            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let timer = Timer::default();

        assert_eq!(timer.system_counter, 0x0000);
        assert_eq!(timer.tima, 0x00);
        assert_eq!(timer.tma, 0x00);
        assert_eq!(timer.tac, 0x00);
        assert_eq!(timer.tima_state, TimaState::Running);
        assert_eq!(timer.irq, false);

        assert_eq!(timer.read_div(), 0x00);
        assert_eq!(timer.read_tima(), 0x00);
        assert_eq!(timer.read_tac(), 0xF8);
    }

    #[test]
    fn test_skip_bootrom_state() {
        let mut timer = Timer::default();

        timer.skip_bootrom();

        assert_eq!(timer.system_counter, 0xABCC);
        assert_eq!(timer.tima, 0x00);
        assert_eq!(timer.tma, 0x00);
        assert_eq!(timer.tac, 0x00);
        assert_eq!(timer.tima_state, TimaState::Running);
        assert_eq!(timer.irq, false);

        assert_eq!(timer.read_div(), 0xAB);
        assert_eq!(timer.read_tima(), 0x00);
        assert_eq!(timer.read_tac(), 0xF8);
    }

    // TODO: rewrite this.
    #[test]
    #[ignore]
    fn test_overflow() {
        let mut timer = Timer::default();

        assert_eq!(timer.tac, 0x00);
        assert_eq!(timer.timer_enable(), false);

        timer.write_tima(0xFF);
        timer.write_tac(0b100);
        timer.write_tma(0xB0);

        assert_eq!(timer.tac, 0b100);
        assert_eq!(timer.timer_enable(), true);
        assert_eq!(timer.tima, 0xFF);
        assert_eq!(timer.tima_state, TimaState::Running);

        for _ in 0..1023 {
            timer.tick();
        }

        assert_eq!(timer.system_counter, 1023);
        assert_eq!(timer.read_div(), 0b0000_0011);
        assert_eq!(timer.tac, 0b100);
        assert_eq!(timer.timer_enable(), true);
        assert_eq!(timer.tima, 0xFF);
        assert_eq!(timer.tima_state, TimaState::Running);

        timer.tick();

        assert_eq!(timer.tac, 0b100);
        assert_eq!(timer.timer_enable(), true);
        assert_eq!(timer.tima, 0);
        assert_eq!(timer.tima_state, TimaState::Overflow(3));

        timer.tick();
        timer.tick();
        timer.tick();

        assert_eq!(timer.tima_state, TimaState::Overflow(0));
        assert_eq!(timer.tima, 0);
        assert_eq!(timer.irq, false);

        timer.tick();

        assert_eq!(timer.tima_state, TimaState::Loading(3));
        assert_eq!(timer.irq, true);

        timer.tick();
        timer.tick();
        timer.tick();

        assert_eq!(timer.tima_state, TimaState::Loading(0));

        timer.tick();

        assert_eq!(timer.tima_state, TimaState::Running);
        assert_eq!(timer.tima, 0xB0);
    }

    // TODO: test overflows and interrupts
}
