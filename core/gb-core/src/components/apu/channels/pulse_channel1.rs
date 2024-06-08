#![allow(dead_code, unused_variables)]

#[derive(Default)]
/// `NR1x`
pub struct PulseChannel1 {
    pub enabled: bool,
}

impl PulseChannel1 {
    /// FF10 — NR10: Channel 1 sweep
    pub fn read_nr10(&self) -> u8 {
        0x80
    }

    /// FF11 — NR11: Channel 1 length timer & duty cycle
    pub fn read_nr11(&self) -> u8 {
        0xBF
    }

    /// FF12 — NR12: Channel 1 volume & envelope
    pub fn read_nr12(&self) -> u8 {
        0xF3
    }

    /// FF13 — NR13: Channel 1 period low [write-only]
    pub fn read_nr13(&self) -> u8 {
        0xFF
        // panic!("NR13 is write-only");
    }

    /// FF14 — NR14: Channel 1 period high & control
    pub fn read_nr14(&self) -> u8 {
        0xBF
    }

    /// FF10 — NR10: Channel 1 sweep
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr10(&mut self, value: u8) {
        //
    }

    /// FF11 — NR11: Channel 1 length timer & duty cycle
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr11(&mut self, value: u8) {
        //
    }

    /// FF12 — NR12: Channel 1 volume & envelope
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr12(&mut self, value: u8) {
        //
    }

    /// FF13 — NR13: Channel 1 period low [write-only]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr13(&mut self, value: u8) {
        //
    }

    /// FF14 — NR14: Channel 1 period high & control
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr14(&mut self, value: u8) {
        //
    }
}
