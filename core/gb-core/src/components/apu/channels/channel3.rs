use crate::components::apu::channels::units::{LengthTimer, PeriodDivider};

/// Wave channel (`NR3x`)
pub struct Channel3 {
    enabled: bool,

    // DAC enable
    dac_enabled: bool,

    // Output level
    output_level: u8,

    // Period high and control
    length_timer: LengthTimer,
    period_divider: PeriodDivider<fn(u16) -> u16>,

    wave_ram: [u8; 16],
    wave_position: usize,
}

impl Default for Channel3 {
    fn default() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            output_level: 0,
            length_timer: LengthTimer::new(256),
            period_divider: PeriodDivider::new(|x| (2048 - x) * 2),
            wave_ram: Default::default(),
            wave_position: 0,
        }
    }
}

impl Channel3 {
    pub fn tick(&mut self) {
        self.period_divider.tick();

        if self.period_divider.expired() {
            self.period_divider.reload();

            self.wave_position = (self.wave_position + 1) % 32;
        }
    }

    pub fn tick_length_timer(&mut self) {
        self.length_timer.tick();
        if self.length_timer.expired() {
            self.enabled = false;
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn disable(&mut self) {
        let wave_ram = self.wave_ram;

        *self = Self::default();
        self.wave_ram = wave_ram;
    }

    pub fn digital_output(&self) -> Option<u8> {
        if !self.dac_enabled || !self.enabled {
            return None;
        }

        let both_nibbles = self.wave_ram[self.wave_position / 2];
        let nibble = if self.wave_position & 0b1 == 0 {
            both_nibbles >> 4
        } else {
            both_nibbles & 0b1111
        };

        let volume_shift = match self.output_level {
            0b00 => 4, // Muted
            0b01 => 0, // 100%
            0b10 => 1, // 50%
            0b11 => 2, // 25%
            _ => unreachable!(),
        };

        let sample = nibble >> volume_shift;

        Some(sample)
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        if self.length_timer.expired() {
            self.length_timer.trigger();
        }

        self.wave_position = 0;

        self.period_divider.reload();
    }

    // Registers (read)

    /// FF1A — NR30: Channel 3 DAC enable
    pub fn read_nr30(&self) -> u8 {
        let dac_enabled_bits = (self.dac_enabled as u8) << 7;
        dac_enabled_bits | 0b0111_1111
    }

    /// FF1B — NR31: Channel 3 length timer (write-only)
    pub fn read_nr31(&self) -> u8 {
        0xFF
    }

    /// FF1C — NR32: Channel 3 output level
    pub fn read_nr32(&self) -> u8 {
        let output_level_bits = self.output_level << 5;
        output_level_bits | 0b1001_1111
    }

    /// FF1D — NR33: Channel 3 period low (write-only)
    pub fn read_nr33(&self) -> u8 {
        0xFF
    }

    /// FF1E — NR34: Channel 3 period high and control
    pub fn read_nr34(&self) -> u8 {
        let length_enable_bits = (self.length_timer.enabled() as u8) << 6;
        length_enable_bits | 0b1011_1111
    }

    // Registers (write)

    /// FF1A — NR30: Channel 3 DAC enable
    pub fn write_nr30(&mut self, value: u8) {
        let dac_enabled = (value & 0b1000_0000) != 0;
        self.dac_enabled = dac_enabled;

        if !dac_enabled {
            self.enabled = false;
        }
    }

    /// FF1B — NR31: Channel 3 length timer (write-only)
    pub fn write_nr31(&mut self, value: u8) {
        self.length_timer.set_counter(value);
    }

    /// FF1C — NR32: Channel 3 output level
    pub fn write_nr32(&mut self, value: u8) {
        let output_level = (value & 0b0110_0000) >> 5;
        self.output_level = output_level;
    }

    /// FF1D — NR33: Channel 3 period low (write-only)
    pub fn write_nr33(&mut self, value: u8) {
        self.period_divider.set_period_low(value);
    }

    /// FF1E — NR34: Channel 3 period high and control
    pub fn write_nr34(&mut self, value: u8) {
        let frequency_high = value & 0b0000_0111;
        let length_enable = (value & 0b0100_0000) != 0;
        let trigger = (value & 0b1000_0000) != 0;

        self.period_divider.set_period_high(frequency_high);
        self.length_timer.write(length_enable);

        if trigger && self.dac_enabled {
            self.trigger();
        }
    }

    // Wave RAM
    // TODO: obscure behaviour

    pub fn read_wave_ram(&self, address: u16) -> u8 {
        let address = (address - 0xFF30) as usize;
        self.wave_ram[address]
    }

    pub fn write_wave_ram(&mut self, address: u16, value: u8) {
        let address = (address - 0xFF30) as usize;
        self.wave_ram[address] = value;
    }
}
