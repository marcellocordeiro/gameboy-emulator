#[derive(Default)]
/// NR2x
pub struct PulseChannel2;

impl PulseChannel2 {
    // Read

    /// FF16 — NR21: Channel 2 length timer & duty cycle
    pub fn read_nr21(&self) -> u8 {
        0x3F
    }

    /// FF17 — NR22: Channel 2 volume & envelope
    pub fn read_nr22(&self) -> u8 {
        0x00
    }

    /// FF18 — NR23: Channel 2 period low [write-only]
    pub fn read_nr23(&self) -> u8 {
        0xFF
        // panic!("NR23 is write-only");
    }

    /// FF19 — NR24: Channel 2 period high & control
    pub fn read_nr24(&self) -> u8 {
        0xBF
    }

    // Write

    /// FF16 — NR21: Channel 2 length timer & duty cycle
    pub fn write_nr21(&mut self, value: u8) {
        //
    }

    /// FF17 — NR22: Channel 2 volume & envelope
    pub fn write_nr22(&mut self, value: u8) {
        //
    }

    /// FF18 — NR23: Channel 2 period low [write-only]
    pub fn write_nr23(&mut self, value: u8) {
        //
    }

    /// FF19 — NR24: Channel 2 period high & control
    pub fn write_nr24(&mut self, value: u8) {
        //
    }
}
