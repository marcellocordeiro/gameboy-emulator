#![allow(dead_code, unused_variables)]

#[derive(Default)]
/// `NR4x`
pub struct NoiseChannel {
    pub enabled: bool,
}

impl NoiseChannel {
    // Read

    /// FF20 — NR41: Channel 4 length timer [write-only]
    pub fn read_nr41(&self) -> u8 {
        0xFF
        // panic!("NR41 is write-only");
    }

    /// FF21 — NR42: Channel 4 volume & envelope
    pub fn read_nr42(&self) -> u8 {
        0x00
    }

    /// FF22 — NR43: Channel 4 frequency & randomness
    pub fn read_nr43(&self) -> u8 {
        0x00
    }

    /// FF23 — NR44: Channel 4 control
    pub fn read_nr44(&self) -> u8 {
        0xBF
    }

    // Write

    /// FF20 — NR41: Channel 4 length timer [write-only]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr41(&mut self, value: u8) {
        //
    }

    /// FF21 — NR42: Channel 4 volume & envelope
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr42(&mut self, value: u8) {
        //
    }

    /// FF22 — NR43: Channel 4 frequency & randomness
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr43(&mut self, value: u8) {
        //
    }

    /// FF23 — NR44: Channel 4 control
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_nr44(&mut self, value: u8) {
        //
    }
}
