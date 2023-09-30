use crate::cartridge::error::Error as CartridgeError;

pub const CARTRIDGE_TYPE_ADDRESS: usize = 0x0147;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartridgeType {
    NoMbc,
    Mbc1,
    Mbc2,
    Mmm01,
    Mbc3,
    Mbc30,
    Mbc5,
    Mbc6,
    Mbc7,
    PocketCamera,
    BandaiTama5,
    Huc3,
    Huc1,
}

impl TryFrom<(u8, usize)> for CartridgeType {
    type Error = CartridgeError;

    fn try_from((code, ram_size): (u8, usize)) -> Result<Self, Self::Error> {
        Ok(match code {
            // $00 ROM ONLY
            // $08 ROM+RAM
            // $09 ROM+RAM+BATTERY
            0x00 => Self::NoMbc,
            0x08..=0x09 => {
                // NoMbc,
                todo!("NoMBC RAM/RAM+BATTERY");
            }

            // $01 MBC1
            // $02 MBC1+RAM
            // $03 MBC1+RAM+BATTERY
            0x01..=0x03 => Self::Mbc1,

            // $05 MBC2
            // $06 MBC2+BATTERY
            0x05..=0x06 => Self::Mbc2,

            // $0B MMM01
            // $0C MMM01+RAM
            // $0D MMM01+RAM+BATTERY
            0x0B..=0x0D => Self::Mmm01,

            // $0F MBC3+TIMER+BATTERY
            // $10 MBC3+TIMER+RAM+BATTERY
            // $11 MBC3
            // $12 MBC3+RAM
            // $13 MBC3+RAM+BATTERY
            0x0F..=0x13 if ram_size == 8 => Self::Mbc30, // 8 banks (64 KiB)
            0x0F..=0x13 => Self::Mbc3,

            // $19 MBC5
            // $1A MBC5+RAM
            // $1B MBC5+RAM+BATTERY
            // $1C MBC5+RUMBLE
            // $1D MBC5+RUMBLE+RAM
            // $1E MBC5+RUMBLE+RAM+BATTERY
            0x19..=0x1E => Self::Mbc5,

            // $20 MBC6
            0x20 => Self::Mbc6,

            // $22 MBC7+SENSOR+RUMBLE+RAM+BATTERY
            0x22 => Self::Mbc7,

            // $FC POCKET CAMERA
            0xFC => Self::PocketCamera,

            // $FD BANDAI TAMA5
            0xFD => Self::BandaiTama5,

            // $FE HuC3
            0xFE => Self::Huc3,

            // $FF HuC1+RAM+BATTERY
            0xFF => Self::Huc1,

            code => return Err(Self::Error::InvalidMbcCode { code }),
        })
    }
}

impl std::fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            Self::NoMbc => "No MBC",
            Self::Mbc1 => "MBC1",
            Self::Mbc2 => "MBC2",
            Self::Mmm01 => "MMM01",
            Self::Mbc3 => "MBC3",
            Self::Mbc30 => "MBC30",
            Self::Mbc5 => "MBC5",
            Self::Mbc6 => "MBC6",
            Self::Mbc7 => "MBC7",
            Self::PocketCamera => "POCKET CAMERA",
            Self::BandaiTama5 => "BANDAI TAMA5",
            Self::Huc3 => "HuC3",
            Self::Huc1 => "HuC1",
        };

        write!(f, "{name}")
    }
}
