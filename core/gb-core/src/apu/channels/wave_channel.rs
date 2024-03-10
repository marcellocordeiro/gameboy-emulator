#[derive(Default)]
/// NR3x
pub struct WaveChannel {
    wave_pattern_ram: [u8; 16],
}

impl WaveChannel {
    // Registers (read)

    /// FF1A — NR30: Channel 3 DAC enable
    pub fn read_nr30(&self) -> u8 {
        0x7F
    }

    /// FF1B — NR31: Channel 3 length timer [write-only]
    pub fn read_nr31(&self) -> u8 {
        0xFF
        // panic!("NR31 is write-only");
    }

    /// FF1C — NR32: Channel 3 output level
    pub fn read_nr32(&self) -> u8 {
        0x9F
    }

    /// FF1D — NR33: Channel 3 period low [write-only]
    pub fn read_nr33(&self) -> u8 {
        0xFF
        // panic!("NR33 is write-only");
    }

    /// FF1E — NR34: Channel 3 period high & control
    pub fn read_nr34(&self) -> u8 {
        0xBF
    }

    // Registers (write)

    /// FF1A — NR30: Channel 3 DAC enable
    pub fn write_nr30(&mut self, value: u8) {
        //
    }

    /// FF1B — NR31: Channel 3 length timer [write-only]
    pub fn write_nr31(&mut self, value: u8) {
        //
    }

    /// FF1C — NR32: Channel 3 output level
    pub fn write_nr32(&mut self, value: u8) {
        //
    }

    /// FF1D — NR33: Channel 3 period low [write-only]
    pub fn write_nr33(&mut self, value: u8) {
        //
    }

    /// FF1E — NR34: Channel 3 period high & control
    pub fn write_nr34(&mut self, value: u8) {
        //
    }

    // Wave pattern RAM

    pub fn read_wave_pattern_ram(&self, address: u16) -> u8 {
        let address = (address - 0xFF30) as usize;
        self.wave_pattern_ram[address]
    }

    pub fn write_wave_pattern_ram(&mut self, address: u16, value: u8) {
        let address = (address - 0xFF30) as usize;
        self.wave_pattern_ram[address] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        todo!();
    }

    pub fn write(&mut self, address: u16, value: u8) {}
}
