use crate::components::apu::channels::units::{Envelope, LengthTimer, PeriodDivider, WaveDuty};

/// Pulse channel 2 (`NR2x`)
pub struct Channel2 {
    enabled: bool,
    dac_enabled: bool,

    // Length timer and duty cycle
    wave_duty: WaveDuty,

    // Volume and envelope
    envelope: Envelope,

    // Period and control
    length_timer: LengthTimer,
    period_divider: PeriodDivider<fn(u16) -> u16>,
}

impl Default for Channel2 {
    fn default() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            wave_duty: WaveDuty::default(),
            envelope: Envelope::default(),
            length_timer: LengthTimer::new(64),
            period_divider: PeriodDivider::new(|x| (2048 - x) * 4),
        }
    }
}

impl Channel2 {
    pub fn tick(&mut self) {
        self.period_divider.tick();

        if self.period_divider.expired() {
            self.period_divider.reload();

            self.wave_duty.tick();
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

        let sample = self.wave_duty.wave_data() * self.envelope.volume();

        Some(sample)
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        if self.length_timer.expired() {
            self.length_timer.trigger();
        }

        self.period_divider.reload();
        self.envelope.trigger();
        self.wave_duty.reset_position();
    }

    // Read

    /// FF16 — NR21: Channel 2 length timer & duty cycle
    pub fn read_nr21(&self) -> u8 {
        let wave_duty_bits = self.wave_duty.read() << 6;
        wave_duty_bits | 0b0011_1111
    }

    /// FF17 — NR22: Channel 2 volume & envelope
    pub fn read_nr22(&self) -> u8 {
        self.envelope.read()
    }

    /// FF18 — NR23: Channel 2 period low (write-only)
    pub fn read_nr23(&self) -> u8 {
        0xFF
    }

    /// FF19 — NR24: Channel 2 period high & control
    pub fn read_nr24(&self) -> u8 {
        let length_enable_bits = (self.length_timer.enabled() as u8) << 6;
        length_enable_bits | 0b1011_1111
    }

    // Write

    /// FF16 — NR21: Channel 2 length timer & duty cycle
    pub fn write_nr21(&mut self, value: u8) {
        let initial_length_timer = value & 0b0011_1111;
        let wave_duty_bits = (value & 0b1100_0000) >> 6;

        self.length_timer.set_counter(initial_length_timer);
        self.wave_duty.write(wave_duty_bits);
    }

    /// FF17 — NR22: Channel 2 volume & envelope
    pub fn write_nr22(&mut self, value: u8) {
        let dac_enabled = (value & 0b1111_1000) != 0;

        self.envelope.write(value);
        self.dac_enabled = dac_enabled;

        if !dac_enabled {
            self.enabled = false;
        }
    }

    /// FF18 — NR23: Channel 2 period low (write-only)
    pub fn write_nr23(&mut self, value: u8) {
        self.period_divider.set_period_low(value);
    }

    /// FF19 — NR24: Channel 2 period high & control
    pub fn write_nr24(&mut self, value: u8) {
        let frequency_high = value & 0b0000_0111;
        let length_enable = (value & 0b0100_0000) != 0;
        let trigger = (value & 0b1000_0000) != 0;

        self.period_divider.set_period_high(frequency_high);
        self.length_timer.write(length_enable);

        if trigger && self.dac_enabled {
            self.trigger();
        }
    }
}
