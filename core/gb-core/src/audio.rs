// TODO: everything

use crate::utils::macros::device_is_cgb;

#[derive(Default)]
pub struct Audio {}

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
            0xFF10 => 0x80,
            0xFF11 => 0xBF,
            0xFF12 => 0xF3,
            0xFF14 => 0xBF,
            0xFF16 => 0x3F,
            0xFF17 => 0x00,
            0xFF19 => 0xBF,
            0xFF1A => 0x7F,
            0xFF1C => 0x9F,
            0xFF1E => 0xBF,
            0xFF21 => 0x00,
            0xFF22 => 0x00,
            0xFF23 => 0xBF,
            0xFF24 => 0x77,
            0xFF25 => 0xF3,
            0xFF26 => 0xF1,

            0xFF76 => {
                if !device_is_cgb!() {
                    return 0xFF;
                }

                0x00
            }

            0xFF77 => {
                if !device_is_cgb!() {
                    return 0xFF;
                }

                0x00
            }

            _ => 0xFF,
        }
    }

    pub fn write(&mut self, _address: u16, _value: u8) {}
}
