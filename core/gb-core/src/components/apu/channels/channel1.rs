use crate::components::apu::{
    envelope::Envelope,
    frequency_timer::FrequencyTimer,
    length_timer::LengthTimer,
    sweep::Sweep,
    wave_duty::WaveDuty,
};

/// Pulse channel 1 (`NR1x`)
pub struct Channel1 {
    enabled: bool,
    dac_enabled: bool,

    // Sweep
    sweep: Sweep,

    // Length timer and duty cycle
    wave_duty: WaveDuty,

    // Volume and envelope
    envelope: Envelope,

    // Period and control
    length_timer: LengthTimer,
    frequency_timer: FrequencyTimer<fn(u16) -> u16>,
}

impl Default for Channel1 {
    fn default() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            sweep: Sweep::default(),
            wave_duty: WaveDuty::default(),
            envelope: Envelope::default(),
            length_timer: LengthTimer::new(64),
            frequency_timer: FrequencyTimer::new(|x| (2048 - x) * 4),
        }
    }
}

impl Channel1 {
    pub fn tick(&mut self) {
        self.frequency_timer.tick();

        if self.frequency_timer.expired() {
            self.frequency_timer.reload();

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

    pub fn tick_sweep(&mut self) {
        if self.sweep.enabled() {
            self.sweep.tick();

            if self.sweep.expired() {
                self.sweep.reload();

                let current_frequency = self.frequency_timer.frequency();

                let (new_frequency, should_disable_channel) =
                    self.sweep.get_new_frequency(current_frequency);

                if should_disable_channel {
                    self.enabled = false;
                }

                self.frequency_timer.set_frequency(new_frequency);
            }
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.envelope.reset();
        self.length_timer.reload();
        self.frequency_timer.reload();
        self.sweep.reload();
    }

    pub fn digital_output(&self) -> Option<u8> {
        if !self.enabled {
            return None;
        }

        let sample = self.wave_duty.wave_data() * self.envelope.current_volume();

        Some(sample)
    }

    fn trigger(&mut self) {
        self.enabled = true;

        if self.length_timer.expired() {
            self.length_timer.reload();
        }

        self.frequency_timer.reload();
        self.envelope.reset();
        self.sweep.reload();
    }

    /// FF10 — NR10: Channel 1 sweep
    pub fn read_nr10(&self) -> u8 {
        self.sweep.read()
    }

    /// FF11 — NR11: Channel 1 length timer & duty cycle
    pub fn read_nr11(&self) -> u8 {
        let wave_duty_bits = self.wave_duty.read();

        (wave_duty_bits << 6) | 0b0011_1111
    }

    /// FF12 — NR12: Channel 1 volume & envelope
    pub fn read_nr12(&self) -> u8 {
        self.envelope.read()
    }

    /// FF13 — NR13: Channel 1 period low (write-only)
    pub fn read_nr13(&self) -> u8 {
        0xFF
    }

    /// FF14 — NR14: Channel 1 period high & control
    pub fn read_nr14(&self) -> u8 {
        ((self.length_timer.enabled as u8) << 6) | 0b1011_1111
    }

    /// FF10 — NR10: Channel 1 sweep
    pub fn write_nr10(&mut self, value: u8) {
        self.sweep.write(value);
    }

    /// FF11 — NR11: Channel 1 length timer & duty cycle
    pub fn write_nr11(&mut self, value: u8) {
        let initial_length_timer = value & 0b0011_1111;
        let wave_duty_bits = (value & 0b1100_0000) >> 6;

        self.length_timer.set_counter(initial_length_timer);
        self.wave_duty.write(wave_duty_bits);
    }

    /// FF12 — NR12: Channel 1 volume & envelope
    pub fn write_nr12(&mut self, value: u8) {
        let dac_enabled = (value & 0b1111_1000) != 0;

        self.envelope.write(value);
        self.dac_enabled = dac_enabled;

        if !dac_enabled {
            self.enabled = false;
        }
    }

    /// FF13 — NR13: Channel 1 period low (write-only)
    pub fn write_nr13(&mut self, value: u8) {
        self.frequency_timer.set_frequency_low(value);
    }

    /// FF14 — NR14: Channel 1 period high & control
    pub fn write_nr14(&mut self, value: u8) {
        let frequency_high = value & 0b0000_0111;
        let length_enable = (value & 0b0100_0000) != 0;
        let trigger = (value & 0b1000_0000) != 0;

        self.frequency_timer.set_frequency_high(frequency_high);
        self.length_timer.enabled = length_enable;

        if trigger {
            self.trigger();
        }
    }
}
