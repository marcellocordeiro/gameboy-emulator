use bitflags::bitflags;

use super::Graphics;

/*
FF40 — LCDC: LCD control

LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.

Bit	Name                            Usage notes
7	LCD and PPU enable				0=Off, 1=On
6	Window tile map area			0=9800-9BFF, 1=9C00-9FFF
5	Window enable					0=Off, 1=On
4	BG and Window tile data area	0=8800-97FF, 1=8000-8FFF
3	BG tile map area				0=9800-9BFF, 1=9C00-9FFF
2	OBJ size						0=8x8, 1=8x16
1	OBJ enable						0=Off, 1=On
0	BG and Window enable/priority	0=Off, 1=On
*/
bitflags!(
    #[derive(Default, Clone, Copy)]
    pub(super) struct LcdControl: u8 {
        const LCD_ENABLE    = 1 << 7;
        const WIN_MAP       = 1 << 6;
        const WIN_ENABLE    = 1 << 5;
        const BG_WIN_ADDR   = 1 << 4;
        const BG_MAP        = 1 << 3;
        const OBJ_SIZE      = 1 << 2;
        const OBJ_ENABLE    = 1 << 1;
        const BG_ENABLE     = 1 << 0;
    }
);

// Convenience methods.
impl LcdControl {
    pub fn get_lcd_enable(self) -> bool {
        self.contains(Self::LCD_ENABLE)
    }

    pub fn get_win_map(self) -> bool {
        self.contains(Self::WIN_MAP)
    }

    pub fn get_win_enable(self) -> bool {
        self.contains(Self::WIN_ENABLE)
    }

    pub fn get_bg_win_addr(self) -> bool {
        self.contains(Self::BG_WIN_ADDR)
    }

    pub fn get_bg_map(self) -> bool {
        self.contains(Self::BG_MAP)
    }

    pub fn get_obj_size(self) -> bool {
        self.contains(Self::OBJ_SIZE)
    }

    pub fn get_obj_enable(self) -> bool {
        self.contains(Self::OBJ_ENABLE)
    }

    pub fn get_bg_enable(self) -> bool {
        self.contains(Self::BG_ENABLE)
    }
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
    pub(super) struct LcdStatus: u8 {
        const COMPARE_IRQ = 1 << 6;
        const OAM_IRQ     = 1 << 5;
        const VBLANK_IRQ  = 1 << 4;
        const HBLANK_IRQ  = 1 << 3;
        const LY_COMPARE  = 1 << 2;

        // Bit 7 is unused (should return 1).
        // Bits 1-0 are handled elsewhere.
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum StatusMode {
    Hblank = 0,
    Vblank = 1,
    OamScan = 2,
    Drawing = 3,
}

impl Graphics {
    pub(super) fn read_lcdc(&self) -> u8 {
        self.lcdc.bits()
    }

    pub(super) fn read_stat(&self) -> u8 {
        if !self.lcdc.get_lcd_enable() {
            return 1 << 7;
        }

        (1 << 7) | self.stat.bits() | self.mode as u8
    }

    pub(super) fn write_lcdc(&mut self, value: u8) {
        let new_lcdc = LcdControl::from_bits_truncate(value);

        let lcd_enable = self.lcdc.get_lcd_enable();
        let new_lcd_enable = new_lcdc.get_lcd_enable();

        if lcd_enable && !new_lcd_enable {
            self.mode = StatusMode::OamScan;
            self.cycles = 0;
            self.ly = 0;
            self.framebuffer.fill(0);
        }

        self.lcdc = new_lcdc;
    }

    pub(super) fn write_stat(&mut self, value: u8) {
        // Bits 0-2 are read-only, so we ignore these bits from the new value,
        // take these bits from the current STAT, and `OR` them together.
        let current_stat_bits = self.stat.bits();
        let result = (value & !0b111) | (current_stat_bits & 0b111);

        self.stat = LcdStatus::from_bits_truncate(result);
    }

    pub(super) fn read_dma(&self) -> u8 {
        self.oam_dma.dma
    }

    pub(super) fn write_dma(&mut self, value: u8) {
        self.oam_dma.start(value);
    }
}
