use bitflags::bitflags;

use super::Graphics;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatusMode {
    Hblank = 0b00,
    Vblank = 0b01,
    OamScan = 0b10,
    Drawing = 0b11,
}

/*
Bit	Name								Usage notes
6	LYC=LY STAT Interrupt source		0=Off, 1=On				(Read/Write)
5	Mode 2 OAM STAT Interrupt source	0=Off, 1=On 			(Read/Write)
4	Mode 1 VBlank STAT Interrupt source	0=Off, 1=On 			(Read/Write)
3	Mode 0 HBlank STAT Interrupt source	0=Off, 1=On 			(Read/Write)
2	LYC=LY Flag							0=Different, 1=Equal	(Read Only)
1-0	Mode Flag							Mode 0-3, see below		(Read Only)
    0: HBlank
    1: VBlank
    2: Searching OAM
    3: Transferring Data to LCD Controller
*/
bitflags!(
    #[derive(Default, Clone, Copy)]
    pub struct LcdStatus: u8 {
        const COMPARE_IRQ = 1 << 6;
        const OAM_IRQ     = 1 << 5;
        const VBLANK_IRQ  = 1 << 4;
        const HBLANK_IRQ  = 1 << 3;
        const LY_COMPARE  = 1 << 2;

        // Bit 7 is unused (should return 1).
        // Bits 1-0 are handled elsewhere. (StatusMode)
    }
);

// Convenience methods.
impl LcdStatus {
    pub fn get_compare_irq(self) -> bool {
        self.contains(Self::COMPARE_IRQ)
    }

    pub fn get_oam_irq(self) -> bool {
        self.contains(Self::OAM_IRQ)
    }

    pub fn get_vblank_irq(self) -> bool {
        self.contains(Self::VBLANK_IRQ)
    }

    pub fn get_hblank_irq(self) -> bool {
        self.contains(Self::HBLANK_IRQ)
    }
}

impl Graphics {
    pub fn read_stat(&self) -> u8 {
        if !self.lcdc.get_lcd_enable() {
            return 1 << 7;
        }

        (1 << 7) | self.stat.bits() | self.mode as u8
    }

    pub fn write_stat(&mut self, value: u8) {
        // Bits 0-2 are read-only, so we ignore these bits from the new value,
        // take these bits from the current STAT, and `OR` them together.
        let current_stat_bits = self.stat.bits();
        let result = (value & !0b111) | (current_stat_bits & 0b111);

        self.stat = LcdStatus::from_bits_truncate(result);
    }
}