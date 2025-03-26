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
    /// Number of steps of 0x10 bytes.
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
            DmaMode::Idle | DmaMode::General => 0xFF,

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
        if (value & 0b1000_0000) == 0 {
            if let DmaMode::Hblank { .. } = self.mode {
                // TODO: update HDMA5
                self.mode = DmaMode::Idle;
                return;
            }
        }

        self.hdma5 = value;

        let steps = (self.hdma5 & 0b0111_1111) + 1;

        self.steps = steps;

        if (value & 0b1000_0000) == 0 {
            self.mode = DmaMode::General;
        } else {
            self.mode = DmaMode::Hblank {
                active: false,
                remaining_steps: steps,
            };
        }
    }

    #[allow(clippy::manual_range_contains)]
    /// `0b1111_1111_2222_XXXX`
    pub fn source(&self) -> u16 {
        let source = ((self.hdma1 as u16) << 8) | (self.hdma2 as u16);

        assert!(
            (source <= 0x7FF0) || (0xA000 <= source && source <= 0xDFF0),
            "with source = {source:#06X}"
        );

        source
    }

    /// `0bXXX3_3333_4444_XXXX`
    pub fn destination(&self) -> u16 {
        0x8000 | ((self.hdma3 as u16) << 8) | (self.hdma4 as u16)
    }

    pub fn perform_gdma(&mut self) -> Option<impl Iterator<Item = u16> + Clone + use<>> {
        if self.mode != DmaMode::General {
            return None;
        }

        self.mode = DmaMode::Idle;

        let length = (self.steps as u16) * 0x10;

        Some(0..length)
    }

    pub fn perform_hdma(&mut self) -> Option<impl Iterator<Item = u16> + Clone + use<>> {
        let step = match self.mode {
            DmaMode::Hblank { active: false, .. } => {
                return None;
            }

            DmaMode::Hblank {
                remaining_steps: 1, ..
            } => {
                self.mode = DmaMode::Idle;

                self.steps - 1
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

        let length = 0x10;
        let base_offset = (step as u16) * 0x10;

        Some(base_offset..(base_offset + length))
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

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test cancellation and HDMA5 reads.

    #[test]
    fn test_gdma() {
        let mut vram_dma = VramDma::default();

        vram_dma.write_hdma1(0xDF); // 0xFF is invalid, but probably allowed.
        vram_dma.write_hdma2(0xFF);
        vram_dma.write_hdma3(0xFF);
        vram_dma.write_hdma4(0xFF);
        vram_dma.write_hdma5(0b0111_1111);

        let source = ((vram_dma.hdma1 as u16) << 8) | (vram_dma.hdma2 as u16);
        assert_eq!(source, 0xDFFF & 0xFFF0);
        assert_eq!(vram_dma.source(), source);

        let destination = 0x8000 | ((vram_dma.hdma3 as u16) << 8) | (vram_dma.hdma4 as u16);
        assert_eq!(destination, 0x8000 | 0x1FF0);
        assert_eq!(vram_dma.destination(), destination);

        let steps = (vram_dma.hdma5 & 0b0111_1111) + 1;
        assert_eq!(vram_dma.steps, steps);
        assert_eq!(vram_dma.mode, DmaMode::General);

        let offsets = vram_dma.perform_gdma().unwrap();
        assert_eq!(
            offsets.clone().cmp(0x0000..((0x7F + 1) * 0x10)),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            offsets.cmp(0x0000..((vram_dma.steps as u16) * 0x10)),
            std::cmp::Ordering::Equal
        );

        assert_eq!(vram_dma.mode, DmaMode::Idle);
    }

    #[test]
    fn test_hdma_setup() {
        let mut vram_dma = VramDma::default();

        vram_dma.write_hdma1(0xDF); // 0xFF is invalid, but probably allowed.
        vram_dma.write_hdma2(0xFF);
        vram_dma.write_hdma3(0xFF);
        vram_dma.write_hdma4(0xFF);
        vram_dma.write_hdma5(0b1111_1111);

        let source = ((vram_dma.hdma1 as u16) << 8) | (vram_dma.hdma2 as u16);
        assert_eq!(source, 0xDFFF & 0xFFF0);
        assert_eq!(vram_dma.source(), source);

        let destination = 0x8000 | ((vram_dma.hdma3 as u16) << 8) | (vram_dma.hdma4 as u16);
        assert_eq!(destination, 0x8000 | 0x1FF0);
        assert_eq!(vram_dma.destination(), destination);

        let steps = (vram_dma.hdma5 & 0b0111_1111) + 1;
        assert_eq!(vram_dma.steps, steps);
        assert_eq!(
            vram_dma.mode,
            DmaMode::Hblank {
                active: false,
                remaining_steps: steps
            }
        );

        vram_dma.resume_hdma();

        let offsets = vram_dma.perform_hdma().unwrap();
        assert_eq!(offsets.cmp(0x0000..0x10), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_hdma() {
        let mut vram_dma = VramDma::default();

        vram_dma.write_hdma1(0xDF); // 0xFF is invalid, but probably allowed.
        vram_dma.write_hdma2(0xFF);
        vram_dma.write_hdma3(0xFF);
        vram_dma.write_hdma4(0xFF);
        vram_dma.write_hdma5(0b1111_1111);

        let steps = (vram_dma.hdma5 & 0b0111_1111) + 1;
        assert_eq!(vram_dma.steps, steps);
        assert_eq!(
            vram_dma.mode,
            DmaMode::Hblank {
                active: false,
                remaining_steps: steps
            }
        );

        vram_dma.resume_hdma();

        for step in 0..steps {
            assert_eq!(
                vram_dma.mode,
                DmaMode::Hblank {
                    active: true,
                    remaining_steps: steps - step
                }
            );

            let offsets = vram_dma.perform_hdma().unwrap();
            assert_eq!(offsets.count(), 0x10);

            vram_dma.resume_hdma();
        }

        assert_eq!(vram_dma.mode, DmaMode::Idle);
    }
}
