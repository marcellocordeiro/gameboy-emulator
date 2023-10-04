use crate::cartridge::error::Error as CartridgeError;

use super::cartridge_type::CARTRIDGE_TYPE_ADDRESS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraFeature {
    Ram,
    Battery,
    Timer,
    Rumble,
    Sensor,
}

impl ExtraFeature {
    pub fn features_with_rom(rom: &[u8]) -> Result<Box<[Self]>, CartridgeError> {
        let cartridge_type_code = *rom
            .get(CARTRIDGE_TYPE_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        Ok(Self::features_from_code(cartridge_type_code))
    }

    pub fn features_from_code(code: u8) -> Box<[Self]> {
        match code {
            // $08 ROM+RAM
            // $09 ROM+RAM+BATTERY
            0x08 => vec![Self::Ram],
            0x09 => vec![Self::Ram, Self::Battery],

            // $02 MBC1+RAM
            // $03 MBC1+RAM+BATTERY
            0x02 => vec![Self::Ram],
            0x03 => vec![Self::Ram, Self::Battery],

            // $06 MBC2+BATTERY (RAM is implied)
            0x06 => vec![Self::Battery],

            // $0C MMM01+RAM
            // $0D MMM01+RAM+BATTERY
            0x0C => vec![Self::Ram],
            0x0D => vec![Self::Ram, Self::Battery],

            // $0F MBC3+TIMER+BATTERY
            // $10 MBC3+TIMER+RAM+BATTERY
            // $12 MBC3+RAM
            // $13 MBC3+RAM+BATTERY
            0x0F => vec![Self::Timer, Self::Battery],
            0x10 => vec![Self::Timer, Self::Ram, Self::Battery],
            0x12 => vec![Self::Ram],
            0x13 => vec![Self::Ram, Self::Battery],

            // $1A MBC5+RAM
            // $1B MBC5+RAM+BATTERY
            // $1C MBC5+RUMBLE
            // $1D MBC5+RUMBLE+RAM
            // $1E MBC5+RUMBLE+RAM+BATTERY
            0x1A => vec![Self::Ram],
            0x1B => vec![Self::Ram, Self::Battery],
            0x1C => vec![Self::Rumble],
            0x1D => vec![Self::Rumble, Self::Ram],
            0x1E => vec![Self::Rumble, Self::Ram, Self::Battery],

            // $22 MBC7+SENSOR+RUMBLE+RAM+BATTERY
            0x22 => vec![Self::Sensor, Self::Rumble, Self::Ram, Self::Battery],

            // $FF HuC1+RAM+BATTERY
            0xFF => vec![Self::Ram, Self::Battery],

            _ => vec![],
        }
        .into_boxed_slice()
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
