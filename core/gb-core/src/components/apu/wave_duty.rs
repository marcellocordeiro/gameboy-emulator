#[derive(Debug, Default)]
pub struct WaveDuty {
    pattern: WaveDutyPattern,
    position: u8,
}

impl WaveDuty {
    pub fn tick(&mut self) {
        self.position = (self.position + 1) & 0b111;
    }

    pub fn wave_data(&self) -> u8 {
        self.pattern.pattern()[self.position as usize]
    }

    pub fn read(&self) -> u8 {
        self.pattern as u8
    }

    pub fn write(&mut self, value: u8) {
        self.pattern = WaveDutyPattern::from_value(value).unwrap();
    }
}

type Pattern = [u8; 8];
const PATTERNS: [Pattern; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [1, 0, 0, 0, 0, 0, 0, 1], // 25%
    [1, 0, 0, 0, 0, 1, 1, 1], // 50%
    [0, 1, 1, 1, 1, 1, 1, 0], // 75%
];

#[derive(Debug, Default, Copy, Clone)]
enum WaveDutyPattern {
    #[default]
    Ratio12_5 = 0b00,
    Ratio25 = 0b01,
    Ratio50 = 0b10,
    Ratio75 = 0b11,
}

impl WaveDutyPattern {
    fn pattern(self) -> Pattern {
        PATTERNS[self as usize]
    }

    fn from_value(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Ratio12_5),
            0b01 => Some(Self::Ratio25),
            0b10 => Some(Self::Ratio50),
            0b11 => Some(Self::Ratio75),
            _ => None,
        }
    }
}
