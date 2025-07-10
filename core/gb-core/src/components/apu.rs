use self::channels::{Channel1, Channel2, Channel3, Channel4};
use crate::{
    components::apu::frame_sequencer::FrameSequencer,
    constants::{CPU_CLOCK_RATE, DeviceModel},
    utils::macros::device_is_cgb,
};

pub const AUDIO_SAMPLE_RATE: usize = 44100;
pub const AUDIO_BUFFER_SIZE: usize = 512;
const AUDIO_CYCLES_PER_SAMPLE: usize = CPU_CLOCK_RATE / AUDIO_SAMPLE_RATE; // 95

pub type StereoSample = [f32; 2];
pub type Callback = dyn Fn(&[StereoSample]);

pub struct Apu {
    prev_system_div: u8,
    internal_cycles: usize,

    frame_sequencer: FrameSequencer,

    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,

    /// NR50: Master volume & VIN panning
    nr50: u8,

    /// NR51: Sound panning
    nr51: u8,

    /// From NR52: Audio master control
    audio_on: bool,

    device_model: DeviceModel,
    cgb_mode: bool,
    double_speed: bool,

    capacitor: f32,

    buffer: [StereoSample; AUDIO_BUFFER_SIZE],
    buffer_position: usize,
    callback: Option<Box<Callback>>,
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
            nr50: 0,
            nr51: 0,
            audio_on: false,
            device_model: DeviceModel::default(),
            cgb_mode: false,
            double_speed: false,
            capacitor: 0.0,
            buffer: [[0.0, 0.0]; AUDIO_BUFFER_SIZE],
            buffer_position: 0,
            callback: None,
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

    pub fn with_device_model(device_model: DeviceModel) -> Self {
        Self {
            device_model,
            ..Default::default()
        }
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        self.cgb_mode = value;
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
                    self.channel1.tick_length_timer();
                    self.channel2.tick_length_timer();
                    self.channel3.tick_length_timer();
                    self.channel4.tick_length_timer();

                    self.channel1.tick_sweep();
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
                    self.channel1.tick_length_timer();
                    self.channel2.tick_length_timer();
                    self.channel3.tick_length_timer();
                    self.channel4.tick_length_timer();

                    self.channel1.tick_sweep();
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
            self.mix();
        }
        self.internal_cycles += 1;
    }

    pub fn add_callback(&mut self, callback: Box<Callback>) {
        self.callback = Some(callback);
    }

    pub fn take_callback(&mut self) -> Option<Box<Callback>> {
        self.callback.take()
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
        self.nr50
    }

    fn write_nr50(&mut self, value: u8) {
        self.nr50 = value;
    }

    fn read_nr51(&self) -> u8 {
        self.nr51
    }

    fn write_nr51(&mut self, value: u8) {
        self.nr51 = value;
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
            self.nr50 = 0;
            self.nr51 = 0;
        }
    }

    fn mix(&mut self) {
        let channel1_sample = self.channel1.digital_output();
        let channel2_sample = self.channel2.digital_output();
        let channel3_sample = self.channel3.digital_output();
        let channel4_sample = self.channel4.digital_output();

        let mixed_sample = [
            channel1_sample,
            channel2_sample,
            channel3_sample,
            channel4_sample,
        ]
        .into_iter()
        .flatten()
        .map(|sample| ((sample as f32) / 7.5) - 1.0)
        .sum::<f32>()
            / 4.0;

        let mixed_sample = self.with_high_pass_filter(mixed_sample);

        // Implies audio is enabled. Otherwise, skip adding samples to the buffer.
        let Some(callback) = &self.callback else {
            return;
        };

        self.buffer[self.buffer_position] = [mixed_sample, mixed_sample];
        self.buffer_position += 1;

        if self.buffer_position >= AUDIO_BUFFER_SIZE {
            callback(self.buffer.as_ref());
            self.buffer_position = 0;
        }
    }

    fn falling_edge(&self, prev: u8, next: u8) -> bool {
        let mask = if self.double_speed { 1 << 5 } else { 1 << 4 };

        ((prev & mask) != 0) && ((next & mask) == 0)
    }

    // https://gbdev.io/pandocs/Audio_details.html#obscure-behavior
    fn with_high_pass_filter(&mut self, in_sample: f32) -> f32 {
        // 0.999958 ^ (4194304 / AUDIO_SAMPLE_RATE)
        const FACTOR: f32 = 0.996; // At 44.1kHz

        if !self.audio_on {
            return 0.0;
        }

        let out = in_sample - self.capacitor;
        self.capacitor = out.mul_add(-FACTOR, in_sample); // in_sample - (out * FACTOR)

        out
    }
}

mod channels;
mod envelope;
mod frame_sequencer;
mod frequency_timer;
mod length_timer;
mod sweep;
mod wave_duty;
