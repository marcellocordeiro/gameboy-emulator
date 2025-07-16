use super::units::{Envelope, LengthTimer, PeriodDivider};

/// Noise channel (`NR4x`)
pub struct Channel4 {
    enabled: bool,
    dac_enabled: bool,

    // Volume and envelope
    envelope: Envelope,

    // Frequency and randomness
    clock_shift: u8,
    /// AKA LFSR width. 0 => 15 bits, 1 => 7 bits
    short_width_mode: bool,
    clock_divider_code: u8,
    lfsr: u16,

    // Period and control
    length_timer: LengthTimer,
    period_divider: PeriodDivider<fn(u16) -> u16>,
}

impl Default for Channel4 {
    fn default() -> Self {
        Self {
            enabled: false,
            clock_shift: 0,
            short_width_mode: false,
            clock_divider_code: 0,
            lfsr: 0,
            envelope: Envelope::default(),
            dac_enabled: false,
            length_timer: LengthTimer::new(64),
            period_divider: PeriodDivider::new(|x| x),
        }
    }
}

impl Channel4 {
    pub fn tick(&mut self) {
        self.period_divider.tick();

        if self.period_divider.expired() {
            let new_frequency = self.clock_divider() << self.clock_shift;
            self.period_divider.set_period(new_frequency);
            self.period_divider.reload();

            let shifted_lfsr = self.lfsr >> 1;
            let xor_result = (self.lfsr & 0b1) ^ (shifted_lfsr & 0b1);

            self.lfsr = (xor_result << 14) | shifted_lfsr;

            if self.short_width_mode {
                self.lfsr &= !(1 << 6);
                self.lfsr |= xor_result << 6;
            }
        }
    }

    pub fn tick_length_timer(&mut self) {
        self.length_timer.tick();
        if self.length_timer.expired() {
            self.enabled = false;
        }
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick();
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn disable(&mut self) {
        *self = Self::default();
    }

    pub fn digital_output(&self) -> Option<u8> {
        if !self.dac_enabled || !self.enabled {
            return None;
        }

        let sample = (((self.lfsr & 1) ^ 1) as u8) * self.envelope.volume();

        Some(sample)
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        if self.length_timer.expired() {
            self.length_timer.trigger();
        }

        self.envelope.trigger();

        let new_frequency = self.clock_divider() << self.clock_shift;
        self.period_divider.set_period(new_frequency);
        self.period_divider.reload();

        self.lfsr = u16::MAX;
    }

    // Read

    /// FF20 — NR41: Channel 4 length timer (write-only)
    pub fn read_nr41(&self) -> u8 {
        0xFF
    }

    /// FF21 — NR42: Channel 4 volume and envelope
    pub fn read_nr42(&self) -> u8 {
        self.envelope.read()
    }

    /// FF22 — NR43: Channel 4 frequency and randomness
    pub fn read_nr43(&self) -> u8 {
        let short_width_mode_bit = self.short_width_mode as u8;

        (self.clock_shift << 4) | (short_width_mode_bit << 3) | self.clock_divider_code
    }

    /// FF23 — NR44: Channel 4 control
    pub fn read_nr44(&self) -> u8 {
        let length_enable_bits = (self.length_timer.enabled() as u8) << 6;
        length_enable_bits | 0b1011_1111
    }

    // Write

    /// FF20 — NR41: Channel 4 length timer (write-only)
    pub fn write_nr41(&mut self, value: u8) {
        let initial_length_timer = value & 0b0011_1111;
        self.length_timer.set_counter(initial_length_timer);
    }

    /// FF21 — NR42: Channel 4 volume and envelope
    pub fn write_nr42(&mut self, value: u8) {
        let dac_enabled = (value & 0b1111_1000) != 0;

        self.envelope.write(value);
        self.dac_enabled = dac_enabled;

        if !dac_enabled {
            self.enabled = false;
        }
    }

    /// FF22 — NR43: Channel 4 frequency and randomness
    pub fn write_nr43(&mut self, value: u8) {
        let clock_shift = (value & 0b1111_0000) >> 4;
        let lfsr_width_bit = (value & 0b0000_1000) != 0;
        let clock_divider = value & 0b0000_0111;

        self.clock_shift = clock_shift;
        self.short_width_mode = lfsr_width_bit;
        self.clock_divider_code = clock_divider;
    }

    /// FF23 — NR44: Channel 4 control
    pub fn write_nr44(&mut self, value: u8) {
        let length_enable = (value & 0b0100_0000) != 0;
        let trigger = (value & 0b1000_0000) != 0;

        self.length_timer.write(length_enable);

        if trigger && self.dac_enabled {
            self.trigger();
        }
    }

    fn clock_divider(&self) -> u16 {
        if self.clock_divider_code == 0 {
            8
        } else {
            (self.clock_divider_code as u16) << 4
        }
    }
}
