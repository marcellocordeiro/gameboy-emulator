use bitflags::bitflags;

use self::channels::{Channel1, Channel2, Channel3, Channel4};
use crate::{
    components::apu::{frame_sequencer::FrameSequencer, high_pass_filter::HighPassFilter},
    constants::{CPU_CLOCK_RATE, DeviceModel},
    utils::macros::device_is_cgb,
};

pub const AUDIO_SAMPLE_RATE: usize = 44100;
pub const AUDIO_BUFFER_SIZE: usize = 4096;
const AUDIO_CYCLES_PER_SAMPLE: usize = CPU_CLOCK_RATE / AUDIO_SAMPLE_RATE; // 95

pub type StereoSample = [f32; 2];
pub type AudioBuffer = [f32; AUDIO_BUFFER_SIZE];
pub type Callback = dyn Fn(&[f32]);

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct Channels: u8 {
        const CH1 = 0b0001; // 0
        const CH2 = 0b0010; // 1
        const CH3 = 0b0100; // 2
        const CH4 = 0b1000; // 3
    }
}

#[allow(clippy::struct_excessive_bools)]
pub struct Apu {
    prev_system_div: u8,
    internal_cycles: usize,

    frame_sequencer: FrameSequencer,

    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,

    /// NR50: Master volume & VIN panning
    vin_left: bool,
    left_volume: u8,
    vin_right: bool,
    right_volume: u8,

    /// NR51: Sound panning
    left_panning: Channels,
    right_panning: Channels,

    /// From NR52: Audio master control
    audio_on: bool,

    device_model: DeviceModel,
    cgb_mode: bool,
    double_speed: bool,

    hpf_left: HighPassFilter,
    hpf_right: HighPassFilter,

    buffer: AudioBuffer,
    buffer_position: usize,
    callback: Option<Box<Callback>>,

    pub ui_channel_overrides: Channels,
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            prev_system_div: 0,
            internal_cycles: 0,
            frame_sequencer: FrameSequencer::default(),
            channel1: Channel1::default(),
            channel2: Channel2::default(),
            channel3: Channel3::default(),
            channel4: Channel4::default(),
            vin_left: false,
            left_volume: 0,
            vin_right: false,
            right_volume: 0,
            left_panning: Channels::empty(),
            right_panning: Channels::empty(),
            audio_on: false,
            cgb_mode: false,
            device_model: DeviceModel::default(),
            double_speed: false,
            hpf_left: HighPassFilter::default(),
            hpf_right: HighPassFilter::default(),
            buffer: [0.0; AUDIO_BUFFER_SIZE],
            buffer_position: 0,
            callback: None,
            ui_channel_overrides: Channels::all(),
        }
    }
}

impl Apu {
    // 0xFF10 ~ 0xFF3F

    // 0xFF10 ~ 0xFF26: Audio registers
    // 0xFF30 ~ 0xFF3F: Wave RAM

    // Undocumented
    // 0xFF76: PCM12
    // 0xFF77: PCM34

    #[must_use]
    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            cgb_mode: device_model.is_cgb(),
            device_model,
            ..Default::default()
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
    }

    pub fn set_double_speed(&mut self, value: bool) {
        self.double_speed = value;
    }

    pub fn skip_bootrom(&mut self) {
        self.channel1.write_nr10(0x80);
        self.channel1.write_nr11(0xBF);
        self.channel1.write_nr12(0xF3);
        self.channel1.write_nr13(0xFF);
        self.channel1.write_nr14(0xBF);

        self.channel2.write_nr21(0x3F);
        self.channel2.write_nr22(0x00);
        self.channel2.write_nr23(0xFF);
        self.channel2.write_nr24(0xBF);

        self.channel3.write_nr30(0x7F);
        self.channel3.write_nr31(0xFF);
        self.channel3.write_nr32(0x9F);
        self.channel3.write_nr33(0xFF);
        self.channel3.write_nr34(0xBF);

        self.channel4.write_nr41(0xFF);
        self.channel4.write_nr42(0x00);
        self.channel4.write_nr43(0x00);
        self.channel4.write_nr44(0xBF);

        self.write_nr50(0x77);
        self.write_nr51(0xF3);
        self.write_nr52(0xF1);
    }

    pub fn tick(&mut self, div: u8) {
        self.channel1.tick();
        self.channel2.tick();
        self.channel3.tick();
        self.channel4.tick();

        // DIV-APU
        if self.audio_on && self.falling_edge(self.prev_system_div, div) {
            match self.frame_sequencer.next_step() {
                0 => {
                    self.channel1.tick_length_timer();
                    self.channel2.tick_length_timer();
                    self.channel3.tick_length_timer();
                    self.channel4.tick_length_timer();
                }

                1 => {}

                2 => {
                    self.channel1.tick_sweep();

                    self.channel1.tick_length_timer();
                    self.channel2.tick_length_timer();
                    self.channel3.tick_length_timer();
                    self.channel4.tick_length_timer();
                }

                3 => {}

                4 => {
                    self.channel1.tick_length_timer();
                    self.channel2.tick_length_timer();
                    self.channel3.tick_length_timer();
                    self.channel4.tick_length_timer();
                }

                5 => {}

                6 => {
                    self.channel1.tick_sweep();

                    self.channel1.tick_length_timer();
                    self.channel2.tick_length_timer();
                    self.channel3.tick_length_timer();
                    self.channel4.tick_length_timer();
                }

                7 => {
                    self.channel1.tick_envelope();
                    self.channel2.tick_envelope();
                    self.channel4.tick_envelope();
                }

                _ => unreachable!(),
            }
        }

        self.prev_system_div = div;

        if self.internal_cycles >= AUDIO_CYCLES_PER_SAMPLE {
            self.internal_cycles = 0;
            self.push_sample();
        }
        self.internal_cycles += 1;
    }

    pub fn add_callback(&mut self, callback: Box<Callback>) {
        self.callback = Some(callback);
    }

    pub fn take_callback(&mut self) -> Option<Box<Callback>> {
        self.callback.take()
    }

    #[must_use]
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
            0xFF24 => self.read_nr50(),

            // FF25 — NR51: Sound panning
            0xFF25 => self.read_nr51(),

            // FF26 — NR52: Audio master control
            0xFF26 => self.read_nr52(),

            // Channel 4's wave pattern RAM
            0xFF30..=0xFF3F => self.channel3.read_wave_ram(address),

            // PCM12 (CGB Mode only): Digital outputs 1 & 2 (read-only)
            0xFF76 => {
                if !device_is_cgb!(self) {
                    return 0xFF;
                }

                let ch1_output = self.channel1.digital_output().unwrap_or(0);
                let ch2_output = self.channel2.digital_output().unwrap_or(0);

                (ch2_output << 4) | ch1_output
            }

            // PCM34 (CGB Mode only): Digital outputs 3 & 4 (read-only)
            0xFF77 => {
                if !device_is_cgb!(self) {
                    return 0xFF;
                }

                let ch3_output = self.channel3.digital_output().unwrap_or(0);
                let ch4_output = self.channel4.digital_output().unwrap_or(0);

                (ch4_output << 4) | ch3_output
            }

            _ => unreachable!("[apu.rs] Invalid read: {address:#06x}"),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // Writes to any register except NR52 are ignored when audio is off
        // Wave RAM is always accessible
        if !self.audio_on {
            match address {
                0xFF26 => (),
                0xFF30..=0xFF3F => (),

                _ => return,
            }
        }

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
            0xFF24 => self.write_nr50(value),

            // FF25 — NR51: Sound panning
            0xFF25 => self.write_nr51(value),

            // FF26 — NR52: Audio master control
            0xFF26 => self.write_nr52(value),

            // Channel 4's wave pattern RAM
            0xFF30..=0xFF3F => self.channel3.write_wave_ram(address, value),

            0xFF76 => (),
            0xFF77 => (),

            _ => unreachable!("[apu.rs] Invalid write: ({address:#06x}) = {value:#04x}"),
        }
    }

    fn read_nr50(&self) -> u8 {
        ((self.vin_left as u8) << 7)
            | (self.left_volume << 4)
            | ((self.vin_right as u8) << 3)
            | self.right_volume
    }

    fn write_nr50(&mut self, value: u8) {
        let vin_left = (value & 0b1000_0000) != 0;
        let left_volume = (value & 0b0111_0000) >> 4;
        let vin_right = (value & 0b0000_1000) != 0;
        let right_volume = value & 0b0000_0111;

        self.vin_left = vin_left;
        self.left_volume = left_volume;
        self.vin_right = vin_right;
        self.right_volume = right_volume;
    }

    fn read_nr51(&self) -> u8 {
        let left_bits = self.left_panning.bits();
        let right_bits = self.right_panning.bits();

        (left_bits << 4) | right_bits
    }

    fn write_nr51(&mut self, value: u8) {
        let left_bits = (value & 0b1111_0000) >> 4;
        let right_bits = value & 0b0000_1111;

        self.left_panning = Channels::from_bits_truncate(left_bits);
        self.right_panning = Channels::from_bits_truncate(right_bits);
    }

    /// FF26 — NR52: Audio master control
    fn read_nr52(&self) -> u8 {
        let audio_on = if self.audio_on { 1 << 7 } else { 0 };
        let ch4_on = if self.channel4.enabled() { 1 << 3 } else { 0 };
        let ch3_on = if self.channel3.enabled() { 1 << 2 } else { 0 };
        let ch2_on = if self.channel2.enabled() { 1 << 1 } else { 0 };
        let ch1_on = if self.channel1.enabled() { 1 << 0 } else { 0 };

        audio_on | ch4_on | ch3_on | ch2_on | ch1_on | 0b0111_0000
    }

    fn write_nr52(&mut self, value: u8) {
        // Everything else is ignored, only `audio_on` is writable
        let audio_on = (value & 0b1000_0000) != 0;
        self.audio_on = audio_on;

        if !audio_on {
            self.channel1.disable();
            self.channel2.disable();
            self.channel3.disable();
            self.channel4.disable();
            self.write_nr50(0);
            self.write_nr51(0);
        }
    }

    fn push_sample(&mut self) {
        let [left, right] = self.mix();

        // Implies audio is enabled. Otherwise, skip adding samples to the buffer.
        let Some(callback) = &self.callback else {
            return;
        };

        self.buffer[self.buffer_position] = left;
        self.buffer[self.buffer_position + 1] = right;
        self.buffer_position += 2;

        if self.buffer_position >= AUDIO_BUFFER_SIZE {
            callback(&self.buffer[0..self.buffer_position]);
            self.buffer_position = 0;
        }
    }

    fn falling_edge(&self, prev: u8, next: u8) -> bool {
        let mask = if self.double_speed { 1 << 5 } else { 1 << 4 };

        ((prev & mask) != 0) && ((next & mask) == 0)
    }

    // TODO: refactor this :')
    fn mix(&mut self) -> [f32; 2] {
        if !self.audio_on {
            return [0.0, 0.0];
        }

        let [left, right] = [
            (Channels::CH1, self.channel1.digital_output()),
            (Channels::CH2, self.channel2.digital_output()),
            (Channels::CH3, self.channel3.digital_output()),
            (Channels::CH4, self.channel4.digital_output()),
        ]
        .into_iter()
        // Remove channels disabled by the UI
        .filter(|(channel, _)| self.ui_channel_overrides.contains(*channel))
        // Remove disabled channels
        .filter_map(|(ch, sample)| sample.map(|sample| (ch, sample)))
        // Normalize
        .map(|(ch, sample)| (ch, ((sample as f32) / 7.5) - 1.0))
        // Stereo panning
        .map(|(channel, sample)| {
            let left = if self.left_panning.contains(channel) {
                sample
            } else {
                0.0
            };

            let right = if self.right_panning.contains(channel) {
                sample
            } else {
                0.0
            };

            [left, right]
        })
        // Accumulate
        .fold([0.0, 0.0], |acc, sample| {
            [acc[0] + sample[0], acc[1] + sample[1]]
        })
        // Average
        .map(|sample| sample / 4.0);

        // These registers should never completely mute the channel.
        let left_volume = (self.left_volume as f32 + 1.0) / 8.0;
        let right_volume = (self.right_volume as f32 + 1.0) / 8.0;

        [
            self.hpf_left.apply(left * left_volume),
            self.hpf_right.apply(right * right_volume),
        ]
    }
}

mod channels;
mod frame_sequencer;
mod high_pass_filter;
