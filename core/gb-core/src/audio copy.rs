// TODO: everything

use crate::{utils::macros::device_is_cgb, DeviceConfig, OptionalCgbComponent};

#[derive(Default)]
pub struct Audio {
    device_config: DeviceConfig,
}

impl OptionalCgbComponent for Audio {
    fn with_device_config(device_config: DeviceConfig) -> Self {
        Self { device_config }
    }

    fn set_device_config(&mut self, device_config: DeviceConfig) {
        self.device_config = device_config;
    }
}

impl Audio {
    // 0xFF10 ~ 0xFF3F

    // 0xFF10..=0xFF26 => 0, // TODO: Sound.
    // 0xFF30..=0xFF3F => 0, // TODO: Wave pattern.

    // Undocumented
    // 0xFF76
    // 0xFF77

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // TODO: stubs for the DMG.

            // Channel 1

            // FF10 — NR10: Channel 1 sweep
            0xFF10 => 0x80,

            // FF11 — NR11: Channel 1 length timer & duty cycle
            0xFF11 => 0xBF,

            // FF12 — NR12: Channel 1 volume & envelope
            0xFF12 => 0xF3,

            // FF13 — NR13: Channel 1 period low [write-only]
            // 0xFF13 => panic!("NR13 is write-only"),

            // FF14 — NR14: Channel 1 period high & control
            0xFF14 => 0xBF,

            // Channel 2

            // FF16 — NR21: Channel 2 length timer & duty cycle
            0xFF16 => 0x3F,

            // FF12 — NR22: Channel 2 volume & envelope
            0xFF17 => 0x00,

            // FF18 — NR23: Channel 2 period low [write-only]
            // 0xFF18 => panic!("NR23 is write-only"),

            // FF19 — NR24: Channel 2 period high & control
            0xFF19 => 0xBF,

            // Channel 3

            // FF1A — NR30: Channel 3 DAC enable
            0xFF1A => 0x7F,

            // FF1B — NR31: Channel 3 length timer [write-only]
            // 0xFF1B => panic!("NR31 is write-only"),

            // FF1C — NR32: Channel 3 output level
            0xFF1C => 0x9F,

            // FF1D — NR33: Channel 3 period low [write-only]
            // 0xFF1D => panic!("NR33 is write-only"),

            // FF1E — NR34: Channel 3 period high & control
            0xFF1E => 0xBF,

            0xFF21 => 0x00,
            0xFF22 => 0x00,
            0xFF23 => 0xBF,

            // FF24 — NR50: Master volume & VIN panning
            0xFF24 => 0x77,

            // FF25 — NR51: Sound panning
            0xFF25 => 0xF3,

            // FF26 — NR52: Audio master control
            0xFF26 => 0xF1,

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

            // _ => panic!("Invalid address"),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, _address: u16, _value: u8) {}
}
