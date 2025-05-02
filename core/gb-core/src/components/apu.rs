// TODO: everything

#![allow(dead_code, unused_variables)]

use self::channels::{NoiseChannel, PulseChannel1, PulseChannel2, WaveChannel};
use crate::{DeviceModel, utils::macros::device_is_cgb};

#[derive(Default)]
pub struct Apu {
    channel1: PulseChannel1,
    channel2: PulseChannel2,
    channel3: WaveChannel,
    channel4: NoiseChannel,

    master_enable: bool,

    cgb_mode: bool,

    device_model: DeviceModel,

    callback: Option<Box<Callback>>,
}

pub type Callback = dyn Fn(&[f32]);

impl Apu {
    // 0xFF10 ~ 0xFF3F

    // 0xFF10..=0xFF26 => 0, // TODO: Sound.
    // 0xFF30..=0xFF3F => 0, // TODO: Wave pattern.

    // Undocumented
    // 0xFF76
    // 0xFF77

    fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            device_model,
            ..Default::default()
        }
    }

    fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    pub fn skip_bootrom(&mut self) {
        self.master_enable = true;
        self.channel1.enabled = true;
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn tick(&mut self, _div: u8) {}

    pub fn add_callback(&mut self, callback: Box<Callback>) {
        self.callback = Some(callback);
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // Channel 1
            0xFF10 => self.channel1.read_nr10(),
            0xFF11 => self.channel1.read_nr11(),
            0xFF12 => self.channel1.read_nr12(),
            0xFF13 => self.channel1.read_nr13(),
            0xFF14 => self.channel1.read_nr14(),

            // Channel 2
            0xFF16 => self.channel2.read_nr21(),
            0xFF17 => self.channel2.read_nr22(),
            0xFF18 => self.channel2.read_nr23(),
            0xFF19 => self.channel2.read_nr24(),

            // Channel 3
            0xFF1A => self.channel3.read_nr30(),
            0xFF1B => self.channel3.read_nr31(),
            0xFF1C => self.channel3.read_nr32(),
            0xFF1D => self.channel3.read_nr33(),
            0xFF1E => self.channel3.read_nr34(),

            // Channel 4
            0xFF20 => self.channel4.read_nr41(),
            0xFF21 => self.channel4.read_nr42(),
            0xFF22 => self.channel4.read_nr43(),
            0xFF23 => self.channel4.read_nr44(),

            // FF24 — NR50: Master volume & VIN panning
            0xFF24 => self.read_nr50(), // 0x77,

            // FF25 — NR51: Sound panning
            0xFF25 => self.read_nr51(),

            // FF26 — NR52: Audio master control
            0xFF26 => self.read_nr52(),

            // Channel 4's wave pattern RAM
            0xFF30..=0xFF3F => self.channel3.read_wave_pattern_ram(address),

            0xFF76 => {
                if !device_is_cgb!(self) {
                    return 0xFF;
                }

                0x00
            }

            0xFF77 => {
                if !device_is_cgb!(self) {
                    return 0xFF;
                }

                0x00
            }

            _ => unreachable!("[apu.rs] Invalid read: {address:#06x}"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // Channel 1
            0xFF10 => self.channel1.write_nr10(value),
            0xFF11 => self.channel1.write_nr11(value),
            0xFF12 => self.channel1.write_nr12(value),
            0xFF13 => self.channel1.write_nr13(value),
            0xFF14 => self.channel1.write_nr14(value),

            // Channel 2
            0xFF16 => self.channel2.write_nr21(value),
            0xFF17 => self.channel2.write_nr22(value),
            0xFF18 => self.channel2.write_nr23(value),
            0xFF19 => self.channel2.write_nr24(value),

            // Channel 3
            0xFF1A => self.channel3.write_nr30(value),
            0xFF1B => self.channel3.write_nr31(value),
            0xFF1C => self.channel3.write_nr32(value),
            0xFF1D => self.channel3.write_nr33(value),
            0xFF1E => self.channel3.write_nr34(value),

            // Channel 4
            0xFF20 => self.channel4.write_nr41(value),
            0xFF21 => self.channel4.write_nr42(value),
            0xFF22 => self.channel4.write_nr43(value),
            0xFF23 => self.channel4.write_nr44(value),

            // FF24 — NR50: Master volume & VIN panning
            0xFF24 => (),

            // FF25 — NR51: Sound panning
            0xFF25 => (),

            // FF26 — NR52: Audio master control
            0xFF26 => (),

            // Channel 4's wave pattern RAM
            0xFF30..=0xFF3F => self.channel3.write_wave_pattern_ram(address, value),

            0xFF76 => if !device_is_cgb!(self) {},

            0xFF77 => if !device_is_cgb!(self) {},

            _ => unreachable!("[apu.rs] Invalid write: ({address:#06x}) = {value:#04x}"),
        }
    }

    fn read_nr50(&self) -> u8 {
        0x77
    }

    fn read_nr51(&self) -> u8 {
        0xF3
    }

    /// FF26 — NR52: Audio master control
    fn read_nr52(&self) -> u8 {
        const MASK: u8 = 0b0111_0000;
        // 0b11110001
        // 0xF1

        let audio_on = if self.master_enable { 1 << 7 } else { 0 };
        let ch4_on = if self.channel4.enabled { 1 << 3 } else { 0 };
        let ch3_on = if self.channel3.enabled { 1 << 2 } else { 0 };
        let ch2_on = if self.channel2.enabled { 1 << 1 } else { 0 };
        let ch1_on = if self.channel1.enabled { 1 << 0 } else { 0 };

        audio_on | MASK | ch4_on | ch3_on | ch2_on | ch1_on
    }
}

mod channels;
