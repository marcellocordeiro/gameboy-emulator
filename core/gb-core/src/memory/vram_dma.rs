#[derive(Debug, Default)]
pub enum DmaMode {
    #[default]
    Idle,
    General,
    Hblank,
}

#[derive(Debug, Default)]
pub struct VramDma {
    /// `0b1111_1111_2222_XXXX`
    pub source: u16,

    /// `0bXXX3_3333_4444_XXXX`
    pub destination: u16,

    pub hdma1: u8,
    pub hdma2: u8,
    pub hdma3: u8,
    pub hdma4: u8,
    pub hdma5: u8,

    pub mode: DmaMode,
}

impl VramDma {
    /// HDMA1 (source high)
    pub fn read_hdma1(&self) -> u8 {
        0xFF
    }

    /// HDMA2 (source low)
    pub fn read_hdma2(&self) -> u8 {
        0xFF
    }

    /// HDMA3 (destination high)
    pub fn read_hdma3(&self) -> u8 {
        0xFF
    }

    /// HDMA4 (destination low)
    pub fn read_hdma4(&self) -> u8 {
        0xFF
    }

    /// HDMA5 (length/mode/start)
    pub fn read_hdma5(&self) -> u8 {
        0xFF
    }

    pub fn write_hdma1(&mut self, value: u8) {
        self.hdma1 = value;
        self.source = ((value as u16) << 8) | (self.source & 0x00FF);

        self.validate_writes();
    }

    pub fn write_hdma2(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b1111_0000;
        self.hdma2 = value & WRITABLE_MASK;
        self.source = (self.source & 0xFF00) | ((value & WRITABLE_MASK) as u16);

        self.validate_writes();
    }

    pub fn write_hdma3(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b0001_1111;
        self.hdma3 = value & WRITABLE_MASK;
        self.destination = (((value & WRITABLE_MASK) as u16) << 8) | (self.destination & 0x00FF);

        self.validate_writes();
    }

    pub fn write_hdma4(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b1111_0000;
        self.hdma4 = value & WRITABLE_MASK;
        self.destination = (self.destination & 0xFF00) | ((value & WRITABLE_MASK) as u16);

        self.validate_writes();
    }

    pub fn write_hdma5(&mut self, value: u8) {}

    fn validate_writes(&self) {
        assert_eq!(((self.hdma1 as u16) << 8) | self.hdma2 as u16, self.source);
        assert_eq!(
            ((self.hdma3 as u16) << 8) | self.hdma4 as u16,
            self.destination
        );
    }
}