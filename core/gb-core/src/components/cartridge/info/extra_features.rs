use super::{header::Header, mbc_type::MBC_TYPE_ADDRESS};
use crate::components::cartridge::error::CartridgeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraFeature {
    Ram,
    Battery,
    Timer,
    Rumble,
    Sensor,
}

impl ExtraFeature {
    pub fn from_header(header: &Header) -> Result<Box<[Self]>, CartridgeError> {
        let code = header[MBC_TYPE_ADDRESS];

        Self::from_code(code).map(Into::into)
    }

    fn from_code(code: u8) -> Result<&'static [Self], CartridgeError> {
        Ok(match code {
            // $00 ROM ONLY
            // $08 ROM+RAM
            // $09 ROM+RAM+BATTERY
            0x00 => &[],
            0x08 => &[Self::Ram],
            0x09 => &[Self::Ram, Self::Battery],

            // $01 MBC1
            // $02 MBC1+RAM
            // $03 MBC1+RAM+BATTERY
            0x01 => &[],
            0x02 => &[Self::Ram],
            0x03 => &[Self::Ram, Self::Battery],

            // $05 MBC2
            // $06 MBC2+BATTERY (RAM is implied)
            0x05 => &[],
            0x06 => &[Self::Battery],

            // $0B MMM01
            // $0C MMM01+RAM
            // $0D MMM01+RAM+BATTERY
            0x0B => &[],
            0x0C => &[Self::Ram],
            0x0D => &[Self::Ram, Self::Battery],

            // $0F MBC3+TIMER+BATTERY
            // $10 MBC3+TIMER+RAM+BATTERY
            // $11 MBC3
            // $12 MBC3+RAM
            // $13 MBC3+RAM+BATTERY
            0x0F => &[Self::Timer, Self::Battery],
            0x10 => &[Self::Timer, Self::Ram, Self::Battery],
            0x11 => &[],
            0x12 => &[Self::Ram],
            0x13 => &[Self::Ram, Self::Battery],

            // $19 MBC5
            // $1A MBC5+RAM
            // $1B MBC5+RAM+BATTERY
            // $1C MBC5+RUMBLE
            // $1D MBC5+RUMBLE+RAM
            // $1E MBC5+RUMBLE+RAM+BATTERY
            0x19 => &[],
            0x1A => &[Self::Ram],
            0x1B => &[Self::Ram, Self::Battery],
            0x1C => &[Self::Rumble],
            0x1D => &[Self::Rumble, Self::Ram],
            0x1E => &[Self::Rumble, Self::Ram, Self::Battery],

            // $20 MBC6
            0x20 => &[],

            // $22 MBC7+SENSOR+RUMBLE+RAM+BATTERY
            0x22 => &[Self::Sensor, Self::Rumble, Self::Ram, Self::Battery],

            // $FC POCKET CAMERA
            0xFC => &[],

            // $FD BANDAI TAMA5
            0xFD => &[],

            // $FE HuC3
            0xF3 => &[],

            // $FF HuC1+RAM+BATTERY
            0xFF => &[Self::Ram, Self::Battery],

            _ => Err(CartridgeError::UnsupportedExtraFeatures { code })?,
        })
    }
}

impl std::fmt::Display for ExtraFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::Ram => "RAM",
            Self::Battery => "BATTERY",
            Self::Timer => "TIMER",
            Self::Rumble => "RUMBLE",
            Self::Sensor => "SENSOR",
        };

        write!(f, "{str}")
    }
}
