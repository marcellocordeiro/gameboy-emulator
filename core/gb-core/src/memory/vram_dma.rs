#[derive(Debug, Default, PartialEq, Eq)]
pub enum DmaMode {
    #[default]
    Idle,
    General,
    Hblank {
        active: bool,
        remaining_steps: u8,
    },
}

#[derive(Debug, Default)]
pub struct VramDma {
    /// `0b1111_1111_2222_XXXX`
    pub source: u16,

    /// `0bXXX3_3333_4444_XXXX`
    pub destination: u16,

    pub steps: u8,

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
        match self.mode {
            DmaMode::Idle => 0xFF,
            DmaMode::General => 0xFF,

            DmaMode::Hblank {
                remaining_steps, ..
            } => remaining_steps,
        }
    }

    pub fn write_hdma1(&mut self, value: u8) {
        self.hdma1 = value;
    }

    pub fn write_hdma2(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b1111_0000;
        self.hdma2 = value & WRITABLE_MASK;
    }

    pub fn write_hdma3(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b0001_1111;
        self.hdma3 = value & WRITABLE_MASK;
    }

    pub fn write_hdma4(&mut self, value: u8) {
        const WRITABLE_MASK: u8 = 0b1111_0000;
        self.hdma4 = value & WRITABLE_MASK;
    }

    pub fn write_hdma5(&mut self, value: u8) {
        self.hdma5 = value;

        let source = ((self.hdma1 as u16) << 8) | (self.hdma2 as u16);
        let destination = 0x8000 | ((self.hdma3 as u16) << 8) | (self.hdma4 as u16);
        let steps = (self.hdma5 & 0b0111_1111) + 1;

        self.source = source;
        self.destination = destination;
        self.steps = steps;

        if value & 0b1000_0000 == 0 {
            self.mode = DmaMode::General;
        } else {
            self.mode = DmaMode::Hblank {
                active: true,
                remaining_steps: steps,
            };
        }
    }

    /*
    pub fn perform_gdma(&mut self) -> Option<impl Iterator<Item = u16>> {
        if self.mode == DmaMode::General {
            let len = ((self.hdma5 & 0b0111_1111) as u16 + 1) * 0x10;
            let iter = 0..len;

            self.mode = DmaMode::Idle;

            return Some(iter);
        }

        None
    }
    */

    pub fn perform_gdma(&mut self) -> Option<u16> {
        if self.mode != DmaMode::General {
            return None;
        }

        self.mode = DmaMode::Idle;

        Some((self.steps as u16) * 0x10)
    }

    pub fn perform_hdma(&mut self) -> Option<u16> {
        let step = match self.mode {
            DmaMode::Hblank { active: false, .. } => {
                return None;
            }

            DmaMode::Hblank {
                remaining_steps: 1, ..
            } => {
                self.mode = DmaMode::Idle;

                return None;
            }

            DmaMode::Hblank {
                ref mut remaining_steps,
                ref mut active,
            } => {
                let current = *remaining_steps;

                *active = false;
                *remaining_steps -= 1;

                self.steps - current
            }

            _ => return None,
        };

        Some((step as u16) * 0x10)
    }

    pub fn resume_hdma(&mut self) {
        if let DmaMode::Hblank {
            active: false,
            remaining_steps,
        } = self.mode
        {
            self.mode = DmaMode::Hblank {
                active: true,
                remaining_steps,
            };
        }
    }
}
