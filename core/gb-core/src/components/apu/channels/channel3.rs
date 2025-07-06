use crate::components::apu::{frequency_timer::FrequencyTimer, length_timer::LengthTimer};

/// Wave channel (`NR3x`)
pub struct Channel3 {
    enabled: bool,

    //nr30: u8, // DAC enable
    dac_enabled: bool,

    //nr31: u8, // Length timer (write-only)
    // initial_length_timer: u8,

    //nr32: u8, // Output level
    current_volume: u8,

    //nr33: u8, // Period low (write-only)
    //nr34: u8, // Period high & control
    length_timer: LengthTimer,
    frequency_timer: FrequencyTimer<fn(u16) -> u16>,

    wave_pattern_ram: [u8; 16],
    wave_position: usize,
}

impl Default for Channel3 {
    fn default() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            // initial_length_timer: 0,
            current_volume: 0,
            length_timer: LengthTimer::new(256),
            frequency_timer: FrequencyTimer::new(|x| (2048 - x) * 2),
            wave_pattern_ram: Default::default(),
            wave_position: 0,
        }
    }
}

impl Channel3 {
    pub fn tick(&mut self) {
        self.frequency_timer.tick();

        if self.frequency_timer.expired() {
            self.frequency_timer.reload();

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
        self.enabled = false;
    }

    pub fn digital_output(&self) -> Option<u8> {
        if !self.dac_enabled {
            return None;
        }

        let both_nibbles = self.wave_pattern_ram[self.wave_position / 2];
        let nibble = if self.wave_position & 0b1 == 0 {
            both_nibbles >> 4
        } else {
            both_nibbles & 0b1111
        };

        let volume_shift = match self.current_volume {
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
        self.enabled = true;

        if self.length_timer.expired() {
            self.length_timer.reload();
        }

        self.wave_position = 0;

        self.frequency_timer.reload();
    }

    // Registers (read)

    /// FF1A — NR30: Channel 3 DAC enable
    pub fn read_nr30(&self) -> u8 {
        ((self.dac_enabled as u8) << 7) | 0b0111_1111
    }

    /// FF1B — NR31: Channel 3 length timer (write-only)
    pub fn read_nr31(&self) -> u8 {
        0xFF
    }

    /// FF1C — NR32: Channel 3 output level
    pub fn read_nr32(&self) -> u8 {
        (self.current_volume << 5) | 0b1001_1111
    }

    /// FF1D — NR33: Channel 3 period low (write-only)
    pub fn read_nr33(&self) -> u8 {
        0xFF
    }

    /// FF1E — NR34: Channel 3 period high & control
    pub fn read_nr34(&self) -> u8 {
        ((self.length_timer.enabled as u8) << 6) | 0b1011_1111
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
        self.current_volume = (value & 0b0110_0000) >> 5;
    }

    /// FF1D — NR33: Channel 3 period low (write-only)
    pub fn write_nr33(&mut self, value: u8) {
        self.frequency_timer.set_frequency_low(value);
    }

    /// FF1E — NR34: Channel 3 period high and control
    pub fn write_nr34(&mut self, value: u8) {
        let frequency_high = value & 0b0000_0111;
        let length_enable = (value & 0b0100_0000) != 0;
        let trigger = (value & 0b1000_0000) != 0;

        self.frequency_timer.set_frequency_high(frequency_high);
        self.length_timer.enabled = length_enable;

        if trigger {
            self.trigger();
        }
    }

    // Wave pattern RAM
    // TODO: obscure behaviour

    pub fn read_wave_pattern_ram(&self, address: u16) -> u8 {
        let address = (address - 0xFF30) as usize;
        self.wave_pattern_ram[address]
    }

    pub fn write_wave_pattern_ram(&mut self, address: u16, value: u8) {
        let address = (address - 0xFF30) as usize;
        self.wave_pattern_ram[address] = value;
    }
}
