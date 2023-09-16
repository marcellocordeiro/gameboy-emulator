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
        self.source = ((value as u16) << 8) | (self.source & 0x00FF);
    }

    pub fn write_hdma2(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b1111_0000;
        self.source = (self.source & 0xFF00) | ((value & WRITABLE_MASK) as u16);
    }

    pub fn write_hdma3(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b0001_1111;
        self.destination = (((value & WRITABLE_MASK) as u16) << 8) | (self.destination & 0x00FF);
    }

    pub fn write_hdma4(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b1111_0000;
        self.destination = (self.destination & 0xFF00) | ((value & WRITABLE_MASK) as u16);
    }

    pub fn write_hdma5(&mut self, value: u8) {
        ()
    }

    /*pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // HDMA1 (source high)
            0xFF51 => (),

            // HDMA2 (source low)
            0xFF52 => (),

            // HDMA3 (destination high)
            0xFF53 => (),

            // HDMA4 (destination low)
            0xFF54 => (),

            // HDMA5 (length/mode/start)
            0xFF55 => (),

            _ => unreachable!("[vram_dma.rs] Write out of bounds: ({address:#06x}) = {value:#04x}"),
        }
    }*/
}
