#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraFeature {
    Ram,
    Battery,
    Timer,
    Rumble,
    Sensor,
}

impl ExtraFeature {
    pub fn get_features(code: u8) -> Vec<ExtraFeature> {
        use ExtraFeature::*;

        match code {
            // $08 ROM+RAM
            // $09 ROM+RAM+BATTERY
            0x08 => [Ram].to_vec(),
            0x09 => [Ram, Battery].to_vec(),

            // $02 MBC1+RAM
            // $03 MBC1+RAM+BATTERY
            0x02 => [Ram].to_vec(),
            0x03 => [Ram, Battery].to_vec(),

            // $06 MBC2+BATTERY (RAM is implied)
            0x06 => [Battery].to_vec(),

            // $0C MMM01+RAM
            // $0D MMM01+RAM+BATTERY
            0x0C => [Ram].to_vec(),
            0x0D => [Ram, Battery].to_vec(),

            // $0F MBC3+TIMER+BATTERY
            // $10 MBC3+TIMER+RAM+BATTERY
            // $12 MBC3+RAM
            // $13 MBC3+RAM+BATTERY
            0x0F => [Timer, Battery].to_vec(),
            0x10 => [Timer, Ram, Battery].to_vec(),
            0x12 => [Ram].to_vec(),
            0x13 => [Ram, Battery].to_vec(),

            // $1A MBC5+RAM
            // $1B MBC5+RAM+BATTERY
            // $1C MBC5+RUMBLE
            // $1D MBC5+RUMBLE+RAM
            // $1E MBC5+RUMBLE+RAM+BATTERY
            0x1A => [Ram].to_vec(),
            0x1B => [Ram, Battery].to_vec(),
            0x1C => [Rumble].to_vec(),
            0x1D => [Rumble, Ram].to_vec(),
            0x1E => [Rumble, Ram, Battery].to_vec(),

            // $22 MBC7+SENSOR+RUMBLE+RAM+BATTERY
            0x22 => [Sensor, Rumble, Ram, Battery].to_vec(),

            // $FF HuC1+RAM+BATTERY
            0xFF => [Ram, Battery].to_vec(),

            _ => [].to_vec(),
        }
    }
}

impl std::fmt::Display for ExtraFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ExtraFeature::*;

        let str = match self {
            Ram => "RAM",
            Battery => "BATTERY",
            Timer => "TIMER",
            Rumble => "RUMBLE",
            Sensor => "SENSOR",
        };

        write!(f, "{str}")
    }
}
